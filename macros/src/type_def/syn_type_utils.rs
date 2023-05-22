use syn::{Field, GenericArgument, PathArguments, Result, Type as SynType, TypePath};
use type_reflect_core::{syn_err, StructMember, Type};

fn leading_segment(path: &TypePath) -> String {
    path.path.segments[0].ident.to_string()
}

fn generic_args(path: &TypePath) -> Result<Vec<Type>> {
    match &path.path.segments[0].arguments {
        PathArguments::None => Ok(vec![]),
        PathArguments::AngleBracketed(args) => (&args.args)
            .into_iter()
            .map(|arg| match arg {
                GenericArgument::Type(inner_ty) => inner_ty.to_type(),
                _ => syn_err!("Generic argument must be a type: {:#?}", arg),
            })
            .collect(),
        _ => syn_err!(
            "Argument type not supported: {:#?}",
            &path.path.segments[0].arguments
        ),
    }
}

fn simple_type(name: String) -> Type {
    match name.as_str() {
        "String" => Type::String,
        "bool" => Type::Boolean,
        "u8" | "u16" | "u32" | "u64" => Type::UnsignedInt,
        "i8" | "i16" | "i32" | "i64" => Type::Int,
        "f8" | "f16" | "f32" | "f64" => Type::Float,
        _ => Type::Named(name),
    }
}

pub trait SynTypeBridge {
    fn syn_type(&self) -> &syn::Type;
    fn to_type(&self) -> Result<Type> {
        match self.syn_type() {
            SynType::Path(type_path)
                if type_path.qself.is_none()
                    && type_path.path.leading_colon.is_none()
                    && type_path.path.segments.len() == 1 =>
            {
                let leading = leading_segment(type_path);
                let generics = generic_args(type_path)?;
                match leading.as_str() {
                    "Option" if generics.len() == 1 => Ok(Type::Option(generics[0].clone().into())),
                    "Vec" if generics.len() == 1 => Ok(Type::Array(generics[0].clone().into())),
                    "HashMap" if generics.len() == 2 => Ok(Type::Map {
                        key: generics[1].clone().into(),
                        value: generics[1].clone().into(),
                    }),
                    _ if generics.len() == 0 => Ok(simple_type(leading)),
                    _ => syn_err!("Unsupported type type: {:#?}", &self.syn_type()),
                }
            }
            _ => syn_err!("Unsupported type: {:#?}", &self.syn_type()),
        }
    }
}

impl SynTypeBridge for syn::Type {
    fn syn_type(&self) -> &syn::Type {
        self
    }
}

fn get_struct_member(field: &Field) -> Result<StructMember> {
    println!("Getting struct member from field: {:#?}", field);
    let name = match &field.ident {
        None => panic!("Struct fields must be named: {:#?}", field),
        Some(ident) => format!("{}", ident),
    };

    let type_ = field.ty.to_type()?;

    Ok(StructMember { name, type_ })
}

fn get_field_type(field: &Field) -> Result<Type> {
    println!("Getting tuple member from field: {:#?}", field);
    match &field.ident {
        None => {}
        Some(_ident) => panic!("Tuple fields must not be named: {:#?}", field),
    };

    let type_ = field.ty.to_type()?;

    Ok(type_)
}

pub trait FieldsNamedBridge {
    fn fields_named(&self) -> &syn::FieldsNamed;
    fn to_struct_members(&self) -> Result<Vec<StructMember>> {
        (&self.fields_named().named)
            .into_iter()
            .map(|field: &Field| get_struct_member(&field))
            .collect()
    }
}

impl FieldsNamedBridge for syn::FieldsNamed {
    fn fields_named(&self) -> &syn::FieldsNamed {
        self
    }
}

pub trait FieldsUnmnamedBridge {
    fn fields_unnamed(&self) -> &syn::FieldsUnnamed;
    fn to_tuple_members(&self) -> Result<Vec<Type>> {
        (&self.fields_unnamed().unnamed)
            .into_iter()
            .map(|field: &Field| get_field_type(&field))
            .collect()
    }
}

impl FieldsUnmnamedBridge for syn::FieldsUnnamed {
    fn fields_unnamed(&self) -> &syn::FieldsUnnamed {
        self
    }
}
