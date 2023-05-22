use crate::Inflection;

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

#[derive(Clone, Debug)]
pub struct EnumCase {
    pub name: String,
    pub type_: EnumCaseType,
    pub inflection: Inflection,
}

#[derive(Clone, Debug)]
pub enum EnumCaseType {
    Simple,
    Tuple(Vec<Type>),
    Struct(Vec<StructMember>),
}

#[derive(Clone, Debug)]
pub enum EnumType {
    Simple,
    Complex {
        case_key: String,
        content_key: Option<String>,
    },
}

// #[derive(Clone, Debug)]
// pub struct TypeSlot {
//     pub optional: bool,
//     pub type_: Type,
// }
