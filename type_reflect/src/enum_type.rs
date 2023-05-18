use type_reflect_core::type_description::{EnumCase, EnumType};

pub trait EnumReflectionType {
    fn name() -> &'static str;
    fn cases() -> Vec<EnumCase>;
    fn enum_type() -> EnumType;
    fn rust() -> String;
}
