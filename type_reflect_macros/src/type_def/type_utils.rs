use proc_macro2::TokenStream;
use quote::*;
use type_reflect_core::*;

pub trait TypeBridge {
    fn type_(&self) -> &Type;
    fn emit_type(&self) -> TokenStream {
        match &self.type_() {
            Type::Named(name) => {
                quote! { Type::Named(#name.to_string()) }
            }
            Type::String => quote! { Type::String },
            Type::Int => quote! { Type::Int },
            Type::UnsignedInt => quote! { Type::UnsignedInt },
            Type::Float => quote! { Type::Float },
            Type::Boolean => quote! { Type::Boolean },
            Type::Option(t) => {
                let inner = t.emit_type();
                quote! { Type::Option( #inner.into() ) }
            }
            Type::Array(t) => {
                let inner = t.emit_type();
                quote! { Type::Array( #inner.into() ) }
            }
            Type::Map { key, value } => {
                let key = key.emit_type();
                let value = value.emit_type();
                quote! { Type::Map{ key: #key.into(), value: #value.into() } }
            }
        }
    }
}

impl TypeBridge for Type {
    fn type_(&self) -> &Type {
        self
    }
}

pub trait TypeFieldsDefinitionBridge {
    fn def(&self) -> &TypeFieldsDefinition;
    fn emit_def(&self) -> TokenStream {
        match &self.def() {
            TypeFieldsDefinition::Unit => quote! { TypeFieldsDefinition::Unit },
            TypeFieldsDefinition::Tuple(inner) => {
                let mut types = quote! {};

                for type_ in inner {
                    let t = type_.emit_type();
                    types.extend(quote! {#t, });
                }

                quote! { TypeFieldsDefinition::Tuple(vec![#types]) }
            }
            TypeFieldsDefinition::Named(inner) => {
                let mut mermbers = quote! {};

                for member in inner {
                    let m = member.emit_member();
                    mermbers.extend(quote! {#m, });
                }

                quote! { TypeFieldsDefinition::Named(vec![#mermbers]) }
            }
        }
    }
}

impl TypeFieldsDefinitionBridge for TypeFieldsDefinition {
    fn def(&self) -> &TypeFieldsDefinition {
        self
    }
}

pub trait NamedFieldBridge {
    fn member(&self) -> &NamedField;
    fn emit_member(&self) -> TokenStream {
        let member = &self.member();
        let name = &member.name;
        let type_ = member.type_.emit_type();
        quote! {
            NamedField {
                name: #name.to_string(),
                type_: #type_,
            }
        }
    }
}

impl NamedFieldBridge for NamedField {
    fn member(&self) -> &NamedField {
        self
    }
}
