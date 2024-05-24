use std::fmt;

use serde::{Deserialize, Serialize};
use types_common::{GenPaths, SchemaList};

#[doc = "Describes an alignment between an achievement and a node in an educational framework."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
pub struct Alignment {
    #[serde(rename = "type")]
    pub type_: AlignmentType,
    #[doc = "If applicable, a locally unique string identifier that identifies the alignment target within its framework and/or targetUrl."]
    #[serde(rename = "targetCode", default, skip_serializing_if = "Option::is_none")]
    pub target_code: Option<String>,
    #[doc = "Short description of the alignment target."]
    #[serde(rename = "targetDescription", default, skip_serializing_if = "Option::is_none")]
    pub target_description: Option<String>,
    #[doc = "Name of the alignment."]
    #[serde(rename = "targetName")]
    pub target_name: String,
    #[doc = "Name of the framework the alignment target."]
    #[serde(rename = "targetFramework", default, skip_serializing_if = "Option::is_none")]
    pub target_framework: Option<String>,
    #[doc = "The type of the alignment target node."]
    #[serde(rename = "targetType", default, skip_serializing_if = "Option::is_none")]
    pub target_type: Option<AlignmentTargetType>,
    #[doc = "URL linking to the official description of the alignment target, for example an individual standard within an educational framework."]
    #[serde(rename = "targetUrl")]
    pub target_url: String,
}

impl From<&Alignment> for Alignment {
    fn from(value: &Alignment) -> Self {
        value.clone()
    }
}

#[doc = "The type of the alignment target node."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
#[serde(untagged)]
pub enum AlignmentTargetType {
    Enum(AlignmentTargetTypeEnum),
    String(AlignmentTargetTypeString),
}
impl From<&AlignmentTargetType> for AlignmentTargetType {
    fn from(value: &AlignmentTargetType) -> Self {
        value.clone()
    }
}
impl From<AlignmentTargetTypeEnum> for AlignmentTargetType {
    fn from(value: AlignmentTargetTypeEnum) -> Self {
        Self::Enum(value)
    }
}
impl From<AlignmentTargetTypeString> for AlignmentTargetType {
    fn from(value: AlignmentTargetTypeString) -> Self {
        Self::String(value)
    }
}

