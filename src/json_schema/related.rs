use serde::{Deserialize, Serialize};

#[doc = "Identifies a related achievement."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Related {
    #[doc = "The related achievement."]
    pub id: String,
    #[doc = "The language of the related achievement."]
    #[serde(rename = "@language", default, skip_serializing_if = "Option::is_none")]
    pub language: Option<RelatedLanguage>,
    #[serde(rename = "type")]
    pub type_: RelatedType,
    #[doc = "The version of the related achievement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl From<&Related> for Related {
    fn from(value: &Related) -> Self {
        value.clone()
    }
}

impl Related {
    pub fn builder() -> builder::Related {
        builder::Related::default()
    }
}

#[doc = "The language of the related achievement."]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RelatedLanguage(String);
impl std::ops::Deref for RelatedLanguage {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl From<RelatedLanguage> for String {
    fn from(value: RelatedLanguage) -> Self {
        value.0
    }
}

impl From<&RelatedLanguage> for RelatedLanguage {
    fn from(value: &RelatedLanguage) -> Self {
        value.clone()
    }
}

impl std::str::FromStr for RelatedLanguage {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new("^[a-z]{2,4}(-[A-Z][a-z]{3})?(-([A-Z]{2}|[0-9]{3}))?$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err(
                "doesn't match pattern \"^[a-z]{2,4}(-[A-Z][a-z]{3})?(-([A-Z]{2}|[0-9]{3}))?$\"",
            );
        }
        Ok(Self(value.to_string()))
    }
}

impl std::convert::TryFrom<&str> for RelatedLanguage {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for RelatedLanguage {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for RelatedLanguage {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for RelatedLanguage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RelatedType {
    SingleString(String),
    VecString(Vec<String>),
}

impl From<&RelatedType> for RelatedType {
    fn from(value: &RelatedType) -> Self {
        value.clone()
    }
}

impl From<Vec<String>> for RelatedType {
    fn from(value: Vec<String>) -> Self {
        Self::VecString(value)
    }
}

pub mod builder {
    
    #[derive(Clone, Debug)]
    pub struct Related {
        id: Result<String, String>,
        language: Result<Option<super::RelatedLanguage>, String>,
        type_: Result<super::RelatedType, String>,
        version: Result<Option<String>, String>,
    }
    impl Default for Related {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                language: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                version: Ok(Default::default()),
            }
        }
    }
    impl Related {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::RelatedLanguage>>,
            T::Error: std::fmt::Display,
        {
            self.language = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for language: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::RelatedType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Related> for super::Related {
        type Error = String;
        fn try_from(value: Related) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                language: value.language?,
                type_: value.type_?,
                version: value.version?,
            })
        }
    }
    impl From<super::Related> for Related {
        fn from(value: super::Related) -> Self {
            Self {
                id: Ok(value.id),
                language: Ok(value.language),
                type_: Ok(value.type_),
                version: Ok(value.version),
            }
        }
    }
}
