use type_reflect_core::type_description::StructMember;

pub trait StructType {
    fn name() -> &'static str;
    fn members() -> Vec<StructMember>;
    fn rust() -> String;
}
