use type_reflect_core::{
    type_description::{EnumCase, EnumType},
    Inflection,
};

/// A type implementing `EnumReflectionType` can
/// be used to emit a enum representation
pub trait EnumReflectionType {
    fn name() -> &'static str;
    fn inflection() -> Inflection;
    fn cases() -> Vec<EnumCase>;
    fn enum_type() -> EnumType;
    fn rust() -> String;
}
