use regress::Regex;
use serde::{
    de::{DeserializeOwned, Error, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::fmt;

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum ObjectOrVector<T> {
    Object(T),
    Vector(Vec<T>),
}

/// Very large structs use box construct to prevent too much memory from being copied.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BoxObjectOrVector<T> {
    /// Vector check needs to be before Object since T can be a vec too.
    Vector(Vec<T>),
    Object(Box<T>),
}

impl<'de, T: DeserializeOwned + fmt::Debug> Deserialize<'de> for ObjectOrVector<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let serde_value: serde_json::Value = Deserialize::deserialize(deserializer)?;

        match serde_value {
            serde_json::Value::Array(arr) => {
                let arr: Vec<T> = arr
                    .into_iter()
                    .map(|item| serde_json::from_value(item).expect("Item not good"))
                    .collect();

                Ok(ObjectOrVector::Vector(arr))
            }
            _ => match serde_json::from_value::<T>(serde_value) {
                Ok(val) => Ok(ObjectOrVector::Object(val)),
                Err(e) => Err(Error::custom(e.to_string())),
            },
        }
    }
}

//fn deserialize_json_string<'de, D>(deserializer: D) -> Result<ActualData, D::Error>
//where
//D: Deserializer<'de>,
//{
//let s: &str = Deserialize::deserialize(deserializer)?;
//serde_json::from_str(s).map_err(de::Error::custom)
//}

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
