use std::{fmt, ops::Deref};

use regress::Regex;
use serde::{
    de::{DeserializeOwned, Error, Visitor},
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
        let value = serde_json::Value::deserialize(deserializer)?;
        let email: Email = serde_json::from_value(value).map_err(|e| Error::custom(e))?;

        let email_str = email.as_ref();

        let email_regex = regress::Regex::new("^[\\w-\\.]+@([\\w-]+\\.)+[\\w-]{2,4}$").unwrap();
        let email_uri_regex = Regex::new("^mailto:[^@]*[^\\.]@[^\\.]($|[^@]*[^\\.]$)").unwrap();

        let valid = email_regex.find(email_str).is_some() || email_uri_regex.find(&email_str).is_some();

        if valid {
            Ok(email)
        } else {
            Err(Error::custom(format!("Email format is not valid: {email_str:?}")))
        }
    }
}
