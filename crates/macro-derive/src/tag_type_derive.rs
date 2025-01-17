use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

pub fn impl_tag_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let tag_source = &input.ident;
    let tag_target = format_ident!("{}Tag", input.ident);

    let expand = quote! {
        impl #tag_source {
            pub fn tag() -> #tag_target {
                #tag_target::default()
            }
        }

        /// Tag matching the struct name
        #[derive(Clone, Debug)]
        pub struct #tag_target(String);

        impl ::std::ops::Deref for #tag_target {
            type Target = String;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl types_common::AddSchemaTypes for #tag_target {}

        impl Default for #tag_target {
            fn default() -> Self {
                Self(stringify!(#tag_source).to_string())
            }
        }

        impl<'de> Deserialize<'de> for #tag_target {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let val = String::deserialize(deserializer)?;

                // if &val != "Person"
                if &val != stringify!(#tag_source) {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(&val),
                        &stringify!(#tag_source),
                    ))
                } else {
                    Ok(#tag_target(val))
                }
            }
        }

        impl Serialize for #tag_target {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.0)
            }
        }
    };

    expand.into()
}
