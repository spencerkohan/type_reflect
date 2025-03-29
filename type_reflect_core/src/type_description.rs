use crate::Inflection;

#[derive(Clone, Debug)]
pub struct NamedType {
    pub name: String,
    pub generic_args: Vec<Box<Type>>,
}

#[derive(Clone, Debug)]
pub enum TransparentTypeCase {
    Box,
    Rc,
    Arc,
}

#[derive(Clone, Debug)]
pub struct TransparentType {
    pub case: TransparentTypeCase,
    pub type_: Box<Type>,
}

#[derive(Clone, Debug)]
pub enum Type {
    Named(NamedType),
    String,
    Int,
    UnsignedInt,
    Float,
    Boolean,
    Transparent(TransparentType),
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
    pub type_: TypeFieldsDefinition,
    pub inflection: Inflection,
}

/**
The TypeFieldDefinition represents the set of fields for a type

This is used both in the context of a struct definition, and for enum variants
*/
#[derive(Clone, Debug)]
pub enum TypeFieldsDefinition {
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
