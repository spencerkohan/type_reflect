use super::{named_fields, tuple_fields};
use ts_quote::ts_string;
use type_reflect_core::{Inflection, TypeFieldsDefinition};

pub fn struct_impl(name: &str, fields: &TypeFieldsDefinition, inflection: Inflection) -> String {
    let fields = match fields {
        TypeFieldsDefinition::Unit => todo!(),
        TypeFieldsDefinition::Tuple(tuple) => {
            let fields = tuple_fields(tuple);
            ts_string! {
                #fields
            }
        }
        TypeFieldsDefinition::Named(named) => {
            let fields = named_fields(named, inflection);
            ts_string! {
                { #fields }
            }
        }
    };
    ts_string! {
        export type #name = #fields;
    }
}
