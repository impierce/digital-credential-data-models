use std::{fmt, ops::Deref};

use serde::de::{Error, Unexpected};
use serde::Serialize;
use serde::{de::DeserializeOwned, Deserialize, Deserializer};

pub use macro_derive::{EnumDeserialize, TagType};
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum ObjectOrVector<T> {
    Object(Box<T>),
    Vector(Vec<T>),
}

impl<'de, T: DeserializeOwned + fmt::Debug> Deserialize<'de> for ObjectOrVector<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let serde_value = serde_json::Value::deserialize(deserializer)?;

        if let serde_json::Value::Array(arr) = serde_value {
            let mut out = vec![];

            for item in arr {
                let result = serde_json::from_value(item);

                match result {
                    Ok(item) => out.push(item),
                    Err(err) => {
                        eprintln!("{}", err);
                        return Err(Error::custom(err));
                    }
                };
            }

            Ok(Self::Vector(out))
        } else {
            let result = serde_json::from_value(serde_value.clone());

            match result {
                Ok(result) => Ok(Self::Object(Box::new(result))),
                Err(err) => {
                    eprintln!("{}", err);
                    Err(Error::custom(err))
                }
            }
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(try_from = "String")]
pub struct Email(pub String);

impl Deref for Email {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let email = String::deserialize(deserializer)?;

        let email_regex = regex::Regex::new("^[\\w-\\.]+@([\\w-]+\\.)+[\\w-]{2,4}$").unwrap();
        let email_uri_regex = regex::Regex::new("^mailto:[^@]*[^\\.]@[^\\.]($|[^@]*[^\\.]$)").unwrap();

        let valid = email_regex.is_match(&email) || email_uri_regex.is_match(&email);

        if valid {
            Ok(Email(email))
        } else {
            Err(Error::invalid_value(Unexpected::Str(&email), &"A valid email format"))
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct PositiveInteger(pub u32);
impl std::ops::Deref for PositiveInteger {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for PositiveInteger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let number: i64 = i64::deserialize(deserializer)?;

        if 0 <= number {
            Ok(PositiveInteger(number as u32))
        } else {
            Err(D::Error::invalid_value(
                Unexpected::Signed(number),
                &"A positive integer",
            ))
        }
    }
}

///DurationType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "format": "duration"
///}
/// ```
/// </details>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DurationType(pub OffsetDateTime);

impl std::ops::Deref for DurationType {
    type Target = OffsetDateTime;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for DurationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let duration = time::serde::iso8601::deserialize(deserializer)?;

        Ok(DurationType(duration))
    }
}
