use type_reflect_core::type_description::Type;

/// A type implementing `AliasType` can
/// be used to emit a type alias representation
pub trait AliasType {
    fn name() -> &'static str;
    fn source_type() -> Type;
    fn rust() -> String;
}
