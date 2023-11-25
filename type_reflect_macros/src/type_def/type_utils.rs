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

pub trait EnumCaseBridge {
    fn case(&self) -> &EnumCaseType;
    fn emit_case(&self) -> TokenStream {
        match &self.case() {
            EnumCaseType::Simple => quote! { EnumCaseType::Simple },
            EnumCaseType::Tuple(inner) => {
                let mut types = quote! {};

                for type_ in inner {
                    let t = type_.emit_type();
                    types.extend(quote! {#t, });
                }

                quote! { EnumCaseType::Tuple(vec![#types]) }
            }
            EnumCaseType::Struct(inner) => {
                let mut mermbers = quote! {};

                for member in inner {
                    let m = member.emit_member();
                    mermbers.extend(quote! {#m, });
                }

                quote! { EnumCaseType::Struct(vec![#mermbers]) }
            }
        }
    }
}

impl EnumCaseBridge for EnumCaseType {
    fn case(&self) -> &EnumCaseType {
        self
    }
}

pub trait StructMemberBridge {
    fn member(&self) -> &StructMember;
    fn emit_member(&self) -> TokenStream {
        let member = &self.member();
        let name = &member.name;
        let type_ = member.type_.emit_type();
        quote! {
            StructMember {
                name: #name.to_string(),
                type_: #type_,
            }
        }
    }
}

impl StructMemberBridge for StructMember {
    fn member(&self) -> &StructMember {
        self
    }
}
