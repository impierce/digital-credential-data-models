use std::fmt;

use regress::Regex;
use serde::{
    de::{DeserializeOwned, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Clone, Debug, Serialize)]
pub enum ObjectOrVector<T> {
    Object(Box<T>),
    Vector(Vec<T>),
}

impl<'de, T: DeserializeOwned + fmt::Debug> Deserialize<'de> for ObjectOrVector<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let deserializer = serde_stacker::Deserializer::new(deserializer);
        let serde_value = serde_json::Value::deserialize(deserializer)?;

        if let serde_json::Value::Array(arr) = serde_value {
            let arr: Vec<T> = arr
                .into_iter()
                .map(|item| serde_json::from_value(item).unwrap())
                .collect();

            Ok(ObjectOrVector::Vector(arr))
        } else {
            Ok(ObjectOrVector::Object(Box::new(
                serde_json::from_value(serde_value).unwrap(),
            )))
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Email(pub String);

struct EmailVisitor;

impl<'de> Visitor<'de> for EmailVisitor {
    type Value = Email;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid email format")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let email_regex = regress::Regex::new("^[\\w-\\.]+@([\\w-]+\\.)+[\\w-]{2,4}$").unwrap();
        let email_uri_regex = Regex::new("^mailto:[^@]*[^\\.]@[^\\.]($|[^@]*[^\\.]$)").unwrap();

        if email_regex.find(v).is_some() || email_uri_regex.find(v).is_some() {
            Ok(Email(v.to_string()))
        } else {
            Err(E::custom(format!("Email format is not valid: {v}")))
        }
    }
}

impl<'de> Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(EmailVisitor)
    }
}
