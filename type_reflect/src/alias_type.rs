use type_reflect_core::{type_description::Type, Inflection};

pub trait AliasType {
    fn name() -> &'static str;
    fn source_type() -> Type;
    fn rust() -> String;
}
