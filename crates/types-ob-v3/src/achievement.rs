use std::fmt;

use super::{alignment, endorsement, general, identity, profile, related, result::ResultDescription};
use serde::{Deserialize, Serialize};
use types_common::{GenPaths, SchemaList};

#[doc = "A collection of information about the accomplishment recognized by the Assertion. Many assertions may be created corresponding to one Achievement."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
pub struct Achievement {
    #[doc = "Unique URI for the Achievement."]
    pub id: String,
    #[serde(rename = "type")]
    pub type_: Type,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alignment: Vec<alignment::Alignment>,
    #[doc = "The type of achievement. This is an extensible vocabulary."]
    #[serde(rename = "achievementType", default, skip_serializing_if = "Option::is_none")]
    pub achievement_type: Option<AchievementType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<profile::Profile>,
    #[serde(rename = "creditsAvailable", default, skip_serializing_if = "Option::is_none")]
    pub credits_available: Option<f64>,
    pub criteria: Criteria,
    #[doc = "A short description of the achievement."]
    pub description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endorsement: Vec<endorsement::EndorsementCredential>,
    #[serde(rename = "endorsementJwt", default, skip_serializing_if = "Vec::is_empty")]
    pub endorsement_jwt: Vec<AchievementEndorsementJwtItem>,
    #[doc = "Category, subject, area of study, discipline, or general branch of knowledge. Examples include Business, Education, Psychology, and Technology."]
    #[serde(rename = "fieldOfStudy", default, skip_serializing_if = "Option::is_none")]
    pub field_of_study: Option<String>,
    #[doc = "The code, generally human readable, associated with an achievement."]
    #[serde(rename = "humanCode", default, skip_serializing_if = "Option::is_none")]
    pub human_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<general::Image>,
    #[doc = "The language of the achievement."]
    #[serde(rename = "@language", default, skip_serializing_if = "Option::is_none")]
    pub language: Option<AchievementLanguage>,
    #[doc = "The name of the achievement."]
    pub name: String,
    #[serde(rename = "otherIdentifier", default, skip_serializing_if = "Vec::is_empty")]
    pub other_identifier: Vec<identity::IdentifierEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related: Vec<related::Related>,
    #[serde(rename = "resultDescription", default, skip_serializing_if = "Vec::is_empty")]
    pub result_description: Vec<ResultDescription>,
    #[doc = "Name given to the focus, concentration, or specific area of study defined in the achievement. Examples include 'Entrepreneurship', 'Technical Communication', and 'Finance'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specialization: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tag: Vec<String>,
    #[doc = "The version property allows issuers to set a version string for an Achievement. This is particularly useful when replacing a previous version with an update."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl From<&Achievement> for Achievement {
    fn from(value: &Achievement) -> Self {
        value.clone()
    }
}

#[doc = "The type of achievement. This is an extensible vocabulary."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
#[serde(untagged)]
pub enum AchievementType {
    Enum(AchievementTypeEnum),
    String(AchievementTypeString),
}
impl From<&AchievementType> for AchievementType {
    fn from(value: &AchievementType) -> Self {
        value.clone()
    }
}
impl From<AchievementTypeEnum> for AchievementType {
    fn from(value: AchievementTypeEnum) -> Self {
        Self::Enum(value)
    }
}
impl From<AchievementTypeString> for AchievementType {
    fn from(value: AchievementTypeString) -> Self {
        Self::String(value)
    }
}
impl std::str::FromStr for AchievementType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        AchievementTypeEnum::from_str(value)
            .map(Self::Enum)
            .or_else(|_| AchievementTypeString::from_str(value).map(Self::String))
            .map_err(|_| "invalid value")
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, GenPaths)]
pub enum AchievementTypeEnum {
    Achievement,
    ApprenticeshipCertificate,
    Assessment,
    Assignment,
    AssociateDegree,
    Award,
    Badge,
    BachelorDegree,
    Certificate,
    CertificateOfCompletion,
    Certification,
    CommunityService,
    Competency,
    Course,
    CoCurricular,
    Degree,
    Diploma,
    DoctoralDegree,
    Fieldwork,
    GeneralEducationDevelopment,
    JourneymanCertificate,
    LearningProgram,
    License,
    Membership,
    ProfessionalDoctorate,
    QualityAssuranceCredential,
    MasterCertificate,
    MasterDegree,
    MicroCredential,
    ResearchDoctorate,
    SecondarySchoolDiploma,
}

