use serde::{Deserialize, Serialize};

#[doc = "JSON-LD Context. Either a URI with the context definition or a Map with a local context definition MUST be supplied."]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Context {
    MapStringJsonValue(serde_json::Map<String, serde_json::Value>),
    SingleString(String),
}
impl From<&Context> for Context {
    fn from(value: &Context) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for Context {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self::MapStringJsonValue(value)
    }
}

#[doc = "Metadata about images that represent assertions, achieve or profiles. These properties can typically be represented as just the id string of the image, but using a fleshed-out document allows for including captions and other applicable metadata."]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Image {
    #[doc = "The caption for the image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[doc = "The URI or Data URI of the image."]
    pub id: String,
    #[doc = "MUST be the IRI 'Image'."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&Image> for Image {
    fn from(value: &Image) -> Self {
        value.clone()
    }
}
impl Image {
    pub fn builder() -> builder::Image {
        builder::Image::default()
    }
}

#[doc = "The information in RefreshService is used to refresh the verifiable credential."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RefreshService {
    #[doc = "The value MUST be the URL of the issuer's refresh service."]
    pub id: String,
    #[doc = "The name of the refresh service method."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&RefreshService> for RefreshService {
    fn from(value: &RefreshService) -> Self {
        value.clone()
    }
}
impl RefreshService {
    pub fn builder() -> builder::RefreshService {
        builder::RefreshService::default()
    }
}

#[doc = "Terms of use can be utilized by an issuer or a holder to communicate the terms under which a verifiable credential or verifiable presentation was issued"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TermsOfUse {
    #[doc = "The value MUST be a URI identifying the term of use."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The value MUST identify the type of the terms of use."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&TermsOfUse> for TermsOfUse {
    fn from(value: &TermsOfUse) -> Self {
        value.clone()
    }
}
impl TermsOfUse {
    pub fn builder() -> builder::TermsOfUse {
        builder::TermsOfUse::default()
    }
}

pub mod builder {

    #[derive(Clone, Debug)]
    pub struct Image {
        caption: Result<Option<String>, String>,
        id: Result<String, String>,
        type_: Result<String, String>,
    }
    impl Default for Image {
        fn default() -> Self {
            Self {
                caption: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Image {
        pub fn caption<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.caption = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for caption: {}", e));
            self
        }
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Image> for super::Image {
        type Error = String;
        fn try_from(value: Image) -> Result<Self, String> {
            Ok(Self {
                caption: value.caption?,
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Image> for Image {
        fn from(value: super::Image) -> Self {
            Self {
                caption: Ok(value.caption),
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct RefreshService {
        id: Result<String, String>,
        type_: Result<String, String>,
    }
    impl Default for RefreshService {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RefreshService {
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<RefreshService> for super::RefreshService {
        type Error = String;
        fn try_from(value: RefreshService) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::RefreshService> for RefreshService {
        fn from(value: super::RefreshService) -> Self {
            Self {
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct TermsOfUse {
        id: Result<Option<String>, String>,
        type_: Result<String, String>,
    }
    impl Default for TermsOfUse {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TermsOfUse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<TermsOfUse> for super::TermsOfUse {
        type Error = String;
        fn try_from(value: TermsOfUse) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::TermsOfUse> for TermsOfUse {
        fn from(value: super::TermsOfUse) -> Self {
            Self {
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
}
