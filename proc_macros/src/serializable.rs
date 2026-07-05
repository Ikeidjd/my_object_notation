use proc_macro::TokenStream;
use quote::quote;
use syn::{Block, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, parse_macro_input};

pub fn derive_proc_macro_impl(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let where_clause = &generics.where_clause;

    let serialization = syn::parse_str::<Block>(&format!("{{{}}}", &match data {
        syn::Data::Struct(data_struct) => struct_serialization(data_struct),
        syn::Data::Enum(data_enum) => enum_serialization(data_enum),
        syn::Data::Union(_) => panic!("Can't serialize a union"),
    })).expect("Couldn't parse block");

    quote! {
        impl #generics my_object_notation::serializable::Serializable for #ident #generics #where_clause {
            fn serialize(&self) -> my_object_notation::mon_object::MonObject #serialization
        }
    }.into()
}

fn struct_serialization(data_struct: DataStruct) -> String {
    match data_struct.fields {
        syn::Fields::Named(fields) => named_fields(fields, true),
        syn::Fields::Unnamed(fields) => unnamed_fields(fields, true),
        syn::Fields::Unit => unit(),
    }
}

fn named_fields(fields: FieldsNamed, include_self: bool) -> String {
    let optional_self = match include_self {
        true => "self.",
        false => "",
    };

    let mut s = "my_object_notation::mon_object::MonObject::Dictionary(std::collections::HashMap::from([".to_owned();

    for field in fields.named {
        s.push_str(&format!("(\"{name}\".to_owned(), {optional_self}{name}.serialize()),", name = field.ident.expect("Couldn't get field name")));
    }

    s.push_str("]))");
    s
}

fn unnamed_fields(fields: FieldsUnnamed, include_self: bool) -> String {
    let optional_self = match include_self {
        true => "self.",
        false => "_",
    };

    let mut s = "my_object_notation::mon_object::MonObject::Array(vec![".to_owned();

    for (n, _) in fields.unnamed.iter().enumerate() {
        s.push_str(&format!("{optional_self}{n}.serialize(),"));
    }

    s.push_str("])");
    s
}

fn unit() -> String {
    format!("my_object_notation::mon_object::MonObject::Array(vec![])")
}

fn enum_serialization(data_enum: DataEnum) -> String {
    let mut s = "match self {".to_owned();

    for variant in data_enum.variants {
        let pattern = match &variant.fields {
            syn::Fields::Named(fields) => enum_named_fields_pattern(fields),
            syn::Fields::Unnamed(fields) => enum_unnamed_fields_pattern(fields),
            syn::Fields::Unit => enum_unit_pattern(),
        };

        s.push_str(&format!("Self::{name}{} => my_object_notation::mon_object::MonObject::Enum(\"{name}\".to_owned(), Box::new({})),", pattern, enum_fields_serialization(variant.fields), name = variant.ident));
    }

    s.push('}');
    s
}

fn enum_named_fields_pattern(fields: &FieldsNamed) -> String {
    format!("{{{}}}", fields.named.iter().map(|field| field.ident.as_ref().expect("Couldn't get enum variant field name").to_string()).collect::<Vec<_>>().join(","))
}

fn enum_unnamed_fields_pattern(fields: &FieldsUnnamed) -> String {
    format!("({})", fields.unnamed.iter().enumerate().map(|(n, _)| format!("_{n}")).collect::<Vec<_>>().join(","))
}

fn enum_unit_pattern() -> String {
    format!("")
}

fn enum_fields_serialization(fields: Fields) -> String {
    match fields {
        Fields::Named(fields) => named_fields(fields, false),
        Fields::Unnamed(fields) => unnamed_fields(fields, false),
        Fields::Unit => unit(),
    }
}
