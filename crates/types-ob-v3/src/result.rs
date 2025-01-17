use std::fmt;

use super::alignment;
use serde::{Deserialize, Serialize};
use types_common::{GenPaths, SchemaList};

/// Originally named: Result
/// Describes a result that was achieved.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
pub struct ResultAchievement {
    #[serde(rename = "type")]
    pub type_: ResultType,
    #[doc = "If the result represents an achieved rubric criterion level (e.g. Mastered), the value is the `id` of the RubricCriterionLevel in linked ResultDescription."]
    #[serde(rename = "achievedLevel", default, skip_serializing_if = "Option::is_none")]
    pub achieved_level: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alignment: Vec<alignment::Alignment>,
    #[doc = "An achievement can have many result descriptions describing possible results. The value of `resultDescription` is the `id` of the result description linked to this result. The linked result description must be in the achievement that is being asserted."]
    #[serde(rename = "resultDescription", default, skip_serializing_if = "Option::is_none")]
    pub result_description: Option<String>,
    #[doc = "The status of the achievement. Required if `resultType` of the linked ResultDescription is Status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResultStatus>,
    #[doc = "A string representing the result of the performance, or demonstration, of the achievement. For example, 'A' if the recipient received an A grade in class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl From<&ResultAchievement> for ResultAchievement {
    fn from(value: &ResultAchievement) -> Self {
        value.clone()
    }
}

#[doc = "The status of the achievement. Required if `resultType` of the linked ResultDescription is Status."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, GenPaths)]
pub enum ResultStatus {
    Completed,
    Enrolled,
    Failed,
    InProgress,
    OnHold,
    Withdrew,
}
impl From<&ResultStatus> for ResultStatus {
    fn from(value: &ResultStatus) -> Self {
        *value
    }
}

impl fmt::Display for ResultStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Completed => f.write_str("Completed"),
            Self::Enrolled => f.write_str("Enrolled"),
            Self::Failed => f.write_str("Failed"),
            Self::InProgress => f.write_str("InProgress"),
            Self::OnHold => f.write_str("OnHold"),
            Self::Withdrew => f.write_str("Withdrew"),
        }
    }
}

