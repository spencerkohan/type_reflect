use type_reflect_core::type_description::StructMember;

// #[derive(Clone, Debug)]
// pub struct StructMemberRecord {
//     pub name: &'static str,
//     pub type_: Type,
// }

pub trait StructType {
    fn name() -> &'static str;
    fn members() -> Vec<StructMember>;
}
