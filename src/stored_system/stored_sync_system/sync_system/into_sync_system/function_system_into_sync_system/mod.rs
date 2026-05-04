macro_rules! impl_function_system_into_sync_system {
    (
        $($params:ident),*
    ) => {
        impl<F, $($params: Injection),*> IntoSyncSystem<($($params,)*)> for F
            where 
                F: Send + Sync,
                for<'a, 'b> &'a mut F:
                    FnMut($($params),*) -> Option<SystemResult> +
                    FnMut($(<$params as Injection>::Item<'b>),*) -> Option<SystemResult>
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

macro_rules! impl_all_function_system_into_sync_system {
    () => {
        use aion_program::prelude::Injection;
        use crate::prelude::{FunctionSystemBase, IntoSyncSystem, SystemResult};

        impl_function_system_into_sync_system!();
    };

    ($first:ident $(, $rest:ident)*) => {
        impl_function_system_into_sync_system!($first $(, $rest)*);
        impl_all_function_system_into_sync_system!($($rest),*);
    };
}


impl_all_function_system_into_sync_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9);