impl From<&AchievementTypeEnum> for AchievementTypeEnum {
    fn from(value: &AchievementTypeEnum) -> Self {
        *value
    }
}

impl fmt::Display for AchievementTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Achievement => f.write_str("Achievement"),
            Self::ApprenticeshipCertificate => f.write_str("ApprenticeshipCertificate"),
            Self::Assessment => f.write_str("Assessment"),
            Self::Assignment => f.write_str("Assignment"),
            Self::AssociateDegree => f.write_str("AssociateDegree"),
            Self::Award => f.write_str("Award"),
            Self::Badge => f.write_str("Badge"),
            Self::BachelorDegree => f.write_str("BachelorDegree"),
            Self::Certificate => f.write_str("Certificate"),
            Self::CertificateOfCompletion => f.write_str("CertificateOfCompletion"),
            Self::Certification => f.write_str("Certification"),
            Self::CommunityService => f.write_str("CommunityService"),
            Self::Competency => f.write_str("Competency"),
            Self::Course => f.write_str("Course"),
            Self::CoCurricular => f.write_str("CoCurricular"),
            Self::Degree => f.write_str("Degree"),
            Self::Diploma => f.write_str("Diploma"),
            Self::DoctoralDegree => f.write_str("DoctoralDegree"),
            Self::Fieldwork => f.write_str("Fieldwork"),
            Self::GeneralEducationDevelopment => f.write_str("GeneralEducationDevelopment"),
            Self::JourneymanCertificate => f.write_str("JourneymanCertificate"),
            Self::LearningProgram => f.write_str("LearningProgram"),
            Self::License => f.write_str("License"),
            Self::Membership => f.write_str("Membership"),
            Self::ProfessionalDoctorate => f.write_str("ProfessionalDoctorate"),
            Self::QualityAssuranceCredential => f.write_str("QualityAssuranceCredential"),
            Self::MasterCertificate => f.write_str("MasterCertificate"),
            Self::MasterDegree => f.write_str("MasterDegree"),
            Self::MicroCredential => f.write_str("MicroCredential"),
            Self::ResearchDoctorate => f.write_str("ResearchDoctorate"),
            Self::SecondarySchoolDiploma => f.write_str("SecondarySchoolDiploma"),
        }
    }
}

impl std::str::FromStr for AchievementTypeEnum {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "Achievement" => Ok(Self::Achievement),
            "ApprenticeshipCertificate" => Ok(Self::ApprenticeshipCertificate),
            "Assessment" => Ok(Self::Assessment),
            "Assignment" => Ok(Self::Assignment),
            "AssociateDegree" => Ok(Self::AssociateDegree),
            "Award" => Ok(Self::Award),
            "Badge" => Ok(Self::Badge),
            "BachelorDegree" => Ok(Self::BachelorDegree),
            "Certificate" => Ok(Self::Certificate),
            "CertificateOfCompletion" => Ok(Self::CertificateOfCompletion),
            "Certification" => Ok(Self::Certification),
            "CommunityService" => Ok(Self::CommunityService),
            "Competency" => Ok(Self::Competency),
            "Course" => Ok(Self::Course),
            "CoCurricular" => Ok(Self::CoCurricular),
            "Degree" => Ok(Self::Degree),
            "Diploma" => Ok(Self::Diploma),
            "DoctoralDegree" => Ok(Self::DoctoralDegree),
            "Fieldwork" => Ok(Self::Fieldwork),
            "GeneralEducationDevelopment" => Ok(Self::GeneralEducationDevelopment),
            "JourneymanCertificate" => Ok(Self::JourneymanCertificate),
            "LearningProgram" => Ok(Self::LearningProgram),
            "License" => Ok(Self::License),
            "Membership" => Ok(Self::Membership),
            "ProfessionalDoctorate" => Ok(Self::ProfessionalDoctorate),
            "QualityAssuranceCredential" => Ok(Self::QualityAssuranceCredential),
            "MasterCertificate" => Ok(Self::MasterCertificate),
            "MasterDegree" => Ok(Self::MasterDegree),
            "MicroCredential" => Ok(Self::MicroCredential),
            "ResearchDoctorate" => Ok(Self::ResearchDoctorate),
            "SecondarySchoolDiploma" => Ok(Self::SecondarySchoolDiploma),
            _ => Err("invalid value"),
        }
    }
}

