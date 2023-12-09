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
pub struct NamedField {
    pub name: String,
    pub type_: Type,
}

#[derive(Clone, Debug)]
pub struct EnumCase {
    pub name: String,
    pub type_: TypeFieldDefinition,
    pub inflection: Inflection,
}

/**
The TypeFieldDefinition represents the set of fields for a type

This is used both in the context of a struct definition, and for enum variants
*/
#[derive(Clone, Debug)]
pub enum TypeFieldDefinition {
    /**
    The Unit field definition describes a type which does not contain data
    */
    Unit,
    /**
    The Tuple field definition describes a type which contains anonymous fields, identified by index
    */
    Tuple(Vec<Type>),
    /**
    The Named field definition describes a type which contains named fields, identified by name
    */
    Named(Vec<NamedField>),
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
