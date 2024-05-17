$.street
String: has no args
$.number
usize: has no args
$.name
String: has no args
$.sur_name
String: has no args
$.age
usize: has no args
$.address
Address: has no args
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub use macro_derive::*;
pub use email_address::*;
use serde::Serialize;
use serde::{de, de::DeserializeOwned, de::Unexpected, Deserializer};
use std::{fmt, ops::Deref};
mod traits {
    use std::{collections::HashMap, path::PathBuf};
    use chrono::{DateTime, TimeZone, Utc};
    use macro_derive::GenPaths;
    pub enum Multiplicity {
        One,
        Many,
    }
    pub struct SchemaData {
        pub src_schema: String,
        pub json_path: String,
        pub tgt_schema: String,
        pub multiplicity: Multiplicity,
    }
    pub trait AddSchemaTypes {
        fn add_schema_types(map: &mut HashMap<String, Vec<SchemaData>>);
    }
    pub struct Address {
        pub street: String,
        pub number: usize,
    }
    impl AddSchemaTypes for Address {
        fn add_schema_types(map: &mut HashMap<String, Vec<SchemaData>>) {
            if map.contains_key("Address") {
                return;
            }
            let mut data = ::alloc::vec::Vec::new();
            data.push(SchemaData {
                src_schema: "Address".to_string(),
                json_path: "$.street".to_string(),
                multiplicity: Multiplicity::One,
                tgt_schema: "String".to_string(),
            });
            String::add_schema_types(map);
            data.push(SchemaData {
                src_schema: "Address".to_string(),
                json_path: "$.number".to_string(),
                multiplicity: Multiplicity::One,
                tgt_schema: "usize".to_string(),
            });
            usize::add_schema_types(map);
            map.insert("Address".to_string(), data);
        }
    }
    pub struct Person {
        name: String,
        sur_name: String,
        age: usize,
        address: Address,
    }
    impl AddSchemaTypes for Person {
        fn add_schema_types(map: &mut HashMap<String, Vec<SchemaData>>) {
            if map.contains_key("Person") {
                return;
            }
            let mut data = ::alloc::vec::Vec::new();
            data.push(SchemaData {
                src_schema: "Person".to_string(),
                json_path: "$.name".to_string(),
                multiplicity: Multiplicity::One,
                tgt_schema: "String".to_string(),
            });
            String::add_schema_types(map);
            data.push(SchemaData {
                src_schema: "Person".to_string(),
                json_path: "$.sur_name".to_string(),
                multiplicity: Multiplicity::One,
                tgt_schema: "String".to_string(),
            });
            String::add_schema_types(map);
            data.push(SchemaData {
                src_schema: "Person".to_string(),
                json_path: "$.age".to_string(),
                multiplicity: Multiplicity::One,
                tgt_schema: "usize".to_string(),
            });
            usize::add_schema_types(map);
            data.push(SchemaData {
                src_schema: "Person".to_string(),
                json_path: "$.address".to_string(),
                multiplicity: Multiplicity::One,
                tgt_schema: "Address".to_string(),
            });
            Address::add_schema_types(map);
            map.insert("Person".to_string(), data);
        }
    }
    impl AddSchemaTypes for usize {
        fn add_schema_types(_map: &mut HashMap<String, Vec<SchemaData>>) {}
    }
    impl AddSchemaTypes for u8 {
        fn add_schema_types(_map: &mut HashMap<String, Vec<SchemaData>>) {}
    }
    impl AddSchemaTypes for u16 {
        fn add_schema_types(_map: &mut HashMap<String, Vec<SchemaData>>) {}
    }
    impl AddSchemaTypes for u32 {
        fn add_schema_types(_map: &mut HashMap<String, Vec<SchemaData>>) {}
    }
    impl AddSchemaTypes for u64 {
        fn add_schema_types(_map: &mut HashMap<String, Vec<SchemaData>>) {}
    }
    impl AddSchemaTypes for u128 {
        fn add_schema_types(_map: &mut HashMap<String, Vec<SchemaData>>) {}
    }
    impl AddSchemaTypes for isize {
        fn add_schema_types(_map: &mut HashMap<String, Vec<SchemaData>>) {}
    }
    impl AddSchemaTypes for i8 {
        fn add_schema_types(_map: &mut HashMap<String, Vec<SchemaData>>) {}
    }
    impl AddSchemaTypes for i16 {
        fn add_schema_types(_map: &mut HashMap<String, Vec<SchemaData>>) {}
    }
    impl AddSchemaTypes for i32 {
        fn add_schema_types(_map: &mut HashMap<String, Vec<SchemaData>>) {}
    }
    impl AddSchemaTypes for i64 {
        fn add_schema_types(_map: &mut HashMap<String, Vec<SchemaData>>) {}
    }
    impl AddSchemaTypes for i128 {
        fn add_schema_types(_map: &mut HashMap<String, Vec<SchemaData>>) {}
    }
    impl AddSchemaTypes for String {
        fn add_schema_types(_map: &mut HashMap<String, Vec<SchemaData>>) {}
    }
    impl AddSchemaTypes for bool {
        fn add_schema_types(_map: &mut HashMap<String, Vec<SchemaData>>) {}
    }
    impl AddSchemaTypes for PathBuf {
        fn add_schema_types(_map: &mut HashMap<String, Vec<SchemaData>>) {}
    }
    impl<T: TimeZone> AddSchemaTypes for DateTime<T> {
        fn add_schema_types(_map: &mut HashMap<String, Vec<SchemaData>>) {}
    }
}
pub enum OneOrMany<T> {
    One(Box<T>),
    Many(Vec<T>),
}
#[automatically_derived]
impl<T: ::core::clone::Clone> ::core::clone::Clone for OneOrMany<T> {
    #[inline]
    fn clone(&self) -> OneOrMany<T> {
        match self {
            OneOrMany::One(__self_0) => {
                OneOrMany::One(::core::clone::Clone::clone(__self_0))
            }
            OneOrMany::Many(__self_0) => {
                OneOrMany::Many(::core::clone::Clone::clone(__self_0))
            }
        }
    }
}
#[automatically_derived]
impl<T: ::core::fmt::Debug> ::core::fmt::Debug for OneOrMany<T> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            OneOrMany::One(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "One", &__self_0)
            }
            OneOrMany::Many(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Many", &__self_0)
            }
        }
    }
}
impl<'de, T: DeserializeOwned + fmt::Debug> de::Deserialize<'de> for OneOrMany<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let serde_value = serde_json::Value::deserialize(deserializer)?;
        if let serde_json::Value::Array(arr) = serde_value {
            let val = serde_json::Value::Array(arr);
            let out = Vec::deserialize(val).map_err(de::Error::custom)?;
            Ok(Self::Many(out))
        } else {
            let out = serde_json::from_value(serde_value).map_err(de::Error::custom)?;
            Ok(Self::One(out))
        }
    }
}
impl<T: Serialize> Serialize for OneOrMany<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            OneOrMany::One(obj) => obj.serialize(serializer),
            OneOrMany::Many(vec) => vec.serialize(serializer),
        }
    }
}
/// Email
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///   "type": "string",
///   "oneOf": [
///     {
///       "type": "string",
///       "format": "email"
///     },
///     {
///       "type": "string",
///       "format": "uri",
///       "pattern": "^mailto:[^@]*[^\\.]@[^\\.]($|[^@]*[^\\.]$)"
///     }
///   ]
/// }
/// ```
/// </details>
#[serde(try_from = "String")]
pub struct Email(pub String);
#[automatically_derived]
impl ::core::fmt::Debug for Email {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Email", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Email {
    #[inline]
    fn clone(&self) -> Email {
        Email(::core::clone::Clone::clone(&self.0))
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Email {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Email {
    #[inline]
    fn eq(&self, other: &Email) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Email {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<String>;
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Email {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serializer::serialize_newtype_struct(__serializer, "Email", &self.0)
        }
    }
};
impl Deref for Email {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'de> de::Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let email = String::deserialize(deserializer)?;
        let email_regex = regex::Regex::new("^[\\w-\\.]+@([\\w-]+\\.)+[\\w-]{2,4}$")
            .unwrap();
        let email_uri_regex = regex::Regex::new(
                "^mailto:[^@]*[^\\.]@[^\\.]($|[^@]*[^\\.]$)",
            )
            .unwrap();
        let valid = email_regex.is_match(&email) || email_uri_regex.is_match(&email);
        if valid {
            Ok(Email(email))
        } else {
            Err(
                de::Error::invalid_value(
                    Unexpected::Str(&email),
                    &"A valid email format",
                ),
            )
        }
    }
}
pub struct PositiveInteger(pub u32);
#[automatically_derived]
impl ::core::clone::Clone for PositiveInteger {
    #[inline]
    fn clone(&self) -> PositiveInteger {
        PositiveInteger(::core::clone::Clone::clone(&self.0))
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for PositiveInteger {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "PositiveInteger", &&self.0)
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for PositiveInteger {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serializer::serialize_newtype_struct(
                __serializer,
                "PositiveInteger",
                &self.0,
            )
        }
    }
};
impl std::ops::Deref for PositiveInteger {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'de> de::Deserialize<'de> for PositiveInteger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let number: i64 = i64::deserialize(deserializer)?;
        if 0 <= number {
            Ok(PositiveInteger(number as u32))
        } else {
            Err(
                <D::Error as de::Error>::invalid_value(
                    Unexpected::Signed(number),
                    &"A positive integer",
                ),
            )
        }
    }
}
pub struct DurationType(iso8601_duration::Duration);
#[automatically_derived]
impl ::core::clone::Clone for DurationType {
    #[inline]
    fn clone(&self) -> DurationType {
        DurationType(::core::clone::Clone::clone(&self.0))
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for DurationType {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "DurationType", &&self.0)
    }
}
impl DurationType {
    pub fn new(duration: iso8601_duration::Duration) -> Self {
        DurationType(duration)
    }
}
impl std::ops::Deref for DurationType {
    type Target = iso8601_duration::Duration;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Serialize for DurationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}
impl<'de> de::Deserialize<'de> for DurationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;
        let duration: iso8601_duration::Duration = str
            .parse()
            .map_err(|e: iso8601_duration::ParseDurationError| serde::de::Error::custom(
                e.input,
            ))?;
        Ok(DurationType(duration))
    }
}
