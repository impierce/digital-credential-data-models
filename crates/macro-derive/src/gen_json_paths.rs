use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, FieldsNamed, Ident, Type, TypePath};

pub fn gen_paths(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let src_schema = &input.ident;

    fn implement_add_schema_types(src_schema: &Ident, tokens: TokenStream) -> proc_macro::TokenStream {
        let src_schema_str = src_schema.to_string();
        let expand = quote! {
            impl AddSchemaTypes for #src_schema {
                fn add_schema_types(map: &mut HashMap<String, Vec<SchemaData>>) {
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
                } ,
                _ => {}
            }
        }
    }

    proc_macro::TokenStream::new()
}

// Return object or vector

fn handle_field(tokens: &mut Vec<TokenStream>, src_schema: &str, json_path: String, field_type: &Type) {
    let mut all_tgt_schemas = HashSet::new();

    match field_type {
        syn::Type::Path(path_type) => {
            for segment in path_type.path.segments.iter() {
                all_tgt_schemas.insert(segment.ident.clone());
                let tgt_schema = segment.ident.to_string();

                // TODO something with args like Option<T>
                let args_str = if segment.arguments.is_empty() {
                    "has no args"
                } else {
                    "has args"
                };

                println!("{tgt_schema}: {args_str}");

                let multiplicity = if &tgt_schema == "Vec" {
                    quote! { Multiplicity::Many }
                } else {
                    quote! { Multiplicity::One }
                };

                tokens.push(quote! {
                    data.push(SchemaData {
                        src_schema: #src_schema.to_string(),
                        json_path: #json_path.to_string(),
                        multiplicity: #multiplicity,
                        tgt_schema: #tgt_schema.to_string(),
                    });
                });
            }
        }
        _ => {}
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
