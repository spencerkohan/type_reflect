#[derive(Clone, Debug)]
pub enum Type {
    Named(String),
    String,
    Int,
    UnsignedInt,
    Float,
    Boolean,
    Option(Box<Type>),
    Array(Box<Type>),
    Map { key: Box<Type>, value: Box<Type> },
}

#[derive(Clone, Debug)]
pub struct StructMember {
    pub name: String,
    pub type_: Type,
}

// #[derive(Clone, Debug)]
// pub struct TypeSlot {
//     pub optional: bool,
//     pub type_: Type,
// }
