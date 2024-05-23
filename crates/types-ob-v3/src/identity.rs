use serde::{Deserialize, Serialize};
use types_common::{GenPaths, SchemaList};

#[doc = "No description supplied."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
#[serde(deny_unknown_fields)]
pub struct IdentifierEntry {
    #[doc = "The value of the type property MUST be an unordered set. One of the items MUST be the IRI 'IdentifierEntry'."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "An identifier."]
    pub identifier: String,
    #[doc = "The identifier type."]
    #[serde(rename = "identifierType")]
    pub identifier_type: IdentifierType,
}
impl From<&IdentifierEntry> for IdentifierEntry {
    fn from(value: &IdentifierEntry) -> Self {
        value.clone()
    }
}

#[doc = "The identifier type."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
#[serde(untagged)]
pub enum IdentifierType {
    Enum(IdentifierTypeEnum),
    String(IdentifierTypeString),
}

impl From<&IdentifierType> for IdentifierType {
    fn from(value: &IdentifierType) -> Self {
        value.clone()
    }
}
impl From<IdentifierTypeEnum> for IdentifierType {
    fn from(value: IdentifierTypeEnum) -> Self {
        Self::Enum(value)
    }
}
impl From<IdentifierTypeString> for IdentifierType {
    fn from(value: IdentifierTypeString) -> Self {
        Self::String(value)
    }
}

