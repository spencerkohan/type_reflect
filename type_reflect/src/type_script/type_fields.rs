use ts_quote::ts_string;
use type_reflect_core::{Inflectable, Inflection, NamedField, Type};

use crate::type_script::to_ts_type;

pub fn named_member(member: &NamedField, inflection: Inflection) -> String {
    let name = &member.name.inflect(inflection);
    match &member.type_ {
        type_reflect_core::Type::Option(t) => {
            let value = to_ts_type(&t);
            format!("{name}?: {value};", name = name, value = value)
        }
        t => {
            let value = to_ts_type(&t);
            format!("{name}: {value};", name = name, value = value)
        }
    }
}

pub fn named_fields(fields: &Vec<NamedField>, inflection: Inflection) -> String {
    let members: Vec<String> = fields
        .into_iter()
        .map(|field| named_member(field, inflection))
        .collect();
    members.join("\n  ")
}

pub fn tuple_fields(fields: &Vec<Type>) -> String {
    if fields.len() == 1 {
        let type_ = to_ts_type(&fields[0]);
        ts_string! { #type_ }
    } else {
        let tuple_items: Vec<String> = fields.into_iter().map(|item| to_ts_type(&item)).collect();
        let tuple_items = tuple_items.join(",\n        ");
        ts_string! {  [ #tuple_items ] }
    }
}
