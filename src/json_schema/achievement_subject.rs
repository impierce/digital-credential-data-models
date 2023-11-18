use serde::{Deserialize, Serialize};
use super::{achievement, profile, result, identity, general};

#[doc = "A collection of information about the recipient of an achievement. Maps to Credential Subject in [[VC-DATA-MODEL]]."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AchievementSubject {
    pub achievement: achievement::Achievement,
    #[doc = "The datetime the activity ended."]
    #[serde(
        rename = "activityEndDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub activity_end_date: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "The datetime the activity started."]
    #[serde(
        rename = "activityStartDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub activity_start_date: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[serde(
        rename = "creditsEarned",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub credits_earned: Option<f64>,
    #[doc = "An identifier for the Credential Subject. Either `id` or at least one `identifier` MUST be supplied."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<identity::IdentityObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<general::Image>,
    #[doc = "The license number that was issued with this credential."]
    #[serde(
        rename = "licenseNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub license_number: Option<String>,
    #[doc = "A narrative that connects multiple pieces of evidence. Likely only present at this location if evidence is a multi-value array."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrative: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<result::ResultTemp>,
    #[doc = "Role, position, or title of the learner when demonstrating or performing the achievement or evidence of learning being asserted. Examples include 'Student President', 'Intern', 'Captain', etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<profile::Profile>,
    #[doc = "The academic term in which this assertion was achieved."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    #[serde(rename = "type")]
    pub type_: AchievementSubjectType,
}
impl From<&AchievementSubject> for AchievementSubject {
    fn from(value: &AchievementSubject) -> Self {
        value.clone()
    }
}
impl AchievementSubject {
    pub fn builder() -> builder::AchievementSubject {
        builder::AchievementSubject::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AchievementSubjectType {
    SingleString(String),
    VecString(Vec<String>),
}
impl From<&AchievementSubjectType> for AchievementSubjectType {
    fn from(value: &AchievementSubjectType) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for AchievementSubjectType {
    fn from(value: Vec<String>) -> Self {
        Self::VecString(value)
    }
}

pub mod builder {
    use crate::json_schema::{achievement, identity, general, result, profile};

    
    #[derive(Clone, Debug)]
    pub struct AchievementSubject {
        achievement: Result<achievement::Achievement, String>,
        activity_end_date: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        activity_start_date: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        credits_earned: Result<Option<f64>, String>,
        id: Result<Option<String>, String>,
        identifier: Result<Vec<identity::IdentityObject>, String>,
        image: Result<Option<general::Image>, String>,
        license_number: Result<Option<String>, String>,
        narrative: Result<Option<String>, String>,
        result: Result<Vec<result::ResultTemp>, String>,
        role: Result<Option<String>, String>,
        source: Result<Option<profile::Profile>, String>,
        term: Result<Option<String>, String>,
        type_: Result<super::AchievementSubjectType, String>,
    }
    impl Default for AchievementSubject {
        fn default() -> Self {
            Self {
                achievement: Err("no value supplied for achievement".to_string()),
                activity_end_date: Ok(Default::default()),
                activity_start_date: Ok(Default::default()),
                credits_earned: Ok(Default::default()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                image: Ok(Default::default()),
                license_number: Ok(Default::default()),
                narrative: Ok(Default::default()),
                result: Ok(Default::default()),
                role: Ok(Default::default()),
                source: Ok(Default::default()),
                term: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AchievementSubject {
        pub fn achievement<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<achievement::Achievement>,
            T::Error: std::fmt::Display,
        {
            self.achievement = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for achievement: {}", e));
            self
        }
        pub fn activity_end_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self.activity_end_date = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for activity_end_date: {}",
                    e
                )
            });
            self
        }
        pub fn activity_start_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self.activity_start_date = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for activity_start_date: {}",
                    e
                )
            });
            self
        }
        pub fn credits_earned<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.credits_earned = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for credits_earned: {}", e));
            self
        }
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
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<identity::IdentityObject>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for identifier: {}", e));
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
        pub fn license_number<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.license_number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for license_number: {}", e));
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
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<result::ResultTemp>>,
            T::Error: std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {}", e));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<profile::Profile>>,
            T::Error: std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn term<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.term = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for term: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AchievementSubjectType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AchievementSubject> for super::AchievementSubject {
        type Error = String;
        fn try_from(value: AchievementSubject) -> Result<Self, String> {
            Ok(Self {
                achievement: value.achievement?,
                activity_end_date: value.activity_end_date?,
                activity_start_date: value.activity_start_date?,
                credits_earned: value.credits_earned?,
                id: value.id?,
                identifier: value.identifier?,
                image: value.image?,
                license_number: value.license_number?,
                narrative: value.narrative?,
                result: value.result?,
                role: value.role?,
                source: value.source?,
                term: value.term?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::AchievementSubject> for AchievementSubject {
        fn from(value: super::AchievementSubject) -> Self {
            Self {
                achievement: Ok(value.achievement),
                activity_end_date: Ok(value.activity_end_date),
                activity_start_date: Ok(value.activity_start_date),
                credits_earned: Ok(value.credits_earned),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                image: Ok(value.image),
                license_number: Ok(value.license_number),
                narrative: Ok(value.narrative),
                result: Ok(value.result),
                role: Ok(value.role),
                source: Ok(value.source),
                term: Ok(value.term),
                type_: Ok(value.type_),
            }
        }
    }
}