impl std::str::FromStr for IdentifierType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        IdentifierTypeEnum::from_str(value)
            .map(Self::Enum)
            .or_else(|_| IdentifierTypeString::from_str(value).map(Self::String))
            .map_err(|_| "invalid value")
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, GenPaths)]
pub enum IdentifierTypeEnum {
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "sourcedId")]
    SourcedId,
    #[serde(rename = "systemId")]
    SystemId,
    #[serde(rename = "productId")]
    ProductId,
    #[serde(rename = "userName")]
    UserName,
    #[serde(rename = "accountId")]
    AccountId,
    #[serde(rename = "emailAddress")]
    EmailAddress,
    #[serde(rename = "nationalIdentityNumber")]
    NationalIdentityNumber,
    #[serde(rename = "isbn")]
    Isbn,
    #[serde(rename = "issn")]
    Issn,
    #[serde(rename = "lisSourcedId")]
    LisSourcedId,
    #[serde(rename = "oneRosterSourcedId")]
    OneRosterSourcedId,
    #[serde(rename = "sisSourcedId")]
    SisSourcedId,
    #[serde(rename = "ltiContextId")]
    LtiContextId,
    #[serde(rename = "ltiDeploymentId")]
    LtiDeploymentId,
    #[serde(rename = "ltiToolId")]
    LtiToolId,
    #[serde(rename = "ltiPlatformId")]
    LtiPlatformId,
    #[serde(rename = "ltiUserId")]
    LtiUserId,
    #[serde(rename = "identifier")]
    Identifier,
}
impl From<&IdentifierTypeEnum> for IdentifierTypeEnum {
    fn from(value: &IdentifierTypeEnum) -> Self {
        *value
    }
}
impl ToString for IdentifierTypeEnum {
    fn to_string(&self) -> String {
        match *self {
            Self::Name => "name".to_string(),
            Self::SourcedId => "sourcedId".to_string(),
            Self::SystemId => "systemId".to_string(),
            Self::ProductId => "productId".to_string(),
            Self::UserName => "userName".to_string(),
            Self::AccountId => "accountId".to_string(),
            Self::EmailAddress => "emailAddress".to_string(),
            Self::NationalIdentityNumber => "nationalIdentityNumber".to_string(),
            Self::Isbn => "isbn".to_string(),
            Self::Issn => "issn".to_string(),
            Self::LisSourcedId => "lisSourcedId".to_string(),
            Self::OneRosterSourcedId => "oneRosterSourcedId".to_string(),
            Self::SisSourcedId => "sisSourcedId".to_string(),
            Self::LtiContextId => "ltiContextId".to_string(),
            Self::LtiDeploymentId => "ltiDeploymentId".to_string(),
            Self::LtiToolId => "ltiToolId".to_string(),
            Self::LtiPlatformId => "ltiPlatformId".to_string(),
            Self::LtiUserId => "ltiUserId".to_string(),
            Self::Identifier => "identifier".to_string(),
        }
    }
}
impl std::str::FromStr for IdentifierTypeEnum {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "name" => Ok(Self::Name),
            "sourcedId" => Ok(Self::SourcedId),
            "systemId" => Ok(Self::SystemId),
            "productId" => Ok(Self::ProductId),
            "userName" => Ok(Self::UserName),
            "accountId" => Ok(Self::AccountId),
            "emailAddress" => Ok(Self::EmailAddress),
            "nationalIdentityNumber" => Ok(Self::NationalIdentityNumber),
            "isbn" => Ok(Self::Isbn),
            "issn" => Ok(Self::Issn),
            "lisSourcedId" => Ok(Self::LisSourcedId),
            "oneRosterSourcedId" => Ok(Self::OneRosterSourcedId),
            "sisSourcedId" => Ok(Self::SisSourcedId),
            "ltiContextId" => Ok(Self::LtiContextId),
            "ltiDeploymentId" => Ok(Self::LtiDeploymentId),
            "ltiToolId" => Ok(Self::LtiToolId),
            "ltiPlatformId" => Ok(Self::LtiPlatformId),
            "ltiUserId" => Ok(Self::LtiUserId),
            "identifier" => Ok(Self::Identifier),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for IdentifierTypeEnum {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IdentifierTypeEnum {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IdentifierTypeEnum {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, GenPaths)]
pub struct IdentifierTypeString(String);
impl std::ops::Deref for IdentifierTypeString {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IdentifierTypeString> for String {
    fn from(value: IdentifierTypeString) -> Self {
        value.0
    }
}
impl From<&IdentifierTypeString> for IdentifierTypeString {
    fn from(value: &IdentifierTypeString) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IdentifierTypeString {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regex::Regex::new("(ext:)[a-z|A-Z|0-9|.|-|_]+").unwrap().is_match(value) {
            Ok(Self(value.to_string()))
        } else {
            Err("doesn't match pattern \"(ext:)[a-z|A-Z|0-9|.|-|_]+\"")
        }
    }
}
impl std::convert::TryFrom<&str> for IdentifierTypeString {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IdentifierTypeString {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IdentifierTypeString {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IdentifierTypeString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "A collection of information about the recipient of an achievement."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
#[serde(deny_unknown_fields)]
pub struct IdentityObject {
    #[doc = "MUST be the IRI 'IdentityObject'."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Whether or not the `identityHash` value is hashed."]
    pub hashed: bool,
    #[doc = "Either the IdentityHash of the identity or the plaintext value. If it's possible that the plaintext transmission and storage of the identity value would leak personally identifiable information where there is an expectation of privacy, it is strongly recommended that an IdentityHash be used."]
    #[serde(rename = "identityHash")]
    pub identity_hash: String,
    #[doc = "The identity type."]
    #[serde(rename = "identityType")]
    pub identity_object_type: IdentityObjectType,
    #[doc = "If the `identityHash` is hashed, this should contain the string used to salt the hash. If this value is not provided, it should be assumed that the hash was not salted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<String>,
}
impl From<&IdentityObject> for IdentityObject {
    fn from(value: &IdentityObject) -> Self {
        value.clone()
    }
}

#[doc = "The identity type."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
#[serde(untagged)]
pub enum IdentityObjectType {
    Enum(IdentityObjectTypeEnum),
    String(IdentityObjectTypeString),
}
impl From<&IdentityObjectType> for IdentityObjectType {
    fn from(value: &IdentityObjectType) -> Self {
        value.clone()
    }
}
impl From<IdentityObjectTypeEnum> for IdentityObjectType {
    fn from(value: IdentityObjectTypeEnum) -> Self {
        Self::Enum(value)
    }
}
impl From<IdentityObjectTypeString> for IdentityObjectType {
    fn from(value: IdentityObjectTypeString) -> Self {
        Self::String(value)
    }
}

impl std::str::FromStr for IdentityObjectType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        IdentityObjectTypeEnum::from_str(value)
            .map(Self::Enum)
            .or_else(|_| IdentityObjectTypeString::from_str(value).map(Self::String))
            .map_err(|_| "invalid value")
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, GenPaths)]
pub enum IdentityObjectTypeEnum {
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "sourcedId")]
    SourcedId,
    #[serde(rename = "systemId")]
    SystemId,
    #[serde(rename = "productId")]
    ProductId,
    #[serde(rename = "userName")]
    UserName,
    #[serde(rename = "accountId")]
    AccountId,
    #[serde(rename = "emailAddress")]
    EmailAddress,
    #[serde(rename = "nationalIdentityNumber")]
    NationalIdentityNumber,
    #[serde(rename = "isbn")]
    Isbn,
    #[serde(rename = "issn")]
    Issn,
    #[serde(rename = "lisSourcedId")]
    LisSourcedId,
    #[serde(rename = "oneRosterSourcedId")]
    OneRosterSourcedId,
    #[serde(rename = "sisSourcedId")]
    SisSourcedId,
    #[serde(rename = "ltiContextId")]
    LtiContextId,
    #[serde(rename = "ltiDeploymentId")]
    LtiDeploymentId,
    #[serde(rename = "ltiToolId")]
    LtiToolId,
    #[serde(rename = "ltiPlatformId")]
    LtiPlatformId,
    #[serde(rename = "ltiUserId")]
    LtiUserId,
    #[serde(rename = "identifier")]
    Identifier,
}
impl From<&IdentityObjectTypeEnum> for IdentityObjectTypeEnum {
    fn from(value: &IdentityObjectTypeEnum) -> Self {
        *value
    }
}
impl ToString for IdentityObjectTypeEnum {
    fn to_string(&self) -> String {
        match *self {
            Self::Name => "name".to_string(),
            Self::SourcedId => "sourcedId".to_string(),
            Self::SystemId => "systemId".to_string(),
            Self::ProductId => "productId".to_string(),
            Self::UserName => "userName".to_string(),
            Self::AccountId => "accountId".to_string(),
            Self::EmailAddress => "emailAddress".to_string(),
            Self::NationalIdentityNumber => "nationalIdentityNumber".to_string(),
            Self::Isbn => "isbn".to_string(),
            Self::Issn => "issn".to_string(),
            Self::LisSourcedId => "lisSourcedId".to_string(),
            Self::OneRosterSourcedId => "oneRosterSourcedId".to_string(),
            Self::SisSourcedId => "sisSourcedId".to_string(),
            Self::LtiContextId => "ltiContextId".to_string(),
            Self::LtiDeploymentId => "ltiDeploymentId".to_string(),
            Self::LtiToolId => "ltiToolId".to_string(),
            Self::LtiPlatformId => "ltiPlatformId".to_string(),
            Self::LtiUserId => "ltiUserId".to_string(),
            Self::Identifier => "identifier".to_string(),
        }
    }
}
impl std::str::FromStr for IdentityObjectTypeEnum {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "name" => Ok(Self::Name),
            "sourcedId" => Ok(Self::SourcedId),
            "systemId" => Ok(Self::SystemId),
            "productId" => Ok(Self::ProductId),
            "userName" => Ok(Self::UserName),
            "accountId" => Ok(Self::AccountId),
            "emailAddress" => Ok(Self::EmailAddress),
            "nationalIdentityNumber" => Ok(Self::NationalIdentityNumber),
            "isbn" => Ok(Self::Isbn),
            "issn" => Ok(Self::Issn),
            "lisSourcedId" => Ok(Self::LisSourcedId),
            "oneRosterSourcedId" => Ok(Self::OneRosterSourcedId),
            "sisSourcedId" => Ok(Self::SisSourcedId),
            "ltiContextId" => Ok(Self::LtiContextId),
            "ltiDeploymentId" => Ok(Self::LtiDeploymentId),
            "ltiToolId" => Ok(Self::LtiToolId),
            "ltiPlatformId" => Ok(Self::LtiPlatformId),
            "ltiUserId" => Ok(Self::LtiUserId),
            "identifier" => Ok(Self::Identifier),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for IdentityObjectTypeEnum {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IdentityObjectTypeEnum {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IdentityObjectTypeEnum {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, GenPaths)]
pub struct IdentityObjectTypeString(String);
impl std::ops::Deref for IdentityObjectTypeString {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IdentityObjectTypeString> for String {
    fn from(value: IdentityObjectTypeString) -> Self {
        value.0
    }
}
impl From<&IdentityObjectTypeString> for IdentityObjectTypeString {
    fn from(value: &IdentityObjectTypeString) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IdentityObjectTypeString {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regex::Regex::new("(ext:)[a-z|A-Z|0-9|.|-|_]+").unwrap().is_match(value) {
            Ok(Self(value.to_string()))
        } else {
            Err("doesn't match pattern \"(ext:)[a-z|A-Z|0-9|.|-|_]+\"")
        }
    }
}
impl std::convert::TryFrom<&str> for IdentityObjectTypeString {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IdentityObjectTypeString {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IdentityObjectTypeString {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IdentityObjectTypeString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IdentifierEntryBuilder {
    identifier: Result<String, String>,
    identifier_type: Result<IdentifierType, String>,
    type_: Result<String, String>,
}
impl Default for IdentifierEntryBuilder {
    fn default() -> Self {
        Self {
            identifier: Err("no value supplied for identifier".to_string()),
            identifier_type: Err("no value supplied for identifier_type".to_string()),
            type_: Err("no value supplied for type_".to_string()),
        }
    }
}
impl IdentifierEntryBuilder {
    pub fn identifier<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<String>,
        T::Error: std::fmt::Display,
    {
        self.identifier = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for identifier: {}", e));
        self
    }
    pub fn identifier_type<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<IdentifierType>,
        T::Error: std::fmt::Display,
    {
        self.identifier_type = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for identifier_type: {}", e));
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
impl std::convert::TryFrom<IdentifierEntryBuilder> for IdentifierEntry {
    type Error = String;
    fn try_from(value: IdentifierEntryBuilder) -> Result<Self, String> {
        Ok(Self {
            identifier: value.identifier?,
            identifier_type: value.identifier_type?,
            type_: value.type_?,
        })
    }
}
impl From<IdentifierEntry> for IdentifierEntryBuilder {
    fn from(value: IdentifierEntry) -> Self {
        Self {
            identifier: Ok(value.identifier),
            identifier_type: Ok(value.identifier_type),
            type_: Ok(value.type_),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct IdentityObjectBuilder {
    hashed: Result<bool, String>,
    identity_hash: Result<String, String>,
    identity_object_type: Result<IdentityObjectType, String>,
    salt: Result<Option<String>, String>,
    type_: Result<String, String>,
}
impl Default for IdentityObjectBuilder {
    fn default() -> Self {
        Self {
            hashed: Err("no value supplied for hashed".to_string()),
            identity_hash: Err("no value supplied for identity_hash".to_string()),
            identity_object_type: Err("no value supplied for identity_object_type".to_string()),
            salt: Ok(Default::default()),
            type_: Err("no value supplied for type_".to_string()),
        }
    }
}
impl IdentityObjectBuilder {
    pub fn hashed<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<bool>,
        T::Error: std::fmt::Display,
    {
        self.hashed = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for hashed: {}", e));
        self
    }
    pub fn identity_hash<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<String>,
        T::Error: std::fmt::Display,
    {
        self.identity_hash = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for identity_hash: {}", e));
        self
    }
    pub fn identity_object_type<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<IdentityObjectType>,
        T::Error: std::fmt::Display,
    {
        self.identity_object_type = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for identity_object_type: {}", e));
        self
    }
    pub fn salt<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.salt = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for salt: {}", e));
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
impl std::convert::TryFrom<IdentityObjectBuilder> for IdentityObject {
    type Error = String;
    fn try_from(value: IdentityObjectBuilder) -> Result<Self, String> {
        Ok(Self {
            hashed: value.hashed?,
            identity_hash: value.identity_hash?,
            identity_object_type: value.identity_object_type?,
            salt: value.salt?,
            type_: value.type_?,
        })
    }
}
impl From<IdentityObject> for IdentityObjectBuilder {
    fn from(value: IdentityObject) -> Self {
        Self {
            hashed: Ok(value.hashed),
            identity_hash: Ok(value.identity_hash),
            identity_object_type: Ok(value.identity_object_type),
            salt: Ok(value.salt),
            type_: Ok(value.type_),
        }
    }
}