impl std::str::FromStr for AlignmentTargetType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        AlignmentTargetTypeEnum::from_str(value)
            .map(Self::Enum)
            .or_else(|_| AlignmentTargetTypeString::from_str(value).map(Self::String))
            .map_err(|_| "invalid value")
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, GenPaths)]
pub enum AlignmentTargetTypeEnum {
    #[serde(rename = "ceasn:Competency")]
    CeasnCompetency,
    #[serde(rename = "ceterms:Credential")]
    CetermsCredential,
    #[serde(rename = "CFItem")]
    CfItem,
    #[serde(rename = "CFRubric")]
    CfRubric,
    #[serde(rename = "CFRubricCriterion")]
    CfRubricCriterion,
    #[serde(rename = "CFRubricCriterionLevel")]
    CfRubricCriterionLevel,
    #[serde(rename = "CTDL")]
    Ctdl,
}
impl From<&AlignmentTargetTypeEnum> for AlignmentTargetTypeEnum {
    fn from(value: &AlignmentTargetTypeEnum) -> Self {
        *value
    }
}
impl fmt::Display for AlignmentTargetTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::CeasnCompetency => f.write_str("ceasn:Competency"),
            Self::CetermsCredential => f.write_str("ceterms:Credential"),
            Self::CfItem => f.write_str("CFItem"),
            Self::CfRubric => f.write_str("CFRubric"),
            Self::CfRubricCriterion => f.write_str("CFRubricCriterion"),
            Self::CfRubricCriterionLevel => f.write_str("CFRubricCriterionLevel"),
            Self::Ctdl => f.write_str("CTDL"),
        }
    }
}
impl std::str::FromStr for AlignmentTargetTypeEnum {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "ceasn:Competency" => Ok(Self::CeasnCompetency),
            "ceterms:Credential" => Ok(Self::CetermsCredential),
            "CFItem" => Ok(Self::CfItem),
            "CFRubric" => Ok(Self::CfRubric),
            "CFRubricCriterion" => Ok(Self::CfRubricCriterion),
            "CFRubricCriterionLevel" => Ok(Self::CfRubricCriterionLevel),
            "CTDL" => Ok(Self::Ctdl),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for AlignmentTargetTypeEnum {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AlignmentTargetTypeEnum {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AlignmentTargetTypeEnum {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, GenPaths)]
pub struct AlignmentTargetTypeString(String);
impl std::ops::Deref for AlignmentTargetTypeString {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AlignmentTargetTypeString> for String {
    fn from(value: AlignmentTargetTypeString) -> Self {
        value.0
    }
}
impl From<&AlignmentTargetTypeString> for AlignmentTargetTypeString {
    fn from(value: &AlignmentTargetTypeString) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for AlignmentTargetTypeString {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regex::Regex::new("(ext:)[a-z|A-Z|0-9|.|-|_]+").unwrap().is_match(value) {
            Ok(Self(value.to_string()))
        } else {
            Err("doesn't match pattern \"(ext:)[a-z|A-Z|0-9|.|-|_]+\"")
        }
    }
}
impl std::convert::TryFrom<&str> for AlignmentTargetTypeString {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AlignmentTargetTypeString {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AlignmentTargetTypeString {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for AlignmentTargetTypeString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
#[serde(untagged)]
pub enum AlignmentType {
    String(String),
    VecString(Vec<String>),
}
impl From<&AlignmentType> for AlignmentType {
    fn from(value: &AlignmentType) -> Self {
        value.clone()
    }
}
impl From<String> for AlignmentType {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
impl From<&str> for AlignmentType {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}
impl From<Vec<String>> for AlignmentType {
    fn from(value: Vec<String>) -> Self {
        Self::VecString(value)
    }
}
impl From<Vec<&str>> for AlignmentType {
    fn from(value: Vec<&str>) -> Self {
        let v = value.iter().map(|v| v.to_string()).collect();
        Self::VecString(v)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AlignmentBuilder {
    target_code: Result<Option<String>, String>,
    target_description: Result<Option<String>, String>,
    target_framework: Result<Option<String>, String>,
    target_name: Result<String, String>,
    target_type: Result<Option<AlignmentTargetType>, String>,
    target_url: Result<String, String>,
    type_: Result<AlignmentType, String>,
}
impl Default for AlignmentBuilder {
    fn default() -> Self {
        Self {
            target_code: Ok(Default::default()),
            target_description: Ok(Default::default()),
            target_framework: Ok(Default::default()),
            target_name: Err("no value supplied for target_name".to_string()),
            target_type: Ok(Default::default()),
            target_url: Err("no value supplied for target_url".to_string()),
            type_: Err("no value supplied for type_".to_string()),
        }
    }
}
impl AlignmentBuilder {
    pub fn target_code<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.target_code = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for target_code: {}", e));
        self
    }
    pub fn target_description<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.target_description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for target_description: {}", e));
        self
    }
    pub fn target_framework<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.target_framework = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for target_framework: {}", e));
        self
    }
    pub fn target_name<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<String>,
        T::Error: std::fmt::Display,
    {
        self.target_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for target_name: {}", e));
        self
    }
    pub fn target_type<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<AlignmentTargetType>>,
        T::Error: std::fmt::Display,
    {
        self.target_type = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for target_type: {}", e));
        self
    }
    pub fn target_url<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<String>,
        T::Error: std::fmt::Display,
    {
        self.target_url = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for target_url: {}", e));
        self
    }
    pub fn type_<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<AlignmentType>,
        T::Error: std::fmt::Display,
    {
        self.type_ = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for type_: {}", e));
        self
    }
}
impl std::convert::TryFrom<AlignmentBuilder> for Alignment {
    type Error = String;
    fn try_from(value: AlignmentBuilder) -> Result<Self, String> {
        Ok(Self {
            target_code: value.target_code?,
            target_description: value.target_description?,
            target_framework: value.target_framework?,
            target_name: value.target_name?,
            target_type: value.target_type?,
            target_url: value.target_url?,
            type_: value.type_?,
        })
    }
}
impl From<Alignment> for AlignmentBuilder {
    fn from(value: Alignment) -> Self {
        Self {
            target_code: Ok(value.target_code),
            target_description: Ok(value.target_description),
            target_framework: Ok(value.target_framework),
            target_name: Ok(value.target_name),
            target_type: Ok(value.target_type),
            target_url: Ok(value.target_url),
            type_: Ok(value.type_),
        }
    }
}
