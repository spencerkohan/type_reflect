use proc_macro2::{Ident, TokenStream};
use quote::{quote};
use syn::{
    ItemEnum, ItemStruct, Result,
};

mod enum_def;
mod struct_def;

pub mod syn_type_utils;
pub use syn_type_utils::*;

pub mod type_utils;

pub use enum_def::*;
pub use struct_def::StructDef;

#[derive(Clone, Debug)]
pub enum TypeDef {
    Struct(StructDef),
    Enum(EnumDef),
}

impl TypeDef {
    pub fn struct_def(item: &ItemStruct) -> Result<Self> {
        Ok(TypeDef::Struct(StructDef::new(item)?))
    }
    pub fn enum_def(item: &ItemEnum) -> Self {
        TypeDef::Enum(EnumDef::new(item))
    }

    pub fn emit(&self) -> TokenStream {
        match self {
            TypeDef::Struct(s) => s.emit(),
            TypeDef::Enum(_) => todo!(),
        }
    }
}

impl RustTypeEmitter for TypeDef {
    fn ident(&self) -> &Ident {
        panic!("unimplemented")
    }
    fn tokens(&self) -> &TokenStream {
        panic!("unimplemented")
    }
    fn emit_type_def_impl(&self) -> TokenStream {
        match self {
            TypeDef::Struct(s) => s.emit_type_def_impl(),
            TypeDef::Enum(e) => e.emit_type_def_impl(),
        }
    }
}

pub trait RustTypeEmitter {
    fn ident(&self) -> &Ident;
    fn tokens(&self) -> &TokenStream;
    fn emit_type_def_impl(&self) -> TokenStream {
        let ident = &self.ident();
        let token_string = format!("{}", self.tokens());
        quote! {
            impl RustType for #ident {
                fn emit_rust(&self) -> String {
                    #token_string.to_string()
                }
            }
        }
    }
}