impl std::str::FromStr for ResultStatus {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "Completed" => Ok(Self::Completed),
            "Enrolled" => Ok(Self::Enrolled),
            "Failed" => Ok(Self::Failed),
            "InProgress" => Ok(Self::InProgress),
            "OnHold" => Ok(Self::OnHold),
            "Withdrew" => Ok(Self::Withdrew),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for ResultStatus {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ResultStatus {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ResultStatus {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
#[serde(untagged)]
pub enum ResultType {
    String(String),
    VecString(Vec<String>),
}
impl From<&ResultType> for ResultType {
    fn from(value: &ResultType) -> Self {
        value.clone()
    }
}
impl From<String> for ResultType {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
impl From<&str> for ResultType {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}
impl From<Vec<String>> for ResultType {
    fn from(value: Vec<String>) -> Self {
        Self::VecString(value)
    }
}
impl From<Vec<&str>> for ResultType {
    fn from(value: Vec<&str>) -> Self {
        let v = value.iter().map(|v| v.to_string()).collect();
        Self::VecString(v)
    }
}

#[doc = "Describes a possible achievement result."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
pub struct ResultDescription {
    #[doc = "The unique URI for this result description. Required so a result can link to this result description."]
    pub id: String,
    #[serde(rename = "type")]
    pub type_: DescriptionType,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alignment: Vec<alignment::Alignment>,
    #[serde(rename = "allowedValue", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_value: Vec<String>,
    #[doc = "The name of the result."]
    pub name: String,
    #[doc = "The `id` of the rubric criterion level required to pass as determined by the achievement creator."]
    #[serde(rename = "requiredLevel", default, skip_serializing_if = "Option::is_none")]
    pub required_level: Option<String>,
    #[doc = "A value from `allowedValue` or within the range of `valueMin` to `valueMax` required to pass as determined by the achievement creator."]
    #[serde(rename = "requiredValue", default, skip_serializing_if = "Option::is_none")]
    pub required_value: Option<String>,
    #[doc = "The type of result this description represents. This is an extensible enumerated vocabulary."]
    #[serde(rename = "resultType")]
    pub result_description_type: ResultDescriptionType,
    #[serde(rename = "rubricCriterionLevel", default, skip_serializing_if = "Vec::is_empty")]
    pub rubric_criterion_level: Vec<RubricCriterionLevel>,
    #[doc = "The maximum possible `value` that may be asserted in a linked result."]
    #[serde(rename = "valueMax", default, skip_serializing_if = "Option::is_none")]
    pub value_max: Option<String>,
    #[doc = "The minimum possible `value` that may be asserted in a linked result."]
    #[serde(rename = "valueMin", default, skip_serializing_if = "Option::is_none")]
    pub value_min: Option<String>,
}

impl From<&ResultDescription> for ResultDescription {
    fn from(value: &ResultDescription) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
#[serde(untagged)]
pub enum DescriptionType {
    String(String),
    VecString(Vec<String>),
}

impl From<&DescriptionType> for DescriptionType {
    fn from(value: &DescriptionType) -> Self {
        value.clone()
    }
}
impl From<String> for DescriptionType {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
impl From<&str> for DescriptionType {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}
impl From<Vec<String>> for DescriptionType {
    fn from(value: Vec<String>) -> Self {
        Self::VecString(value)
    }
}
impl From<Vec<&str>> for DescriptionType {
    fn from(value: Vec<&str>) -> Self {
        let v = value.iter().map(|v| v.to_string()).collect();
        Self::VecString(v)
    }
}

#[doc = "The type of result this description represents. This is an extensible enumerated vocabulary."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
#[serde(untagged)]
pub enum ResultDescriptionType {
    Enum(ResultDescriptionTypeEnum),
    String(ResultDescriptionTypeString),
}
impl From<&ResultDescriptionType> for ResultDescriptionType {
    fn from(value: &ResultDescriptionType) -> Self {
        value.clone()
    }
}
impl From<ResultDescriptionTypeEnum> for ResultDescriptionType {
    fn from(value: ResultDescriptionTypeEnum) -> Self {
        Self::Enum(value)
    }
}

impl std::str::FromStr for ResultDescriptionType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        ResultDescriptionTypeEnum::from_str(value)
            .map(Self::Enum)
            .or_else(|_| ResultDescriptionTypeString::from_str(value).map(Self::String))
            .map_err(|_| "invalid value")
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, GenPaths)]
pub enum ResultDescriptionTypeEnum {
    GradePointAverage,
    LetterGrade,
    Percent,
    PerformanceLevel,
    PredictedScore,
    RawScore,
    Result,
    RubricCriterion,
    RubricCriterionLevel,
    RubricScore,
    ScaledScore,
    Status,
}
impl From<&ResultDescriptionTypeEnum> for ResultDescriptionTypeEnum {
    fn from(value: &ResultDescriptionTypeEnum) -> Self {
        *value
    }
}

impl fmt::Display for ResultDescriptionTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GradePointAverage => f.write_str("GradePointAverage"),
            Self::LetterGrade => f.write_str("LetterGrade"),
            Self::Percent => f.write_str("Percent"),
            Self::PerformanceLevel => f.write_str("PerformanceLevel"),
            Self::PredictedScore => f.write_str("PredictedScore"),
            Self::RawScore => f.write_str("RawScore"),
            Self::Result => f.write_str("Result"),
            Self::RubricCriterion => f.write_str("RubricCriterion"),
            Self::RubricCriterionLevel => f.write_str("RubricCriterionLevel"),
            Self::RubricScore => f.write_str("RubricScore"),
            Self::ScaledScore => f.write_str("ScaledScore"),
            Self::Status => f.write_str("Status"),
        }
    }
}

impl std::str::FromStr for ResultDescriptionTypeEnum {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "GradePointAverage" => Ok(Self::GradePointAverage),
            "LetterGrade" => Ok(Self::LetterGrade),
            "Percent" => Ok(Self::Percent),
            "PerformanceLevel" => Ok(Self::PerformanceLevel),
            "PredictedScore" => Ok(Self::PredictedScore),
            "RawScore" => Ok(Self::RawScore),
            "Result" => Ok(Self::Result),
            "RubricCriterion" => Ok(Self::RubricCriterion),
            "RubricCriterionLevel" => Ok(Self::RubricCriterionLevel),
            "RubricScore" => Ok(Self::RubricScore),
            "ScaledScore" => Ok(Self::ScaledScore),
            "Status" => Ok(Self::Status),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for ResultDescriptionTypeEnum {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ResultDescriptionTypeEnum {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ResultDescriptionTypeEnum {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, GenPaths)]
pub struct ResultDescriptionTypeString(String);
impl std::ops::Deref for ResultDescriptionTypeString {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ResultDescriptionTypeString> for String {
    fn from(value: ResultDescriptionTypeString) -> Self {
        value.0
    }
}
impl From<&ResultDescriptionTypeString> for ResultDescriptionTypeString {
    fn from(value: &ResultDescriptionTypeString) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for ResultDescriptionTypeString {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regex::Regex::new("(ext:)[a-z|A-Z|0-9|.|-|_]+").unwrap().is_match(value) {
            Ok(Self(value.to_string()))
        } else {
            Err("doesn't match pattern \"(ext:)[a-z|A-Z|0-9|.|-|_]+\"")
        }
    }
}
impl std::convert::TryFrom<&str> for ResultDescriptionTypeString {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ResultDescriptionTypeString {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ResultDescriptionTypeString {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for ResultDescriptionTypeString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "Describes a rubric criterion level."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
pub struct RubricCriterionLevel {
    #[doc = "The unique URI for this rubric criterion level. Required so a result can link to this rubric criterion level."]
    pub id: String,
    #[serde(rename = "type")]
    pub type_: RubricCriterionLevelType,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alignment: Vec<alignment::Alignment>,
    #[doc = "Description of the rubric criterion level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The rubric performance level in terms of success."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[doc = "The name of the rubric criterion level."]
    pub name: String,
    #[doc = "The points associated with this rubric criterion level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub points: Option<String>,
}
impl From<&RubricCriterionLevel> for RubricCriterionLevel {
    fn from(value: &RubricCriterionLevel) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
#[serde(untagged)]
pub enum RubricCriterionLevelType {
    String(String),
    VecString(Vec<String>),
}
impl From<&RubricCriterionLevelType> for RubricCriterionLevelType {
    fn from(value: &RubricCriterionLevelType) -> Self {
        value.clone()
    }
}
impl From<String> for RubricCriterionLevelType {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
impl From<&str> for RubricCriterionLevelType {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}
impl From<Vec<String>> for RubricCriterionLevelType {
    fn from(value: Vec<String>) -> Self {
        Self::VecString(value)
    }
}
impl From<Vec<&str>> for RubricCriterionLevelType {
    fn from(value: Vec<&str>) -> Self {
        let v = value.iter().map(|v| v.to_string()).collect();
        Self::VecString(v)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResultDescriptionBuilder {
    alignment: Result<Vec<alignment::Alignment>, String>,
    allowed_value: Result<Vec<String>, String>,
    id: Result<String, String>,
    name: Result<String, String>,
    required_level: Result<Option<String>, String>,
    required_value: Result<Option<String>, String>,
    result_description_type: Result<ResultDescriptionType, String>,
    rubric_criterion_level: Result<Vec<RubricCriterionLevel>, String>,
    type_: Result<DescriptionType, String>,
    value_max: Result<Option<String>, String>,
    value_min: Result<Option<String>, String>,
}
impl Default for ResultDescriptionBuilder {
    fn default() -> Self {
        Self {
            alignment: Ok(Default::default()),
            allowed_value: Ok(Default::default()),
            id: Err("no value supplied for id".to_string()),
            name: Err("no value supplied for name".to_string()),
            required_level: Ok(Default::default()),
            required_value: Ok(Default::default()),
            result_description_type: Err("no value supplied for result_description_type".to_string()),
            rubric_criterion_level: Ok(Default::default()),
            type_: Err("no value supplied for type_".to_string()),
            value_max: Ok(Default::default()),
            value_min: Ok(Default::default()),
        }
    }
}
impl ResultDescriptionBuilder {
    pub fn alignment<T>(mut self, value: Vec<T>) -> Self
    where
        T: std::convert::TryInto<alignment::Alignment>,
        T::Error: std::fmt::Display,
    {
        self.alignment = value
            .into_iter()
            .map(|value| {
                value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for alignment: {}", e))
            })
            .collect();
        self
    }
    pub fn allowed_value<T>(mut self, value: Vec<T>) -> Self
    where
        T: std::convert::TryInto<String>,
        T::Error: std::fmt::Display,
    {
        self.allowed_value = value
            .into_iter()
            .map(|value| {
                value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for allowed_value: {}", e))
            })
            .collect();
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
    pub fn name<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<String>,
        T::Error: std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {}", e));
        self
    }
    pub fn required_level<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.required_level = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for required_level: {}", e));
        self
    }
    pub fn required_value<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.required_value = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for required_value: {}", e));
        self
    }
    pub fn result_description_type<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<ResultDescriptionType>,
        T::Error: std::fmt::Display,
    {
        self.result_description_type = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for result_description_type: {}", e));
        self
    }
    pub fn rubric_criterion_level<T>(mut self, value: Vec<T>) -> Self
    where
        T: std::convert::TryInto<RubricCriterionLevel>,
        T::Error: std::fmt::Display,
    {
        self.rubric_criterion_level = value
            .into_iter()
            .map(|value| {
                value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rubric_criterion_level: {}", e))
            })
            .collect();
        self
    }
    pub fn type_<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<DescriptionType>,
        T::Error: std::fmt::Display,
    {
        self.type_ = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for type_: {}", e));
        self
    }
    pub fn value_max<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.value_max = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for value_max: {}", e));
        self
    }
    pub fn value_min<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.value_min = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for value_min: {}", e));
        self
    }
}
impl std::convert::TryFrom<ResultDescriptionBuilder> for ResultDescription {
    type Error = String;
    fn try_from(value: ResultDescriptionBuilder) -> Result<Self, String> {
        Ok(Self {
            alignment: value.alignment?,
            allowed_value: value.allowed_value?,
            id: value.id?,
            name: value.name?,
            required_level: value.required_level?,
            required_value: value.required_value?,
            result_description_type: value.result_description_type?,
            rubric_criterion_level: value.rubric_criterion_level?,
            type_: value.type_?,
            value_max: value.value_max?,
            value_min: value.value_min?,
        })
    }
}
impl From<ResultDescription> for ResultDescriptionBuilder {
    fn from(value: ResultDescription) -> Self {
        Self {
            alignment: Ok(value.alignment),
            allowed_value: Ok(value.allowed_value),
            id: Ok(value.id),
            name: Ok(value.name),
            required_level: Ok(value.required_level),
            required_value: Ok(value.required_value),
            result_description_type: Ok(value.result_description_type),
            rubric_criterion_level: Ok(value.rubric_criterion_level),
            type_: Ok(value.type_),
            value_max: Ok(value.value_max),
            value_min: Ok(value.value_min),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResultBuilder {
    achieved_level: Result<Option<String>, String>,
    alignment: Result<Vec<alignment::Alignment>, String>,
    result_description: Result<Option<String>, String>,
    status: Result<Option<ResultStatus>, String>,
    type_: Result<ResultType, String>,
    value: Result<Option<String>, String>,
}
impl Default for ResultBuilder {
    fn default() -> Self {
        Self {
            achieved_level: Ok(Default::default()),
            alignment: Ok(Default::default()),
            result_description: Ok(Default::default()),
            status: Ok(Default::default()),
            type_: Err("no value supplied for type_".to_string()),
            value: Ok(Default::default()),
        }
    }
}
impl ResultBuilder {
    pub fn achieved_level<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.achieved_level = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for achieved_level: {}", e));
        self
    }
    pub fn alignment<T>(mut self, value: Vec<T>) -> Self
    where
        T: std::convert::TryInto<alignment::Alignment>,
        T::Error: std::fmt::Display,
    {
        self.alignment = value
            .into_iter()
            .map(|value| {
                value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for alignment: {}", e))
            })
            .collect();
        self
    }
    pub fn result_description<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.result_description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for result_description: {}", e));
        self
    }
    pub fn status<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<ResultStatus>>,
        T::Error: std::fmt::Display,
    {
        self.status = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for status: {}", e));
        self
    }
    pub fn type_<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<ResultType>,
        T::Error: std::fmt::Display,
    {
        self.type_ = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for type_: {}", e));
        self
    }
    pub fn value<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.value = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for value: {}", e));
        self
    }
}
impl std::convert::TryFrom<ResultBuilder> for ResultAchievement {
    type Error = String;
    fn try_from(value: ResultBuilder) -> Result<Self, String> {
        Ok(Self {
            achieved_level: value.achieved_level?,
            alignment: value.alignment?,
            result_description: value.result_description?,
            status: value.status?,
            type_: value.type_?,
            value: value.value?,
        })
    }
}
impl From<ResultAchievement> for ResultBuilder {
    fn from(value: ResultAchievement) -> Self {
        Self {
            achieved_level: Ok(value.achieved_level),
            alignment: Ok(value.alignment),
            result_description: Ok(value.result_description),
            status: Ok(value.status),
            type_: Ok(value.type_),
            value: Ok(value.value),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RubricCriterionLevelBuilder {
    alignment: Result<Vec<alignment::Alignment>, String>,
    description: Result<Option<String>, String>,
    id: Result<String, String>,
    level: Result<Option<String>, String>,
    name: Result<String, String>,
    points: Result<Option<String>, String>,
    type_: Result<RubricCriterionLevelType, String>,
}
impl Default for RubricCriterionLevelBuilder {
    fn default() -> Self {
        Self {
            alignment: Ok(Default::default()),
            description: Ok(Default::default()),
            id: Err("no value supplied for id".to_string()),
            level: Ok(Default::default()),
            name: Err("no value supplied for name".to_string()),
            points: Ok(Default::default()),
            type_: Err("no value supplied for type_".to_string()),
        }
    }
}
impl RubricCriterionLevelBuilder {
    pub fn alignment<T>(mut self, value: Vec<T>) -> Self
    where
        T: std::convert::TryInto<alignment::Alignment>,
        T::Error: std::fmt::Display,
    {
        self.alignment = value
            .into_iter()
            .map(|value| {
                value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for alignment: {}", e))
            })
            .collect();
        self
    }
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {}", e));
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
    pub fn level<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.level = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for level: {}", e));
        self
    }
    pub fn name<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<String>,
        T::Error: std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {}", e));
        self
    }
    pub fn points<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.points = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for points: {}", e));
        self
    }
    pub fn type_<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<RubricCriterionLevelType>,
        T::Error: std::fmt::Display,
    {
        self.type_ = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for type_: {}", e));
        self
    }
}
impl std::convert::TryFrom<RubricCriterionLevelBuilder> for RubricCriterionLevel {
    type Error = String;
    fn try_from(value: RubricCriterionLevelBuilder) -> Result<Self, String> {
        Ok(Self {
            alignment: value.alignment?,
            description: value.description?,
            id: value.id?,
            level: value.level?,
            name: value.name?,
            points: value.points?,
            type_: value.type_?,
        })
    }
}
impl From<RubricCriterionLevel> for RubricCriterionLevelBuilder {
    fn from(value: RubricCriterionLevel) -> Self {
        Self {
            alignment: Ok(value.alignment),
            description: Ok(value.description),
            id: Ok(value.id),
            level: Ok(value.level),
            name: Ok(value.name),
            points: Ok(value.points),
            type_: Ok(value.type_),
        }
    }
}
