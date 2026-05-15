use crate::prelude::AsyncSystemExecutable;

pub trait IntoAsyncSystem<Input> {
    type System: AsyncSystemExecutable;

    fn into_system(self) -> Self::System;
}