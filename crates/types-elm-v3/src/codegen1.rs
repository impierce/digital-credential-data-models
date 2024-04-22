use serde::{Deserialize, Serialize};
/// Error types.
pub mod error {
    /// Error from a TryFrom or FromStr implementation.
    pub struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///GeneratedSchemaForRoot
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Generated schema for Root",
///  "type": "object",
///  "required": [
///    "email"
///  ],
///  "properties": {
///    "email": {
///      "type": "string",
///      "anyOf": [
///        {
///          "format": "email"
///        },
///        {
///          "format": "uri",
///          "pattern": "^mailto:[^@]*[^\\.]@[^\\.]($|[^@]*[^\\.]$)"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GeneratedSchemaForRoot {
    pub email: GeneratedSchemaForRootEmail,
}
impl From<&GeneratedSchemaForRoot> for GeneratedSchemaForRoot {
    fn from(value: &GeneratedSchemaForRoot) -> Self {
        value.clone()
    }
}
impl GeneratedSchemaForRoot {
    pub fn builder() -> builder::GeneratedSchemaForRoot {
        Default::default()
    }
}
///GeneratedSchemaForRootEmail
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "anyOf": [
///    {
///      "format": "email"
///    },
///    {
///      "format": "uri",
///      "pattern": "^mailto:[^@]*[^\\.]@[^\\.]($|[^@]*[^\\.]$)"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GeneratedSchemaForRootEmail {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<Email>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<String>,
}
impl From<&GeneratedSchemaForRootEmail> for GeneratedSchemaForRootEmail {
    fn from(value: &GeneratedSchemaForRootEmail) -> Self {
        value.clone()
    }
}
impl GeneratedSchemaForRootEmail {
    pub fn builder() -> builder::GeneratedSchemaForRootEmail {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct GeneratedSchemaForRoot {
        email: Result<super::GeneratedSchemaForRootEmail, String>,
    }
    impl Default for GeneratedSchemaForRoot {
        fn default() -> Self {
            Self {
                email: Err("no value supplied for email".to_string()),
            }
        }
    }
    impl GeneratedSchemaForRoot {
        pub fn email<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::GeneratedSchemaForRootEmail>,
            T::Error: std::fmt::Display,
        {
            self.email = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for email: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<GeneratedSchemaForRoot>
    for super::GeneratedSchemaForRoot {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GeneratedSchemaForRoot,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self { email: value.email? })
        }
    }
    impl From<super::GeneratedSchemaForRoot> for GeneratedSchemaForRoot {
        fn from(value: super::GeneratedSchemaForRoot) -> Self {
            Self { email: Ok(value.email) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GeneratedSchemaForRootEmail {
        subtype_0: Result<Option<Email>, String>,
        subtype_1: Result<Option<String>, String>,
    }
    impl Default for GeneratedSchemaForRootEmail {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl GeneratedSchemaForRootEmail {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Email>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_0: {}", e)
                });
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_1: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<GeneratedSchemaForRootEmail>
    for super::GeneratedSchemaForRootEmail {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GeneratedSchemaForRootEmail,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl From<super::GeneratedSchemaForRootEmail> for GeneratedSchemaForRootEmail {
        fn from(value: super::GeneratedSchemaForRootEmail) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
}
