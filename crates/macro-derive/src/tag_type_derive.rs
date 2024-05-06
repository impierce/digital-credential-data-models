use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

pub fn impl_tag_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let tag_value = &input.ident;
    let tag_struct = format_ident!("{}Tag", input.ident);

    let expand = quote! {
        /// Tag matching the struct name
        #[derive(Clone, Debug, serde::Serialize)]
        pub struct #tag_struct(String);

        impl ::std::ops::Deref for #tag_struct {
            type Target = String;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl Default for #tag_struct {
            fn default() -> Self {
                Self(stringify!(#tag_value).to_string())
            }
        }

        impl<'de> Deserialize<'de> for #tag_struct {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let val = String::deserialize(deserializer)?;

                // if &val != "Person"
                if &val != stringify!(#tag_value) {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(&val),
                        &stringify!(#tag_value),
                    ))
                } else {
                    Ok(#tag_struct(val))
                }
            }
        }
    };

    expand.into()
}
