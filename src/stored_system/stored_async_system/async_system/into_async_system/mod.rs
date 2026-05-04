use crate::prelude::AsyncSystem;

pub mod function_system_into_async_system;
pub mod impl_async_system_on_function_system;

pub trait IntoAsyncSystem<Input> {
    type System: AsyncSystem;

    fn into_system(self) -> Self::System;
}