macro_rules! impl_sync_system_on_function_system {
    (
        $($params:ident),*
    ) => {
        #[allow(unused_variables)]
        #[allow(unused_assignments)]
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
                auto_access_builder: &AccessBuilder,
                #[allow(unused_mut)]
                mut manual_access_builders: Vec<&AccessBuilder>
            ) -> Result<Option<SystemResult>, SystemError> {
                fn call_inner<$($params),*>(
                    mut f: impl FnMut($($params),*) -> Option<SystemResult>,
                    $($params: $params),*
                ) -> Option<SystemResult> {
                    f($($params),*)
                }

                #[allow(unused_mut)]
                let mut claims: HashMap<usize, Vec<&AccessBuilder>> = HashMap::new();
                #[allow(unused_mut)]
                let mut absolute_index = 0;
                $(
                    let mut indexes = $params::claim_indexes(manual_access_builders.clone());
                    indexes.sort();
                    let sorted_indexes = indexes;

                    let claimed_access_builders = sorted_indexes.iter().rev().filter_map(|index| {
                        if manual_access_builders.len() > *index {
                            Some(manual_access_builders.remove(*index))
                        } else {
                            None
                        }
                    }).rev().collect();

                    claims.insert(absolute_index, claimed_access_builders);

                    absolute_index += 1;
                )*

                #[allow(unused_mut)]
                let mut absolute_index = 0;
                $(
                    let $params = {
                        let claimed_access_builders = claims.remove(&absolute_index).unwrap();
                        let mut access_builders = vec![auto_access_builder.clone()];
                        access_builders.extend(claimed_access_builders.clone().into_iter().cloned());

                        match program_registry.resolve::<$params>(access_builders) {
                            Ok(Ok(item)) => item,
                            _ => {
                                return Err(SystemError::ParameterFailure)
                            }
                        }
                    };

                    absolute_index += 1;
                )*

                Ok(call_inner(&mut self.f, $($params),*))
            }
        }
    };
}

macro_rules! impl_all_sync_system_on_function_system {
    () => {
        use std::{sync::Arc, collections::HashMap};
        pub use aion_program::prelude::{Injection, ProgramRegistry, ProgramId, ValuePassword, UserId, UserPassword, ResourceId, ResourceAccess, AccessBuilder};
        pub use crate::prelude::{FunctionSystemBase, SyncSystem, SystemResult, SystemError};

        impl_sync_system_on_function_system!();
    };

    ($first:ident $(, $rest:ident)*) => {
        impl_sync_system_on_function_system!($first $(, $rest)*);
        impl_all_sync_system_on_function_system!($($rest),*);
    };
}

impl_all_sync_system_on_function_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9);