use std::collections::HashSet;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse_macro_input, AngleBracketedGenericArguments, DeriveInput, FieldsNamed, Ident, PathArguments, Type, TypePath,
};

pub fn gen_paths(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let src_schema = &input.ident;

    fn implement_add_schema_types(src_schema: &Ident, tokens: TokenStream) -> proc_macro::TokenStream {
        let src_schema_str = src_schema.to_string();
        let expand = quote! {
            impl AddSchemaTypes for #src_schema {
                fn add_schema_types(map: &mut std::collections::HashMap<String, Vec<SchemaData>>) {
                    if map.contains_key(#src_schema_str) {
                        return;
                    }

                    let mut data = vec![];

                    #tokens

                    map.insert(#src_schema_str.to_string(), data);
                }
            }
        };

        expand.into()
    }

    match input.data {
        syn::Data::Enum(_) => {}
        syn::Data::Union(_) => {}
        syn::Data::Struct(obj) => {
            /* TODO generate triples
            vec![
                (Person, $.birth_name,,),
                (Person, $.citizenship_country, Concept, One),
                (Person, $.citizenship_country, Concept, Many),
            ]
            */
            match obj.fields {
                syn::Fields::Named(named) => {
                    let tokens = handle_named_fields(src_schema.to_string(), named);
                    return implement_add_schema_types(src_schema, tokens);
                }
                _ => {}
            }
        }
    }

    proc_macro::TokenStream::new()
}

#[derive(Default)]
pub struct TargetMetadata {
    target_type: Option<Ident>,
    optional: bool,
    is_array: bool,
}

fn traverse_type(field_type: &Type, meta: &mut TargetMetadata) {
    match field_type {
        Type::Path(path_type) => {
            for segment in path_type.path.segments.iter() {
                let type_name = segment.ident.to_string();

                if &type_name == "Vec" {
                    meta.is_array = true;
                    fetch_schema_target_ident(&segment.arguments, meta);
                } else if &type_name == "Option" {
                    meta.optional = true;
                    fetch_schema_target_ident(&segment.arguments, meta);
                } else {
                    meta.target_type = Some(segment.ident.clone());
                };
            }
        }
        _ => {}
    }
}

fn fetch_schema_target_ident(path_args: &PathArguments, meta: &mut TargetMetadata) {
    fn search_segments(ab_args: &AngleBracketedGenericArguments, meta: &mut TargetMetadata) {
        for arg in ab_args.args.iter() {
            match arg {
                syn::GenericArgument::Type(type_args) => traverse_type(type_args, meta),
                _ => {}
            }
        }
    }

    match path_args {
        syn::PathArguments::AngleBracketed(ab_args) => search_segments(ab_args, meta),
        _ => {
            // TODO?
            panic!("Not implemented");
        }
    }
}

fn handle_field(tokens: &mut Vec<TokenStream>, src_schema: &str, json_path: String, field_type: &Type) {
    let mut all_tgt_schemas = HashSet::new();

    let mut meta = TargetMetadata::default();
    traverse_type(field_type, &mut meta);

    if let Some(target_ident) = &meta.target_type {
        all_tgt_schemas.insert(target_ident.clone());

        let multiplicity = if meta.is_array {
            quote! { Multiplicity::Many }
        } else {
            quote! { Multiplicity::One }
        };

        tokens.push(quote! {
            data.push(SchemaData {
                src_schema: #src_schema.to_string(),
                json_path: #json_path.to_string(),
                multiplicity: #multiplicity,
                tgt_schema: stringify!(#target_ident).to_string(),
            });
        });
    }

    // TODO recurse into tree.
    for tgt_schema in all_tgt_schemas.iter() {
        tokens.push(quote! {
            #tgt_schema::add_schema_types(map);
        });
    }
}

fn handle_named_fields(src_schema: String, fields: FieldsNamed) -> TokenStream {
    let mut tokens = vec![];
    for field in fields.named.iter() {
        if let Some(field_ident) = &field.ident {
            // TODO apply serde renaming
            let json_path = format!("$.{}", field_ident);

            println!("{json_path}");
            handle_field(&mut tokens, &src_schema, json_path, &field.ty);
        }
    }

    TokenStream::from_iter(tokens)
}
