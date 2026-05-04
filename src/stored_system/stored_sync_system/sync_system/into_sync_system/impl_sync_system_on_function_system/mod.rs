macro_rules! impl_sync_system_on_function_system {
    (
        $($params:ident),*
    ) => {
        #[allow(unused_variables)]
        #[allow(non_snake_case)]
        impl<F, $($params: Injection),*> SyncSystem for FunctionSystemBase<($($params,)*), F>
            where
                F: Send + Sync,
                for<'a, 'b> &'a mut F:
                    FnMut($($params),*) -> Option<SystemResult> +
                    FnMut($(<$params as Injection>::Item<'b>),*) -> Option<SystemResult>
        {
            fn execute(
                &mut self, 
                program_registry: &Arc<ProgramRegistry>,
                program_id: &ProgramId,
                program_password: Option<&ValuePassword>,
                user_details: Option<(&UserId, &UserPassword)>
            ) -> Result<Option<SystemResult>, SystemError> {
                fn call_inner<$($params),*>(
                    mut f: impl FnMut($($params),*) -> Option<SystemResult>,
                    $($params: $params),*
                ) -> Option<SystemResult> {
                    f($($params),*)
                }

                $(
                    let $params = {
                        match program_registry.resolve::<$params>(vec![PromptedProgramAccess {
                            program_id,
                            program_password,
                            user_details,
                            resource_id: None,
                            resource_access: None,
                            resource_password: None,
                        }]) {
                            Ok(Ok(item)) => item,
                            _ => {
                                return Err(SystemError::ParameterFailure)
                            }
                        }
                    };
                )*

                Ok(call_inner(&mut self.f, $($params),*))
            }
        }
    };
}

macro_rules! impl_all_sync_system_on_function_system {
    () => {
        use std::sync::Arc;
        pub use aion_program::prelude::{Injection, ProgramRegistry, PromptedProgramAccess, ProgramId, ValuePassword, UserId, UserPassword};
        pub use crate::prelude::{FunctionSystemBase, SyncSystem, SystemResult, SystemError};

        impl_sync_system_on_function_system!();
    };

    ($first:ident $(, $rest:ident)*) => {
        impl_sync_system_on_function_system!($first $(, $rest)*);
        impl_all_sync_system_on_function_system!($($rest),*);
    };
}

impl_all_sync_system_on_function_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9);