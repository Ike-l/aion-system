use std::marker::PhantomData;

pub struct FunctionSystemBase<Input, F> {
    pub f: F,
    pub marker: PhantomData<fn() -> Input>
}