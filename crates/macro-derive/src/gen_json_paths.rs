use std::collections::HashSet;
use proc_macro2::TokenStream;
use quote::quote;
use convert_case::{Case, Casing};

pub fn gen_paths(input: syn::DeriveInput) -> syn::Result<proc_macro::TokenStream> {
    let src_schema = &input.ident;

    fn implement_add_schema_types(
        src_schema: &syn::Ident,
        tokens: TokenStream,
    ) -> syn::Result<proc_macro::TokenStream> {
        let src_schema_str = src_schema.to_string();
        let expand = quote! {
            impl AddSchemaTypes for #src_schema {
                fn add_schema_types(map: &mut ::std::collections::HashMap<String, Vec<SchemaData>>) {
                    if map.contains_key(#src_schema_str) {
                        return;
                    }

                    let mut data = vec![];

                    #tokens

                    map.insert(#src_schema_str.to_string(), data);
                }
            }
        };

        Ok(expand.into())
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
                    let mut camel_case = false;

                    for attr in input.attrs.iter() {
                        if attr.path().is_ident("serde") {
                            attr.parse_nested_meta(|meta| {
                                if meta.path.is_ident("rename_all") {
                                    let value = meta.value()?;
                                    let s: syn::LitStr = value.parse()?;
                                    if s.value() == "camelCase" {
                                        camel_case = true;
                                    }
                                }

                                Ok(())
                            })?;
                        }
                    }
                    let tokens = handle_named_fields(src_schema.to_string(), named, camel_case)?;
                    return implement_add_schema_types(src_schema, tokens);
                }
                _ => {}
            }
        }
    }

    Ok(proc_macro::TokenStream::new())
}

pub struct TargetMetadata {
    target_type: Option<syn::Ident>,
    optional: bool,
    is_array: bool,
    is_one: bool,
}

impl Default for TargetMetadata {
    fn default() -> Self {
        Self {
            target_type: None,
            optional: false,
            is_array: false,
            is_one: true,
        }
    }
}

fn traverse_type(field_type: &syn::Type, meta: &mut TargetMetadata) {
    match field_type {
        syn::Type::Path(path_type) => {
            for segment in path_type.path.segments.iter() {
                let type_name = segment.ident.to_string();

                if &type_name == "OneOrMany" {
                    meta.is_array = true;
                    fetch_schema_target_ident(&segment.arguments, meta);
                } else if &type_name == "Vec" {
                    meta.is_array = true;
                    meta.is_one = false;
                    fetch_schema_target_ident(&segment.arguments, meta);
                } else if &type_name == "Option" {
                    meta.optional = true;
                    fetch_schema_target_ident(&segment.arguments, meta);
                } else if !segment.arguments.is_empty() {
                    fetch_schema_target_ident(&segment.arguments, meta);
                } else {
                    meta.target_type = Some(segment.ident.clone());
                }
            }
        }
        _ => {}
    }
}

fn fetch_schema_target_ident(path_args: &syn::PathArguments, meta: &mut TargetMetadata) {
    fn search_segments(ab_args: &syn::AngleBracketedGenericArguments, meta: &mut TargetMetadata) {
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

fn handle_field(tokens: &mut Vec<TokenStream>, src_schema: &str, json_path: String, field_type: &syn::Type) {
    let mut all_tgt_schemas = HashSet::new();

    let mut meta = TargetMetadata::default();
    traverse_type(field_type, &mut meta);

    if let Some(target_ident) = &meta.target_type {
        all_tgt_schemas.insert(target_ident.clone());

        let multiplicity = if meta.is_array && meta.is_one {
            quote! { Multiplicity::OneOrMany }
        } else if meta.is_array {
            quote! { Multiplicity::Many }
        } else {
            quote! { Multiplicity::One }
        };

        let optional = meta.optional;

        tokens.push(quote! {
            data.push(SchemaData {
                src_schema: #src_schema.to_string(),
                json_path: #json_path.to_string(),
                multiplicity: #multiplicity,
                tgt_schema: stringify!(#target_ident).to_string(),
                optional: #optional
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

fn handle_named_fields(src_schema: String, fields: syn::FieldsNamed, rename_cc: bool) -> syn::Result<TokenStream> {
    let mut tokens = vec![];

    for field in fields.named.iter() {
        if let Some(field_ident) = &field.ident {
            let mut field_name = field_ident.to_string();

            if rename_cc {
                field_name = field_name.to_case(Case::Camel)
            }

            for attr in field.attrs.iter() {
                if attr.path().is_ident("serde") {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("rename") {
                            let value = meta.value()?;
                            let s: syn::LitStr = value.parse()?;
                            field_name = s.value();
                        }

                        Ok(())
                    })?;
                }
            }

            println!("{}", field_name);
            let json_path = format!("$.{}", field_name);

            handle_field(&mut tokens, &src_schema, json_path, &field.ty);
        }
    }

    Ok(TokenStream::from_iter(tokens))
}
