use serde::{Deserialize, Serialize};
use super::alignment;

#[doc = "Describes a possible achievement result."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResultDescription {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alignment: Vec<alignment::Alignment>,
    #[serde(
        rename = "allowedValue",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_value: Vec<String>,
    #[doc = "The unique URI for this result description. Required so a result can link to this result description."]
    pub id: String,
    #[doc = "The name of the result."]
    pub name: String,
    #[doc = "The `id` of the rubric criterion level required to pass as determined by the achievement creator."]
    #[serde(
        rename = "requiredLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub required_level: Option<String>,
    #[doc = "A value from `allowedValue` or within the range of `valueMin` to `valueMax` required to pass as determined by the achievement creator."]
    #[serde(
        rename = "requiredValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub required_value: Option<String>,
    #[doc = "The type of result this description represents. This is an extensible enumerated vocabulary."]
    #[serde(rename = "resultType")]
    pub result_type: DescriptionResultType,
    #[serde(
        rename = "rubricCriterionLevel",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rubric_criterion_level: Vec<RubricCriterionLevel>,
    #[serde(rename = "type")]
    pub type_: DescriptionType,
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
impl ResultDescription {
    pub fn builder() -> builder::ResultDescription {
        builder::ResultDescription::default()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DescriptionType {
    SingleString(String),
    VecString(Vec<String>),
}

impl From<&DescriptionType> for DescriptionType {
    fn from(value: &DescriptionType) -> Self {
        value.clone()
    }
}

impl From<Vec<String>> for DescriptionType {
    fn from(value: Vec<String>) -> Self {
        Self::VecString(value)
    }
}


#[doc = "The type of result this description represents. This is an extensible enumerated vocabulary."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DescriptionResultType {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<ResultDescriptionResultTypeSubtype0>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<ResultDescriptionResultTypeSubtype1>,
}
impl From<&DescriptionResultType> for DescriptionResultType {
    fn from(value: &DescriptionResultType) -> Self {
        value.clone()
    }
}
impl DescriptionResultType {
    pub fn builder() -> builder::ResultDescriptionResultType {
        builder::ResultDescriptionResultType::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ResultDescriptionResultTypeSubtype0 {
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
impl From<&ResultDescriptionResultTypeSubtype0> for ResultDescriptionResultTypeSubtype0 {
    fn from(value: &ResultDescriptionResultTypeSubtype0) -> Self {
        value.clone()
    }
}
impl ToString for ResultDescriptionResultTypeSubtype0 {
    fn to_string(&self) -> String {
        match *self {
            Self::GradePointAverage => "GradePointAverage".to_string(),
            Self::LetterGrade => "LetterGrade".to_string(),
            Self::Percent => "Percent".to_string(),
            Self::PerformanceLevel => "PerformanceLevel".to_string(),
            Self::PredictedScore => "PredictedScore".to_string(),
            Self::RawScore => "RawScore".to_string(),
            Self::Result => "Result".to_string(),
            Self::RubricCriterion => "RubricCriterion".to_string(),
            Self::RubricCriterionLevel => "RubricCriterionLevel".to_string(),
            Self::RubricScore => "RubricScore".to_string(),
            Self::ScaledScore => "ScaledScore".to_string(),
            Self::Status => "Status".to_string(),
        }
    }
}
impl std::str::FromStr for ResultDescriptionResultTypeSubtype0 {
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
impl std::convert::TryFrom<&str> for ResultDescriptionResultTypeSubtype0 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ResultDescriptionResultTypeSubtype0 {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ResultDescriptionResultTypeSubtype0 {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ResultDescriptionResultTypeSubtype1(String);
impl std::ops::Deref for ResultDescriptionResultTypeSubtype1 {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ResultDescriptionResultTypeSubtype1> for String {
    fn from(value: ResultDescriptionResultTypeSubtype1) -> Self {
        value.0
    }
}
impl From<&ResultDescriptionResultTypeSubtype1> for ResultDescriptionResultTypeSubtype1 {
    fn from(value: &ResultDescriptionResultTypeSubtype1) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for ResultDescriptionResultTypeSubtype1 {
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
impl std::convert::TryFrom<&str> for ResultDescriptionResultTypeSubtype1 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ResultDescriptionResultTypeSubtype1 {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ResultDescriptionResultTypeSubtype1 {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for ResultDescriptionResultTypeSubtype1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}

#[doc = "Describes a result that was achieved."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResultTemp {
    #[doc = "If the result represents an achieved rubric criterion level (e.g. Mastered), the value is the `id` of the RubricCriterionLevel in linked ResultDescription."]
    #[serde(
        rename = "achievedLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub achieved_level: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alignment: Vec<alignment::Alignment>,
    #[doc = "An achievement can have many result descriptions describing possible results. The value of `resultDescription` is the `id` of the result description linked to this result. The linked result description must be in the achievement that is being asserted."]
    #[serde(
        rename = "resultDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_description: Option<String>,
    #[doc = "The status of the achievement. Required if `resultType` of the linked ResultDescription is Status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResultTempStatus>,
    #[serde(rename = "type")]
    pub type_: ResultTempType,
    #[doc = "A string representing the result of the performance, or demonstration, of the achievement. For example, 'A' if the recipient received an A grade in class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl From<&ResultTemp> for ResultTemp {
    fn from(value: &ResultTemp) -> Self {
        value.clone()
    }
}
impl ResultTemp {
    pub fn builder() -> builder::ResultTemp {
        builder::ResultTemp::default()
    }
}
#[doc = "The status of the achievement. Required if `resultType` of the linked ResultDescription is Status."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ResultTempStatus {
    Completed,
    Enrolled,
    Failed,
    InProgress,
    OnHold,
    Withdrew,
}
impl From<&ResultTempStatus> for ResultTempStatus {
    fn from(value: &ResultTempStatus) -> Self {
        value.clone()
    }
}
impl ToString for ResultTempStatus {
    fn to_string(&self) -> String {
        match *self {
            Self::Completed => "Completed".to_string(),
            Self::Enrolled => "Enrolled".to_string(),
            Self::Failed => "Failed".to_string(),
            Self::InProgress => "InProgress".to_string(),
            Self::OnHold => "OnHold".to_string(),
            Self::Withdrew => "Withdrew".to_string(),
        }
    }
}
impl std::str::FromStr for ResultTempStatus {
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
impl std::convert::TryFrom<&str> for ResultTempStatus {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ResultTempStatus {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ResultTempStatus {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ResultTempType {
    SingleString(String),
    VecString(Vec<String>),
}
impl From<&ResultTempType> for ResultTempType {
    fn from(value: &ResultTempType) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for ResultTempType {
    fn from(value: Vec<String>) -> Self {
        Self::VecString(value)
    }
}
#[doc = "Describes a rubric criterion level."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RubricCriterionLevel {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alignment: Vec<alignment::Alignment>,
    #[doc = "Description of the rubric criterion level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The unique URI for this rubric criterion level. Required so a result can link to this rubric criterion level."]
    pub id: String,
    #[doc = "The rubric performance level in terms of success."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[doc = "The name of the rubric criterion level."]
    pub name: String,
    #[doc = "The points associated with this rubric criterion level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub points: Option<String>,
    #[serde(rename = "type")]
    pub type_: RubricCriterionLevelType,
}
impl From<&RubricCriterionLevel> for RubricCriterionLevel {
    fn from(value: &RubricCriterionLevel) -> Self {
        value.clone()
    }
}
impl RubricCriterionLevel {
    pub fn builder() -> builder::RubricCriterionLevel {
        builder::RubricCriterionLevel::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RubricCriterionLevelType {
    SingleString(String),
    VecString(Vec<String>),
}
impl From<&RubricCriterionLevelType> for RubricCriterionLevelType {
    fn from(value: &RubricCriterionLevelType) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for RubricCriterionLevelType {
    fn from(value: Vec<String>) -> Self {
        Self::VecString(value)
    }
}


pub mod builder {
    use crate::json_schema::alignment;

    #[derive(Clone, Debug)]
    pub struct ResultDescription {
        alignment: Result<Vec<alignment::Alignment>, String>,
        allowed_value: Result<Vec<String>, String>,
        id: Result<String, String>,
        name: Result<String, String>,
        required_level: Result<Option<String>, String>,
        required_value: Result<Option<String>, String>,
        result_type: Result<super::DescriptionResultType, String>,
        rubric_criterion_level: Result<Vec<super::RubricCriterionLevel>, String>,
        type_: Result<super::DescriptionType, String>,
        value_max: Result<Option<String>, String>,
        value_min: Result<Option<String>, String>,
    }
    impl Default for ResultDescription {
        fn default() -> Self {
            Self {
                alignment: Ok(Default::default()),
                allowed_value: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                required_level: Ok(Default::default()),
                required_value: Ok(Default::default()),
                result_type: Err("no value supplied for result_type".to_string()),
                rubric_criterion_level: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                value_max: Ok(Default::default()),
                value_min: Ok(Default::default()),
            }
        }
    }
    impl ResultDescription {
        pub fn alignment<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<alignment::Alignment>>,
            T::Error: std::fmt::Display,
        {
            self.alignment = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alignment: {}", e));
            self
        }
        pub fn allowed_value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.allowed_value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for allowed_value: {}", e));
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
        pub fn result_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DescriptionResultType>,
            T::Error: std::fmt::Display,
        {
            self.result_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result_type: {}", e));
            self
        }
        pub fn rubric_criterion_level<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::RubricCriterionLevel>>,
            T::Error: std::fmt::Display,
        {
            self.rubric_criterion_level = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for rubric_criterion_level: {}",
                    e
                )
            });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DescriptionType>,
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
    impl std::convert::TryFrom<ResultDescription> for super::ResultDescription {
        type Error = String;
        fn try_from(value: ResultDescription) -> Result<Self, String> {
            Ok(Self {
                alignment: value.alignment?,
                allowed_value: value.allowed_value?,
                id: value.id?,
                name: value.name?,
                required_level: value.required_level?,
                required_value: value.required_value?,
                result_type: value.result_type?,
                rubric_criterion_level: value.rubric_criterion_level?,
                type_: value.type_?,
                value_max: value.value_max?,
                value_min: value.value_min?,
            })
        }
    }
    impl From<super::ResultDescription> for ResultDescription {
        fn from(value: super::ResultDescription) -> Self {
            Self {
                alignment: Ok(value.alignment),
                allowed_value: Ok(value.allowed_value),
                id: Ok(value.id),
                name: Ok(value.name),
                required_level: Ok(value.required_level),
                required_value: Ok(value.required_value),
                result_type: Ok(value.result_type),
                rubric_criterion_level: Ok(value.rubric_criterion_level),
                type_: Ok(value.type_),
                value_max: Ok(value.value_max),
                value_min: Ok(value.value_min),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResultDescriptionResultType {
        subtype_0: Result<Option<super::ResultDescriptionResultTypeSubtype0>, String>,
        subtype_1: Result<Option<super::ResultDescriptionResultTypeSubtype1>, String>,
    }
    impl Default for ResultDescriptionResultType {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl ResultDescriptionResultType {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ResultDescriptionResultTypeSubtype0>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ResultDescriptionResultTypeSubtype1>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ResultDescriptionResultType> for super::DescriptionResultType {
        type Error = String;
        fn try_from(value: ResultDescriptionResultType) -> Result<Self, String> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl From<super::DescriptionResultType> for ResultDescriptionResultType {
        fn from(value: super::DescriptionResultType) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResultTemp {
        achieved_level: Result<Option<String>, String>,
        alignment: Result<Vec<alignment::Alignment>, String>,
        result_description: Result<Option<String>, String>,
        status: Result<Option<super::ResultTempStatus>, String>,
        type_: Result<super::ResultTempType, String>,
        value: Result<Option<String>, String>,
    }
    impl Default for ResultTemp {
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
    impl ResultTemp {
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
        pub fn alignment<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<alignment::Alignment>>,
            T::Error: std::fmt::Display,
        {
            self.alignment = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alignment: {}", e));
            self
        }
        pub fn result_description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.result_description = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for result_description: {}",
                    e
                )
            });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ResultTempStatus>>,
            T::Error: std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ResultTempType>,
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
    impl std::convert::TryFrom<ResultTemp> for super::ResultTemp {
        type Error = String;
        fn try_from(value: ResultTemp) -> Result<Self, String> {
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
    impl From<super::ResultTemp> for ResultTemp {
        fn from(value: super::ResultTemp) -> Self {
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

    #[derive(Clone, Debug)]
    pub struct RubricCriterionLevel {
        alignment: Result<Vec<alignment::Alignment>, String>,
        description: Result<Option<String>, String>,
        id: Result<String, String>,
        level: Result<Option<String>, String>,
        name: Result<String, String>,
        points: Result<Option<String>, String>,
        type_: Result<super::RubricCriterionLevelType, String>,
    }
    impl Default for RubricCriterionLevel {
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
    impl RubricCriterionLevel {
        pub fn alignment<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<alignment::Alignment>>,
            T::Error: std::fmt::Display,
        {
            self.alignment = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alignment: {}", e));
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
            T: std::convert::TryInto<super::RubricCriterionLevelType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<RubricCriterionLevel> for super::RubricCriterionLevel {
        type Error = String;
        fn try_from(value: RubricCriterionLevel) -> Result<Self, String> {
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
    impl From<super::RubricCriterionLevel> for RubricCriterionLevel {
        fn from(value: super::RubricCriterionLevel) -> Self {
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
}
