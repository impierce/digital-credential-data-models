mod enum_derive;
mod tag_type_derive;

#[proc_macro_derive(EnumDeserialize)]
pub fn enum_deserialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    enum_derive::impl_enum_deserialize(input)
}

#[proc_macro_derive(TagType)]
pub fn tag_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    tag_type_derive::impl_tag_type(input)
}
