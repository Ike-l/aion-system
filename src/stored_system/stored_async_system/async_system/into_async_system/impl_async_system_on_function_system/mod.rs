macro_rules! impl_async_system_on_function_system {
    (
        $($params:ident),*
    ) => {
        #[allow(unused_variables)]
        #[allow(non_snake_case)]
        impl<F, Fut, $($params: Injection),*> AsyncSystem for FunctionSystemBase<($($params,)*), F>
            where
                Fut: Future<Output = Option<SystemResult>> + Send + 'static,
                F: Send + Sync,
                for<'b> F:
                    FnMut($($params),*) -> Fut +
                    FnMut($(<$params as Injection>::Item<'b>),*) -> Fut
        {
            fn execute<'a>(
                &'a mut self, 
                program_registry: Arc<ProgramRegistry>,
                program_id: ProgramId,
                program_password: Option<ValuePassword>,
                user_details: Option<(UserId, UserPassword)>
            ) -> Pin<Box<dyn Future<Output = Result<Option<SystemResult>, SystemError>> + 'a + Send>> {
                Box::pin(async move {
                    let user_details = user_details.as_ref().map(|(user_id, user_password)| (user_id, user_password));
                    
                    $(
                        let $params = {
                            match program_registry.resolve::<$params>(vec![PromptedProgramAccess {
                                program_id: &program_id,
                                program_password: program_password.as_ref(),
                                user_details,
                                resource_details: None
                            }]) {
                                Ok(item) => item,
                                Err(resolve_resource_error) => {
                                    return Err(SystemError::ParameterFailure)
                                }
                            }
                        };
                    )*

                    Ok((self.f)($($params),*).await)
                })
            }
        }
    };
}

macro_rules! impl_all_async_system_on_function_system {
    () => {
        use std::{sync::Arc, pin::Pin};
        pub use aion_program::prelude::{Injection, ProgramRegistry, PromptedProgramAccess, ProgramId, ValuePassword, UserId, UserPassword};
        pub use crate::prelude::{FunctionSystemBase, AsyncSystem, SystemResult, SystemError};

        impl_async_system_on_function_system!();
    };

    ($first:ident $(, $rest:ident)*) => {
        impl_async_system_on_function_system!($first $(, $rest)*);
        impl_all_async_system_on_function_system!($($rest),*);
    };
}

impl_all_async_system_on_function_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9);