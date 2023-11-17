use serde::{Deserialize, Serialize};

#[doc = "No description supplied."]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IdentifierEntry {
    #[doc = "An identifier."]
    pub identifier: String,
    #[doc = "The identifier type."]
    #[serde(rename = "identifierType")]
    pub identifier_type: IdentifierEntryIdentifierType,
    #[doc = "The value of the type property MUST be an unordered set. One of the items MUST be the IRI 'IdentifierEntry'."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&IdentifierEntry> for IdentifierEntry {
    fn from(value: &IdentifierEntry) -> Self {
        value.clone()
    }
}
impl IdentifierEntry {
    pub fn builder() -> builder::IdentifierEntry {
        builder::IdentifierEntry::default()
    }
}
#[doc = "The identifier type."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdentifierEntryIdentifierType {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<IdentifierEntryIdentifierTypeSubtype0>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<IdentifierEntryIdentifierTypeSubtype1>,
}
impl From<&IdentifierEntryIdentifierType> for IdentifierEntryIdentifierType {
    fn from(value: &IdentifierEntryIdentifierType) -> Self {
        value.clone()
    }
}
impl IdentifierEntryIdentifierType {
    pub fn builder() -> builder::IdentifierEntryIdentifierType {
        builder::IdentifierEntryIdentifierType::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IdentifierEntryIdentifierTypeSubtype0 {
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
impl From<&IdentifierEntryIdentifierTypeSubtype0> for IdentifierEntryIdentifierTypeSubtype0 {
    fn from(value: &IdentifierEntryIdentifierTypeSubtype0) -> Self {
        value.clone()
    }
}
impl ToString for IdentifierEntryIdentifierTypeSubtype0 {
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
impl std::str::FromStr for IdentifierEntryIdentifierTypeSubtype0 {
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
impl std::convert::TryFrom<&str> for IdentifierEntryIdentifierTypeSubtype0 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IdentifierEntryIdentifierTypeSubtype0 {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IdentifierEntryIdentifierTypeSubtype0 {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IdentifierEntryIdentifierTypeSubtype1(String);
impl std::ops::Deref for IdentifierEntryIdentifierTypeSubtype1 {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IdentifierEntryIdentifierTypeSubtype1> for String {
    fn from(value: IdentifierEntryIdentifierTypeSubtype1) -> Self {
        value.0
    }
}
impl From<&IdentifierEntryIdentifierTypeSubtype1> for IdentifierEntryIdentifierTypeSubtype1 {
    fn from(value: &IdentifierEntryIdentifierTypeSubtype1) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IdentifierEntryIdentifierTypeSubtype1 {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new("(ext:)[a-z|A-Z|0-9|.|-|_]+")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"(ext:)[a-z|A-Z|0-9|.|-|_]+\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IdentifierEntryIdentifierTypeSubtype1 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IdentifierEntryIdentifierTypeSubtype1 {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IdentifierEntryIdentifierTypeSubtype1 {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IdentifierEntryIdentifierTypeSubtype1 {
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
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityObject {
    #[doc = "Whether or not the `identityHash` value is hashed."]
    pub hashed: bool,
    #[doc = "Either the IdentityHash of the identity or the plaintext value. If it's possible that the plaintext transmission and storage of the identity value would leak personally identifiable information where there is an expectation of privacy, it is strongly recommended that an IdentityHash be used."]
    #[serde(rename = "identityHash")]
    pub identity_hash: String,
    #[doc = "The identity type."]
    #[serde(rename = "identityType")]
    pub identity_type: IdentityObjectIdentityType,
    #[doc = "If the `identityHash` is hashed, this should contain the string used to salt the hash. If this value is not provided, it should be assumed that the hash was not salted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<String>,
    #[doc = "MUST be the IRI 'IdentityObject'."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&IdentityObject> for IdentityObject {
    fn from(value: &IdentityObject) -> Self {
        value.clone()
    }
}
impl IdentityObject {
    pub fn builder() -> builder::IdentityObject {
        builder::IdentityObject::default()
    }
}
#[doc = "The identity type."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdentityObjectIdentityType {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<IdentityObjectIdentityTypeSubtype0>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<IdentityObjectIdentityTypeSubtype1>,
}
impl From<&IdentityObjectIdentityType> for IdentityObjectIdentityType {
    fn from(value: &IdentityObjectIdentityType) -> Self {
        value.clone()
    }
}
impl IdentityObjectIdentityType {
    pub fn builder() -> builder::IdentityObjectIdentityType {
        builder::IdentityObjectIdentityType::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IdentityObjectIdentityTypeSubtype0 {
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
impl From<&IdentityObjectIdentityTypeSubtype0> for IdentityObjectIdentityTypeSubtype0 {
    fn from(value: &IdentityObjectIdentityTypeSubtype0) -> Self {
        value.clone()
    }
}
impl ToString for IdentityObjectIdentityTypeSubtype0 {
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
impl std::str::FromStr for IdentityObjectIdentityTypeSubtype0 {
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
impl std::convert::TryFrom<&str> for IdentityObjectIdentityTypeSubtype0 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IdentityObjectIdentityTypeSubtype0 {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IdentityObjectIdentityTypeSubtype0 {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IdentityObjectIdentityTypeSubtype1(String);
impl std::ops::Deref for IdentityObjectIdentityTypeSubtype1 {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IdentityObjectIdentityTypeSubtype1> for String {
    fn from(value: IdentityObjectIdentityTypeSubtype1) -> Self {
        value.0
    }
}
impl From<&IdentityObjectIdentityTypeSubtype1> for IdentityObjectIdentityTypeSubtype1 {
    fn from(value: &IdentityObjectIdentityTypeSubtype1) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IdentityObjectIdentityTypeSubtype1 {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new("(ext:)[a-z|A-Z|0-9|.|-|_]+")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"(ext:)[a-z|A-Z|0-9|.|-|_]+\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IdentityObjectIdentityTypeSubtype1 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IdentityObjectIdentityTypeSubtype1 {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IdentityObjectIdentityTypeSubtype1 {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IdentityObjectIdentityTypeSubtype1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}


pub mod builder {
    #[derive(Clone, Debug)]
    pub struct IdentifierEntry {
        identifier: Result<String, String>,
        identifier_type: Result<super::IdentifierEntryIdentifierType, String>,
        type_: Result<String, String>,
    }
    impl Default for IdentifierEntry {
        fn default() -> Self {
            Self {
                identifier: Err("no value supplied for identifier".to_string()),
                identifier_type: Err("no value supplied for identifier_type".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl IdentifierEntry {
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
            T: std::convert::TryInto<super::IdentifierEntryIdentifierType>,
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
    impl std::convert::TryFrom<IdentifierEntry> for super::IdentifierEntry {
        type Error = String;
        fn try_from(value: IdentifierEntry) -> Result<Self, String> {
            Ok(Self {
                identifier: value.identifier?,
                identifier_type: value.identifier_type?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::IdentifierEntry> for IdentifierEntry {
        fn from(value: super::IdentifierEntry) -> Self {
            Self {
                identifier: Ok(value.identifier),
                identifier_type: Ok(value.identifier_type),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IdentifierEntryIdentifierType {
        subtype_0: Result<Option<super::IdentifierEntryIdentifierTypeSubtype0>, String>,
        subtype_1: Result<Option<super::IdentifierEntryIdentifierTypeSubtype1>, String>,
    }
    impl Default for IdentifierEntryIdentifierType {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl IdentifierEntryIdentifierType {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IdentifierEntryIdentifierTypeSubtype0>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IdentifierEntryIdentifierTypeSubtype1>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<IdentifierEntryIdentifierType> for super::IdentifierEntryIdentifierType {
        type Error = String;
        fn try_from(value: IdentifierEntryIdentifierType) -> Result<Self, String> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl From<super::IdentifierEntryIdentifierType> for IdentifierEntryIdentifierType {
        fn from(value: super::IdentifierEntryIdentifierType) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IdentityObject {
        hashed: Result<bool, String>,
        identity_hash: Result<String, String>,
        identity_type: Result<super::IdentityObjectIdentityType, String>,
        salt: Result<Option<String>, String>,
        type_: Result<String, String>,
    }
    impl Default for IdentityObject {
        fn default() -> Self {
            Self {
                hashed: Err("no value supplied for hashed".to_string()),
                identity_hash: Err("no value supplied for identity_hash".to_string()),
                identity_type: Err("no value supplied for identity_type".to_string()),
                salt: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl IdentityObject {
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
        pub fn identity_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IdentityObjectIdentityType>,
            T::Error: std::fmt::Display,
        {
            self.identity_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for identity_type: {}", e));
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
    impl std::convert::TryFrom<IdentityObject> for super::IdentityObject {
        type Error = String;
        fn try_from(value: IdentityObject) -> Result<Self, String> {
            Ok(Self {
                hashed: value.hashed?,
                identity_hash: value.identity_hash?,
                identity_type: value.identity_type?,
                salt: value.salt?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::IdentityObject> for IdentityObject {
        fn from(value: super::IdentityObject) -> Self {
            Self {
                hashed: Ok(value.hashed),
                identity_hash: Ok(value.identity_hash),
                identity_type: Ok(value.identity_type),
                salt: Ok(value.salt),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IdentityObjectIdentityType {
        subtype_0: Result<Option<super::IdentityObjectIdentityTypeSubtype0>, String>,
        subtype_1: Result<Option<super::IdentityObjectIdentityTypeSubtype1>, String>,
    }
    impl Default for IdentityObjectIdentityType {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl IdentityObjectIdentityType {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IdentityObjectIdentityTypeSubtype0>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IdentityObjectIdentityTypeSubtype1>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<IdentityObjectIdentityType> for super::IdentityObjectIdentityType {
        type Error = String;
        fn try_from(value: IdentityObjectIdentityType) -> Result<Self, String> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl From<super::IdentityObjectIdentityType> for IdentityObjectIdentityType {
        fn from(value: super::IdentityObjectIdentityType) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
}