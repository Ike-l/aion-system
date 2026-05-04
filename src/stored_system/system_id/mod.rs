use std::any::TypeId;

pub enum SystemId {
    Label(String),
    TypeId(TypeId)
}