use std::{fmt, ops::Deref};

use regress::Regex;
use serde::de::{self, Error};
use serde::ser::Serialize;
use serde::{de::DeserializeOwned, Deserialize, Deserializer};

#[derive(Clone, Debug)]
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

impl<T: Serialize> Serialize for ObjectOrVector<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Vector(vector) => vector.serialize(serializer),
            Self::Object(obj) => obj.serialize(serializer),
        }
    }
}

pub trait EnumDeserialize
where
    Self: Sized,
{
    fn variants() -> &'static [&'static str];

    fn into_enum(key: &str, value: serde_json::Value) -> Result<Self, serde_json::Error>;
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

//impl TryFrom<String> for Email {
//type Error = String;

//fn try_from(email: String) -> Result<Self, Self::Error> {
//let email_regex = regress::Regex::new("^[\\w-\\.]+@([\\w-]+\\.)+[\\w-]{2,4}$").unwrap();
//let email_uri_regex = Regex::new("^mailto:[^@]*[^\\.]@[^\\.]($|[^@]*[^\\.]$)").unwrap();

//let valid = email_regex.find(&email).is_some() || email_uri_regex.find(&email).is_some();

//if valid {
//Ok(Email(email))
//} else {
//Err(format!("Email format is not valid: {email:?}"))
//}
//}
//}

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
