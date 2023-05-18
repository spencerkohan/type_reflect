use type_reflect_core::{type_description::StructMember, Inflection};

pub trait StructType {
    fn name() -> &'static str;
    fn inflection() -> Inflection;
    fn members() -> Vec<StructMember>;
    fn rust() -> String;
}
