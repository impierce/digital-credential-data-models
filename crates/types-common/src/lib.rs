use regress::Regex;
use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ObjectOrVector<T> {
    Object(T),
    Vector(Vec<T>),
}

/// Very large structs use box construct to prevent too much memory from being copied.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BoxObjectOrVector<T> {
    Object(Box<T>),
    Vector(Vec<T>),
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
