use std::marker::PhantomData;

pub struct FunctionSystemBase<Input, F> {
    f: F,
    marker: PhantomData<Input>
}