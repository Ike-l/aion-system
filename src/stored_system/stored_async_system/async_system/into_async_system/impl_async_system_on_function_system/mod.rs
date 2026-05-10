macro_rules! impl_async_system_on_function_system {
    (
        $($params:ident),*
    ) => {
        #[allow(unused_variables)]
        #[allow(non_snake_case)]
        #[allow(unused_assignments)]
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
                auto_access_builder: AccessBuilder,
                #[allow(unused_mut)]
                mut manual_access_builders: Vec<AccessBuilder>,
            ) -> Pin<Box<dyn Future<Output = Result<Option<SystemResult>, SystemError>> + 'a + Send>> {
                Box::pin(async move {
                    #[allow(unused_mut)]
                    let mut claims: HashMap<usize, Vec<AccessBuilder>> = HashMap::new();
                    #[allow(unused_mut)]
                    let mut absolute_index = 0;
                    $(
                        let mut indexes = $params::claim_manual_access_builders(manual_access_builders.iter().collect());
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
                            let mut access_builders: Vec<AccessBuilder> = vec![auto_access_builder.clone()];
                            access_builders.extend(claimed_access_builders);

                            match program_registry.resolve::<$params>(access_builders) {
                                Ok(Ok(item)) => item,
                                _ => {
                                    return Err(SystemError::ParameterFailure)
                                }
                            }
                        };

                        absolute_index += 1;
                    )*

                    Ok((self.f)($($params),*).await)
                })
            }
        
            fn check_accesses(
                &self,
                program_registry: &Arc<ProgramRegistry>,
                auto_access_builder: &AccessBuilder,
                #[allow(unused_mut)]
                mut manual_access_builders: Vec<&AccessBuilder>,
            ) -> bool { 
                #[allow(unused_mut)]
                let mut claims: HashMap<usize, Vec<&AccessBuilder>> = HashMap::new();
                #[allow(unused_mut)]
                let mut absolute_index = 0;
                $(
                    let mut indexes = $params::claim_manual_access_builders(manual_access_builders.clone());
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
                        access_builders.extend(claimed_access_builders.into_iter().cloned());

                        match program_registry.resolve::<$params>(access_builders) {
                            Ok(Ok(item)) => item,
                            _ => {
                                return false;
                            }
                        }
                    };

                    absolute_index += 1;
                )*

                true
            }
        }
    };
}

macro_rules! impl_all_async_system_on_function_system {
    () => {
        use std::{sync::Arc, pin::Pin, collections::HashMap};
        pub use aion_program::prelude::{Injection, ProgramRegistry, ProgramId, ValuePassword, UserId, UserPassword, ResourceId, ResourceAccess, AccessBuilder};
        pub use crate::prelude::{FunctionSystemBase, AsyncSystem, SystemResult, SystemError};

        impl_async_system_on_function_system!();
    };

    ($first:ident $(, $rest:ident)*) => {
        impl_async_system_on_function_system!($first $(, $rest)*);
        impl_all_async_system_on_function_system!($($rest),*);
    };
}

impl_all_async_system_on_function_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9);