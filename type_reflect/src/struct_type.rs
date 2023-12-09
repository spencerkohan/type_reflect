use type_reflect_core::{type_description::NamedField, Inflection};

/// A type implementing `StructType` can
/// be used to emit a struct representation
pub trait StructType {
    fn name() -> &'static str;
    fn inflection() -> Inflection;
    fn fields() -> Vec<NamedField>;
    fn rust() -> String;
}
