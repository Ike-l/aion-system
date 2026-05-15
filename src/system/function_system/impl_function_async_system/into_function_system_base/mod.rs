macro_rules! impl_function_system_into_async_system {
    (
        $($params:ident),*
    ) => {
        impl<F, Fut, $($params: Injection),*> IntoAsyncSystem<($($params,)*)> for F
            where 
                Fut: Future<Output = Option<SystemResult>> + Send + 'static,
                F: Send + Sync,
                for<'b> F:
                    FnMut($($params),*) -> Fut +
                    FnMut($(<$params as Injection>::Item<'b>),*) -> Fut
        {
            type System = FunctionSystemBase<($($params,)*), Self>;

            fn into_system(self) -> Self::System {
                FunctionSystemBase {
                    f: self,
                    marker: Default::default(),
                }
            }
        }
    };
}

macro_rules! impl_all_function_system_into_async_system {
    () => {
        use aion_program::prelude::Injection;
        use crate::prelude::{FunctionSystemBase, IntoAsyncSystem, SystemResult};

        impl_function_system_into_async_system!();
    };

    ($first:ident $(, $rest:ident)*) => {
        impl_function_system_into_async_system!($first $(, $rest)*);
        impl_all_function_system_into_async_system!($($rest),*);
    };
}


impl_all_function_system_into_async_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9);