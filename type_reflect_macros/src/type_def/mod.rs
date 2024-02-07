use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{ItemEnum, ItemStruct, ItemType, Result};

mod enum_def;
mod struct_def;
mod type_alias_def;
pub use type_alias_def::*;

pub mod syn_type_utils;

pub mod type_utils;

pub use enum_def::*;
pub use struct_def::StructDef;
use type_reflect_core::Inflection;

#[derive(Clone, Debug)]
pub enum TypeDef {
    Struct(StructDef),
    Enum(EnumDef),
    Alias(TypeAliasDef),
}

impl TypeDef {
    pub fn struct_def(item: &ItemStruct) -> Result<Self> {
        Ok(TypeDef::Struct(StructDef::new(item)?))
    }
    pub fn alias_def(item: &ItemType) -> Result<Self> {
        Ok(TypeDef::Alias(TypeAliasDef::new(item)?))
    }
    pub fn enum_def(item: &ItemEnum) -> Result<Self> {
        // println!("ATTRIBUTES:");
        // for attr in &item.attrs {
        //     println!("    {:?}", attr);
        // }
        Ok(TypeDef::Enum(EnumDef::new(item)?))
    }

    pub fn emit(&self) -> TokenStream {
        match self {
            TypeDef::Struct(s) => s.emit(),
            TypeDef::Enum(e) => e.emit(),
            TypeDef::Alias(t) => t.emit(),
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
    // fn emit_type_def_impl(&self) -> TokenStream {
    //     match self {
    //         TypeDef::Struct(s) => s.emit_type_def_impl(),
    //         TypeDef::Enum(e) => e.emit_type_def_impl(),
    //         TypeDef::Alias(_) => todo!(),
    //     }
    // }
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

pub trait InflectionTokenProvider {
    fn inflection(&self) -> &Inflection;
    fn to_tokens(&self) -> TokenStream {
        match &self.inflection() {
            Inflection::Lower => quote!(Inflection::Lower),
            Inflection::Upper => quote!(Inflection::Upper),
            Inflection::Camel => quote!(Inflection::Camel),
            Inflection::Snake => quote!(Inflection::Snake),
            Inflection::Pascal => quote!(Inflection::Pascal),
            Inflection::ScreamingSnake => quote!(Inflection::ScreamingSnake),
            Inflection::Kebab => quote!(Inflection::Kebab),
            Inflection::None => quote!(Inflection::None),
        }
    }
}

impl InflectionTokenProvider for Inflection {
    fn inflection(&self) -> &Inflection {
        self
    }
}