impl std::convert::TryFrom<&str> for AchievementTypeEnum {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for AchievementTypeEnum {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for AchievementTypeEnum {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, GenPaths)]
pub struct AchievementTypeString(String);
impl std::ops::Deref for AchievementTypeString {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl From<AchievementTypeString> for String {
    fn from(value: AchievementTypeString) -> Self {
        value.0
    }
}

impl From<&AchievementTypeString> for AchievementTypeString {
    fn from(value: &AchievementTypeString) -> Self {
        value.clone()
    }
}

impl std::str::FromStr for AchievementTypeString {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regex::Regex::new("(ext:)[a-z|A-Z|0-9|.|-|_]+").unwrap().is_match(value) {
            Ok(Self(value.to_string()))
        } else {
            Err("doesn't match pattern \"(ext:)[a-z|A-Z|0-9|.|-|_]+\"")
        }
    }
}

impl std::convert::TryFrom<&str> for AchievementTypeString {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for AchievementTypeString {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for AchievementTypeString {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for AchievementTypeString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}

#[doc = "Descriptive metadata about the achievements necessary to be recognized with an assertion of a particular achievement. This data is added to the Achievement class so that it may be rendered when the achievement assertion is displayed, instead of simply a link to human-readable criteria external to the achievement. Embedding criteria allows either enhancement of an external criteria page or increased portability and ease of use by allowing issuers to skip hosting the formerly-required external criteria page altogether. Criteria is used to allow would-be recipients to learn what is required of them to be recognized with an assertion of a particular achievement. It is also used after the assertion is awarded to a recipient to let those inspecting earned achievements know the general requirements that the recipients met in order to earn it."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
pub struct Criteria {
    #[doc = "The URI of a webpage that describes in a human-readable format the criteria for the achievement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "A narrative of what is needed to earn the achievement. Markdown is allowed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrative: Option<String>,
}

impl From<&Criteria> for Criteria {
    fn from(value: &Criteria) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
#[serde(untagged)]
pub enum Type {
    String(String),
    VecString(Vec<String>),
}

impl From<&Type> for Type {
    fn from(value: &Type) -> Self {
        value.clone()
    }
}
impl From<String> for Type {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
impl From<&str> for Type {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}
impl From<Vec<String>> for Type {
    fn from(value: Vec<String>) -> Self {
        Self::VecString(value)
    }
}
impl From<Vec<&str>> for Type {
    fn from(value: Vec<&str>) -> Self {
        let v = value.iter().map(|v| v.to_string()).collect();
        Self::VecString(v)
    }
}

#[doc = "Allows endorsers to make specific claims about the Achievement. These endorsements are signed with the VC-JWT proof format."]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, GenPaths)]
pub struct AchievementEndorsementJwtItem(String);
impl std::ops::Deref for AchievementEndorsementJwtItem {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl From<AchievementEndorsementJwtItem> for String {
    fn from(value: AchievementEndorsementJwtItem) -> Self {
        value.0
    }
}

impl From<&AchievementEndorsementJwtItem> for AchievementEndorsementJwtItem {
    fn from(value: &AchievementEndorsementJwtItem) -> Self {
        value.clone()
    }
}

impl std::str::FromStr for AchievementEndorsementJwtItem {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regex::Regex::new("^[a-zA-Z0-9_-]+\\.[a-zA-Z0-9_-]*\\.[a-zA-Z0-9_-]+$")
            .unwrap()
            .is_match(value)
        {
            Ok(Self(value.to_string()))
        } else {
            Err("doesn't match pattern \"^[a-zA-Z0-9_-]+\\.[a-zA-Z0-9_-]*\\.[a-zA-Z0-9_-]+$\"")
        }
    }
}

impl std::convert::TryFrom<&str> for AchievementEndorsementJwtItem {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for AchievementEndorsementJwtItem {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for AchievementEndorsementJwtItem {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for AchievementEndorsementJwtItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}

#[doc = "The language of the achievement."]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, GenPaths)]
pub struct AchievementLanguage(String);
impl std::ops::Deref for AchievementLanguage {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl From<AchievementLanguage> for String {
    fn from(value: AchievementLanguage) -> Self {
        value.0
    }
}

impl From<&AchievementLanguage> for AchievementLanguage {
    fn from(value: &AchievementLanguage) -> Self {
        value.clone()
    }
}

impl std::str::FromStr for AchievementLanguage {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regex::Regex::new("^[a-z]{2,4}(-[A-Z][a-z]{3})?(-([A-Z]{2}|[0-9]{3}))?$")
            .unwrap()
            .is_match(value)
        {
            Ok(Self(value.to_string()))
        } else {
            Err("doesn't match pattern \"^[a-z]{2,4}(-[A-Z][a-z]{3})?(-([A-Z]{2}|[0-9]{3}))?$\"")
        }
    }
}

impl std::convert::TryFrom<&str> for AchievementLanguage {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for AchievementLanguage {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for AchievementLanguage {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for AchievementLanguage {
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
pub struct AchievementBuilder {
    achievement_type: Result<Option<AchievementType>, String>,
    alignment: Result<Vec<alignment::Alignment>, String>,
    creator: Result<Option<profile::Profile>, String>,
    credits_available: Result<Option<f64>, String>,
    criteria: Result<Criteria, String>,
    description: Result<String, String>,
    endorsement: Result<Vec<endorsement::EndorsementCredential>, String>,
    endorsement_jwt: Result<Vec<AchievementEndorsementJwtItem>, String>,
    field_of_study: Result<Option<String>, String>,
    human_code: Result<Option<String>, String>,
    id: Result<String, String>,
    image: Result<Option<general::Image>, String>,
    language: Result<Option<AchievementLanguage>, String>,
    name: Result<String, String>,
    other_identifier: Result<Vec<identity::IdentifierEntry>, String>,
    related: Result<Vec<related::Related>, String>,
    result_description: Result<Vec<ResultDescription>, String>,
    specialization: Result<Option<String>, String>,
    tag: Result<Vec<String>, String>,
    type_: Result<Type, String>,
    version: Result<Option<String>, String>,
}
impl Default for AchievementBuilder {
    fn default() -> Self {
        Self {
            achievement_type: Ok(Default::default()),
            alignment: Ok(Default::default()),
            creator: Ok(Default::default()),
            credits_available: Ok(Default::default()),
            criteria: Err("no value supplied for criteria".to_string()),
            description: Err("no value supplied for description".to_string()),
            endorsement: Ok(Default::default()),
            endorsement_jwt: Ok(Default::default()),
            field_of_study: Ok(Default::default()),
            human_code: Ok(Default::default()),
            id: Err("no value supplied for id".to_string()),
            image: Ok(Default::default()),
            language: Ok(Default::default()),
            name: Err("no value supplied for name".to_string()),
            other_identifier: Ok(Default::default()),
            related: Ok(Default::default()),
            result_description: Ok(Default::default()),
            specialization: Ok(Default::default()),
            tag: Ok(Default::default()),
            type_: Err("no value supplied for type_".to_string()),
            version: Ok(Default::default()),
        }
    }
}
impl AchievementBuilder {
    pub fn achievement_type<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<AchievementType>>,
        T::Error: std::fmt::Display,
    {
        self.achievement_type = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for achievement_type: {}", e));
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
    pub fn creator<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<profile::Profile>>,
        T::Error: std::fmt::Display,
    {
        self.creator = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for creator: {}", e));
        self
    }
    pub fn credits_available<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<f64>>,
        T::Error: std::fmt::Display,
    {
        self.credits_available = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for credits_available: {}", e));
        self
    }
    pub fn criteria<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Criteria>,
        T::Error: std::fmt::Display,
    {
        self.criteria = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for criteria: {}", e));
        self
    }
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<String>,
        T::Error: std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {}", e));
        self
    }
    pub fn endorsement<T>(mut self, value: Vec<T>) -> Self
    where
        T: std::convert::TryInto<endorsement::EndorsementCredential>,
        T::Error: std::fmt::Display,
    {
        self.endorsement = value
            .into_iter()
            .map(|value| {
                value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for endorsement: {}", e))
            })
            .collect();
        self
    }
    pub fn endorsement_jwt<T>(mut self, value: Vec<T>) -> Self
    where
        T: std::convert::TryInto<AchievementEndorsementJwtItem>,
        T::Error: std::fmt::Display,
    {
        self.endorsement_jwt = value
            .into_iter()
            .map(|value| {
                value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for endorsement_jwt: {}", e))
            })
            .collect();
        self
    }
    pub fn field_of_study<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.field_of_study = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for field_of_study: {}", e));
        self
    }
    pub fn human_code<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.human_code = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for human_code: {}", e));
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
    pub fn image<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<general::Image>>,
        T::Error: std::fmt::Display,
    {
        self.image = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for image: {}", e));
        self
    }
    pub fn language<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<AchievementLanguage>>,
        T::Error: std::fmt::Display,
    {
        self.language = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for language: {}", e));
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
    pub fn other_identifier<T>(mut self, value: Vec<T>) -> Self
    where
        T: std::convert::TryInto<identity::IdentifierEntry>,
        T::Error: std::fmt::Display,
    {
        self.other_identifier = value
            .into_iter()
            .map(|value| {
                value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for alignment: {}", e))
            })
            .collect();
        self
    }
    pub fn related<T>(mut self, value: Vec<T>) -> Self
    where
        T: std::convert::TryInto<related::Related>,
        T::Error: std::fmt::Display,
    {
        self.related = value
            .into_iter()
            .map(|value| {
                value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for related: {}", e))
            })
            .collect();
        self
    }
    pub fn result_description<T>(mut self, value: Vec<T>) -> Self
    where
        T: std::convert::TryInto<ResultDescription>,
        T::Error: std::fmt::Display,
    {
        self.result_description = value
            .into_iter()
            .map(|value| {
                value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for result_description: {}", e))
            })
            .collect();
        self
    }
    pub fn specialization<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.specialization = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for specialization: {}", e));
        self
    }
    pub fn tag<T>(mut self, value: Vec<T>) -> Self
    where
        T: std::convert::TryInto<String>,
        T::Error: std::fmt::Display,
    {
        self.tag = value
            .into_iter()
            .map(|value| {
                value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tag: {}", e))
            })
            .collect();
        self
    }
    pub fn type_<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Type>,
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
impl std::convert::TryFrom<AchievementBuilder> for Achievement {
    type Error = String;
    fn try_from(value: AchievementBuilder) -> Result<Self, String> {
        Ok(Self {
            achievement_type: value.achievement_type?,
            alignment: value.alignment?,
            creator: value.creator?,
            credits_available: value.credits_available?,
            criteria: value.criteria?,
            description: value.description?,
            endorsement: value.endorsement?,
            endorsement_jwt: value.endorsement_jwt?,
            field_of_study: value.field_of_study?,
            human_code: value.human_code?,
            id: value.id?,
            image: value.image?,
            language: value.language?,
            name: value.name?,
            other_identifier: value.other_identifier?,
            related: value.related?,
            result_description: value.result_description?,
            specialization: value.specialization?,
            tag: value.tag?,
            type_: value.type_?,
            version: value.version?,
        })
    }
}
impl From<Achievement> for AchievementBuilder {
    fn from(value: Achievement) -> Self {
        Self {
            achievement_type: Ok(value.achievement_type),
            alignment: Ok(value.alignment),
            creator: Ok(value.creator),
            credits_available: Ok(value.credits_available),
            criteria: Ok(value.criteria),
            description: Ok(value.description),
            endorsement: Ok(value.endorsement),
            endorsement_jwt: Ok(value.endorsement_jwt),
            field_of_study: Ok(value.field_of_study),
            human_code: Ok(value.human_code),
            id: Ok(value.id),
            image: Ok(value.image),
            language: Ok(value.language),
            name: Ok(value.name),
            other_identifier: Ok(value.other_identifier),
            related: Ok(value.related),
            result_description: Ok(value.result_description),
            specialization: Ok(value.specialization),
            tag: Ok(value.tag),
            type_: Ok(value.type_),
            version: Ok(value.version),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CriteriaBuilder {
    id: Result<Option<String>, String>,
    narrative: Result<Option<String>, String>,
}
impl Default for CriteriaBuilder {
    fn default() -> Self {
        Self {
            id: Ok(Default::default()),
            narrative: Ok(Default::default()),
        }
    }
}
impl CriteriaBuilder {
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
    pub fn narrative<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.narrative = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for narrative: {}", e));
        self
    }
}
impl std::convert::TryFrom<CriteriaBuilder> for Criteria {
    type Error = String;
    fn try_from(value: CriteriaBuilder) -> Result<Self, String> {
        Ok(Self {
            id: value.id?,
            narrative: value.narrative?,
        })
    }
}
impl From<Criteria> for CriteriaBuilder {
    fn from(value: Criteria) -> Self {
        Self {
            id: Ok(value.id),
            narrative: Ok(value.narrative),
        }
    }
}
