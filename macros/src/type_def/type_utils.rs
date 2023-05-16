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
