use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn impl_enum_deserialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let mut variants: Vec<proc_macro2::TokenStream> = vec![];
    let mut variant_names: Vec<String> = vec![];

    fn invalid_enum() {
        panic!("EnumDeserialize may only have one unnamed field");
    }

    if let syn::Data::Enum(curr_enum) = &input.data {
        for variant in curr_enum.variants.iter() {
            if variant.fields.is_empty() || 1 < variant.fields.len() {
                invalid_enum();
            }

            match &variant.fields {
                syn::Fields::Unnamed(_) => {}
                _ => invalid_enum(),
            }

            let variant = &variant.ident;

            // Deserialize the object if matches the tag
            variants.push(quote! {
                if (&tag == stringify!(#variant)) {
                    return
                        Ok(Self::#variant(serde_json::from_value(serde_value).map_err(::serde::de::Error::custom)?));
                }
            });

            variant_names.push(variant.to_string());
        }
    } else {
        panic!("EnumDeserialize only works on enums");
    }

    fn create_choices(variant_names: &[String]) -> proc_macro2::TokenStream {
        let mut choices = "&[".to_string();

        for (i, variant) in variant_names.iter().enumerate() {
            choices.push_str(&format!(r#""{}""#, variant));
            if i + 1 < variant_names.len() {
                choices.push_str(", ");
            }
        }

        choices.push(']');
        choices.parse().unwrap()
    }

    let choices = create_choices(&variant_names);
    let parse_variants = proc_macro2::TokenStream::from_iter(variants);

    // This will add the trait to the struct.
    let expanded = quote! {
        impl<'de> ::serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let serde_value = serde_json::Value::deserialize(deserializer)?;

                // Look if type exist on the json object.
                let tag = serde_value
                    .get("type")
                    .map(|t| t.as_str().map(|s| s.to_string()))
                    .ok_or(::serde::de::Error::missing_field("type"))?;

                if let Some(tag) = tag {
                    // Deserialize the correct variant
                    #parse_variants

                    Err(::serde::de::Error::unknown_variant(&tag, #choices))
                } else {
                    Err(::serde::de::Error::missing_field("type"))
                }
            }
        }
    };

    expanded.into()
}
