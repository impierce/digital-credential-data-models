use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;

pub fn gen_paths(input: syn::DeriveInput) -> syn::Result<proc_macro::TokenStream> {
    /* Generate flat data (source, jpath, target, multiplicity, optionality)
        (Person, $.birthName, DateTime, 1, true),
        (Person, $.citizenship_country, Concept, 1, false),
        (Person, $.citizenship_country, Concept, *, true),
    */
    match &input.data {
        syn::Data::Enum(union) => return handle_enum(union, &input),
        syn::Data::Union(_) => {}
        syn::Data::Struct(obj) => {
            return handle_object(obj, &input);
        }
    }

    Ok(proc_macro::TokenStream::new())
}

pub struct SchemaTokensCtx {
    connected_schema_tokens: Vec<TokenStream>,
    unconnected_schema_tokens: Vec<TokenStream>,
    recurse_schema_tokens: Vec<TokenStream>,
}

pub struct TargetMetadata {
    target_type: Option<syn::Ident>,
    optional: bool,
    is_many: bool,
    is_one_or_many: bool,
}

impl Default for TargetMetadata {
    fn default() -> Self {
        Self {
            target_type: None,
            optional: false,
            is_many: false,
            is_one_or_many: false,
        }
    }
}

fn traverse_type(field_type: &syn::Type, meta: &mut TargetMetadata) {
    match field_type {
        syn::Type::Path(path_type) => {
            for segment in path_type.path.segments.iter() {
                let type_name = segment.ident.to_string();

                //println!("{}", segment.arguments);

                if &type_name == "OneOrMany" {
                    meta.is_one_or_many = true;
                    fetch_schema_target_ident(&segment.arguments, meta);
                } else if &type_name == "Vec" {
                    meta.is_many = true;
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

fn handle_enum(data_enum: &syn::DataEnum, input: &syn::DeriveInput) -> syn::Result<proc_macro::TokenStream> {
    let mut connected_schema_tokens = vec![];
    let mut unconnected_schema_tokens = vec![];
    let mut recurse_schema_tokens = vec![];
    let src_schema = &input.ident;

    for variant in data_enum.variants.iter() {
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                assert!(fields.unnamed.len() == 1);

                if let Some(first_field) = fields.unnamed.first() {
                    add_schema_data(
                        &mut connected_schema_tokens,
                        &mut unconnected_schema_tokens,
                        &mut recurse_schema_tokens,
                        src_schema,
                        "$".to_string(),
                        &first_field.ty,
                    );
                }
            }
            _ => {
                panic!("Can only handle unnamed variants");
            }
        }
    }

    let ctx = SchemaTokensCtx {
        recurse_schema_tokens,
        connected_schema_tokens,
        unconnected_schema_tokens,
    };

    implement_add_schema_types(input, ctx, false)
}

fn handle_object(object: &syn::DataStruct, input: &syn::DeriveInput) -> syn::Result<proc_macro::TokenStream> {
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

    match &object.fields {
        syn::Fields::Named(named) => {
            let ctx = handle_struct_fields(&input.ident, named, camel_case)?;
            return implement_add_schema_types(input, ctx, true);
        }
        _ => panic!("Can only handle structs with named fields"),
    }
}

fn implement_add_schema_types(
    input: &syn::DeriveInput,
    ctx: SchemaTokensCtx,
    add_schema: bool,
) -> syn::Result<proc_macro::TokenStream> {
    let src_schema = &input.ident;
    let generics = &input.generics;

    let connected_schema_tokens = TokenStream::from_iter(ctx.connected_schema_tokens);
    let unconnected_schema_tokens = TokenStream::from_iter(ctx.unconnected_schema_tokens);
    let recurse_schema_tokens = TokenStream::from_iter(ctx.recurse_schema_tokens);

    let expand = quote! {
        impl #generics AddSchemaTypes for #src_schema #generics {
            fn add_schema_types(data: &mut Vec<SchemaData>, parent_src_schema: &str, parent_json_path: &str, optional: bool) {
                if #src_schema::add_schema() {
                    #connected_schema_tokens
                } else {
                    #unconnected_schema_tokens
                }

                #recurse_schema_tokens
            }

            fn add_schema() -> bool {
                #add_schema
            }
        }
    };

    Ok(expand.into())
}

fn add_schema_data(
    connected_schema_tokens: &mut Vec<TokenStream>,
    unconnected_schema_tokens: &mut Vec<TokenStream>,
    recurse_schema_tokens: &mut Vec<TokenStream>,
    src_schema: &syn::Ident,
    json_path: String,
    field_type: &syn::Type,
) {
    let mut meta = TargetMetadata::default();
    traverse_type(field_type, &mut meta);

    if let Some(target_schema) = &meta.target_type {
        let multiplicity = if meta.is_one_or_many {
            quote! { Multiplicity::OneOrMany }
        } else if meta.is_many {
            quote! { Multiplicity::Many }
        } else {
            quote! { Multiplicity::One }
        };

        let optional = meta.optional;

        connected_schema_tokens.push(quote! {
            // If add schema we add ourselves
            data.push(SchemaData {
                src_schema: stringify!(#src_schema).to_string(),
                json_path: #json_path.to_string(),
                multiplicity: #multiplicity,
                tgt_schema: stringify!(#target_schema).to_string(),
                optional: #optional
            });
        });

        unconnected_schema_tokens.push(quote! {
            // In case of add_schema is false, we don't add the types so in the enum (AgentOrPersonOrOrganization)
            // The type AgentOrPersonOrOrganization is not added but its sub types. Will go like this ->
            //  DigitalCredential, $.issuer, Agent
            //  DigitalCredential, $.issuer, Person
            //  DigitalCredential, $.issuer, Organization
            data.push(SchemaData {
                src_schema: parent_src_schema.to_string(),
                json_path: parent_json_path.to_string(),
                multiplicity: #multiplicity,
                tgt_schema: stringify!(#target_schema).to_string(),
                optional: #optional
            });
        });

        recurse_schema_tokens.push(quote! {
            #target_schema::add_schema_types(data, stringify!(#target_schema), #json_path, #optional);
        });
    }
}

fn handle_struct_fields(
    src_schema: &syn::Ident,
    fields: &syn::FieldsNamed,
    rename_cc: bool,
) -> syn::Result<SchemaTokensCtx> {
    let mut connected_schema_tokens = vec![];
    let mut unconnected_schema_tokens = vec![];
    let mut recurse_schema_tokens = vec![];

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

            add_schema_data(
                &mut connected_schema_tokens,
                &mut unconnected_schema_tokens,
                &mut recurse_schema_tokens,
                &src_schema,
                json_path,
                &field.ty,
            );
        }
    }

    Ok(SchemaTokensCtx {
        connected_schema_tokens,
        unconnected_schema_tokens,
        recurse_schema_tokens,
    })
}
