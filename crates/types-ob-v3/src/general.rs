use serde::{Deserialize, Serialize};
use types_common::{GenPaths, SchemaList};

#[doc = "JSON-LD Context. Either a URI with the context definition or a Map with a local context definition MUST be supplied."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
#[serde(untagged)]
pub enum Context {
    Map(serde_json::Map<String, serde_json::Value>),
    String(String),
}
impl From<&Context> for Context {
    fn from(value: &Context) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for Context {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self::Map(value)
    }
}

impl From<String> for Context {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Context {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

#[doc = "Metadata about images that represent assertions, achieve or profiles. These properties can typically be represented as just the id string of the image, but using a fleshed-out document allows for including captions and other applicable metadata."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
#[serde(deny_unknown_fields)]
pub struct Image {
    #[doc = "The URI or Data URI of the image."]
    pub id: String,
    #[doc = "MUST be the IRI 'Image'."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The caption for the image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
}
impl From<&Image> for Image {
    fn from(value: &Image) -> Self {
        value.clone()
    }
}

#[doc = "The information in RefreshService is used to refresh the verifiable credential."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
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

#[doc = "Terms of use can be utilized by an issuer or a holder to communicate the terms under which a verifiable credential or verifiable presentation was issued"]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct ImageBuilder {
    caption: Result<Option<String>, String>,
    id: Result<String, String>,
    type_: Result<String, String>,
}
impl Default for ImageBuilder {
    fn default() -> Self {
        Self {
            caption: Ok(Default::default()),
            id: Err("no value supplied for id".to_string()),
            type_: Err("no value supplied for type_".to_string()),
        }
    }
}
impl ImageBuilder {
    pub fn caption<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<String>,
        T::Error: std::fmt::Display,
    {
        self.caption = value
            .try_into()
            .map(Some)
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
impl std::convert::TryFrom<ImageBuilder> for Image {
    type Error = String;
    fn try_from(value: ImageBuilder) -> Result<Self, String> {
        Ok(Self {
            caption: value.caption?,
            id: value.id?,
            type_: value.type_?,
        })
    }
}
impl std::convert::TryFrom<ImageBuilder> for Option<Image> {
    type Error = String;
    fn try_from(value: ImageBuilder) -> Result<Self, String> {
        Ok(Some(Image {
            caption: value.caption?,
            id: value.id?,
            type_: value.type_?,
        }))
    }
}
impl From<Image> for ImageBuilder {
    fn from(value: Image) -> Self {
        Self {
            caption: Ok(value.caption),
            id: Ok(value.id),
            type_: Ok(value.type_),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RefreshServiceBuilder {
    id: Result<String, String>,
    type_: Result<String, String>,
}
impl Default for RefreshServiceBuilder {
    fn default() -> Self {
        Self {
            id: Err("no value supplied for id".to_string()),
            type_: Err("no value supplied for type_".to_string()),
        }
    }
}
impl RefreshServiceBuilder {
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
impl std::convert::TryFrom<RefreshServiceBuilder> for RefreshService {
    type Error = String;
    fn try_from(value: RefreshServiceBuilder) -> Result<Self, String> {
        Ok(Self {
            id: value.id?,
            type_: value.type_?,
        })
    }
}
impl std::convert::TryFrom<RefreshServiceBuilder> for Option<RefreshService> {
    type Error = String;
    fn try_from(value: RefreshServiceBuilder) -> Result<Self, String> {
        Ok(Some(RefreshService {
            id: value.id?,
            type_: value.type_?,
        }))
    }
}
impl From<RefreshService> for RefreshServiceBuilder {
    fn from(value: RefreshService) -> Self {
        Self {
            id: Ok(value.id),
            type_: Ok(value.type_),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TermsOfUseBuilder {
    id: Result<Option<String>, String>,
    type_: Result<String, String>,
}
impl Default for TermsOfUseBuilder {
    fn default() -> Self {
        Self {
            id: Ok(Default::default()),
            type_: Err("no value supplied for type_".to_string()),
        }
    }
}
impl TermsOfUseBuilder {
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
impl std::convert::TryFrom<TermsOfUseBuilder> for TermsOfUse {
    type Error = String;
    fn try_from(value: TermsOfUseBuilder) -> Result<Self, String> {
        Ok(Self {
            id: value.id?,
            type_: value.type_?,
        })
    }
}
impl From<TermsOfUse> for TermsOfUseBuilder {
    fn from(value: TermsOfUse) -> Self {
        Self {
            id: Ok(value.id),
            type_: Ok(value.type_),
        }
    }
}
