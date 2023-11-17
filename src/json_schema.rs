#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

#[doc = "A collection of information about the accomplishment recognized by the Assertion. Many assertions may be created corresponding to one Achievement."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Achievement {
    #[doc = "The type of achievement. This is an extensible vocabulary."]
    #[serde(
        rename = "achievementType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub achievement_type: Option<AchievementAchievementType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alignment: Vec<Alignment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<Profile>,
    #[serde(
        rename = "creditsAvailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub credits_available: Option<f64>,
    pub criteria: Criteria,
    #[doc = "A short description of the achievement."]
    pub description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endorsement: Vec<EndorsementCredential>,
    #[serde(
        rename = "endorsementJwt",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub endorsement_jwt: Vec<AchievementEndorsementJwtItem>,
    #[doc = "Category, subject, area of study, discipline, or general branch of knowledge. Examples include Business, Education, Psychology, and Technology."]
    #[serde(
        rename = "fieldOfStudy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub field_of_study: Option<String>,
    #[doc = "The code, generally human readable, associated with an achievement."]
    #[serde(rename = "humanCode", default, skip_serializing_if = "Option::is_none")]
    pub human_code: Option<String>,
    #[doc = "Unique URI for the Achievement."]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    #[doc = "The language of the achievement."]
    #[serde(rename = "@language", default, skip_serializing_if = "Option::is_none")]
    pub language: Option<AchievementLanguage>,
    #[doc = "The name of the achievement."]
    pub name: String,
    #[serde(
        rename = "otherIdentifier",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub other_identifier: Vec<IdentifierEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related: Vec<Related>,
    #[serde(
        rename = "resultDescription",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub result_description: Vec<ResultDescription>,
    #[doc = "Name given to the focus, concentration, or specific area of study defined in the achievement. Examples include 'Entrepreneurship', 'Technical Communication', and 'Finance'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specialization: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tag: Vec<String>,
    #[serde(rename = "type")]
    pub type_: AchievementType,
    #[doc = "The version property allows issuers to set a version string for an Achievement. This is particularly useful when replacing a previous version with an update."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl From<&Achievement> for Achievement {
    fn from(value: &Achievement) -> Self {
        value.clone()
    }
}
impl Achievement {
    pub fn builder() -> builder::Achievement {
        builder::Achievement::default()
    }
}
#[doc = "The type of achievement. This is an extensible vocabulary."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AchievementAchievementType {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<AchievementAchievementTypeSubtype0>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<AchievementAchievementTypeSubtype1>,
}
impl From<&AchievementAchievementType> for AchievementAchievementType {
    fn from(value: &AchievementAchievementType) -> Self {
        value.clone()
    }
}
impl AchievementAchievementType {
    pub fn builder() -> builder::AchievementAchievementType {
        builder::AchievementAchievementType::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AchievementAchievementTypeSubtype0 {
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
impl From<&AchievementAchievementTypeSubtype0> for AchievementAchievementTypeSubtype0 {
    fn from(value: &AchievementAchievementTypeSubtype0) -> Self {
        value.clone()
    }
}
impl ToString for AchievementAchievementTypeSubtype0 {
    fn to_string(&self) -> String {
        match *self {
            Self::Achievement => "Achievement".to_string(),
            Self::ApprenticeshipCertificate => "ApprenticeshipCertificate".to_string(),
            Self::Assessment => "Assessment".to_string(),
            Self::Assignment => "Assignment".to_string(),
            Self::AssociateDegree => "AssociateDegree".to_string(),
            Self::Award => "Award".to_string(),
            Self::Badge => "Badge".to_string(),
            Self::BachelorDegree => "BachelorDegree".to_string(),
            Self::Certificate => "Certificate".to_string(),
            Self::CertificateOfCompletion => "CertificateOfCompletion".to_string(),
            Self::Certification => "Certification".to_string(),
            Self::CommunityService => "CommunityService".to_string(),
            Self::Competency => "Competency".to_string(),
            Self::Course => "Course".to_string(),
            Self::CoCurricular => "CoCurricular".to_string(),
            Self::Degree => "Degree".to_string(),
            Self::Diploma => "Diploma".to_string(),
            Self::DoctoralDegree => "DoctoralDegree".to_string(),
            Self::Fieldwork => "Fieldwork".to_string(),
            Self::GeneralEducationDevelopment => "GeneralEducationDevelopment".to_string(),
            Self::JourneymanCertificate => "JourneymanCertificate".to_string(),
            Self::LearningProgram => "LearningProgram".to_string(),
            Self::License => "License".to_string(),
            Self::Membership => "Membership".to_string(),
            Self::ProfessionalDoctorate => "ProfessionalDoctorate".to_string(),
            Self::QualityAssuranceCredential => "QualityAssuranceCredential".to_string(),
            Self::MasterCertificate => "MasterCertificate".to_string(),
            Self::MasterDegree => "MasterDegree".to_string(),
            Self::MicroCredential => "MicroCredential".to_string(),
            Self::ResearchDoctorate => "ResearchDoctorate".to_string(),
            Self::SecondarySchoolDiploma => "SecondarySchoolDiploma".to_string(),
        }
    }
}
impl std::str::FromStr for AchievementAchievementTypeSubtype0 {
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
impl std::convert::TryFrom<&str> for AchievementAchievementTypeSubtype0 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AchievementAchievementTypeSubtype0 {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AchievementAchievementTypeSubtype0 {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AchievementAchievementTypeSubtype1(String);
impl std::ops::Deref for AchievementAchievementTypeSubtype1 {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AchievementAchievementTypeSubtype1> for String {
    fn from(value: AchievementAchievementTypeSubtype1) -> Self {
        value.0
    }
}
impl From<&AchievementAchievementTypeSubtype1> for AchievementAchievementTypeSubtype1 {
    fn from(value: &AchievementAchievementTypeSubtype1) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for AchievementAchievementTypeSubtype1 {
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
impl std::convert::TryFrom<&str> for AchievementAchievementTypeSubtype1 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AchievementAchievementTypeSubtype1 {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AchievementAchievementTypeSubtype1 {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for AchievementAchievementTypeSubtype1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "AchievementCredentials are representations of an awarded achievement, used to share information about a achievement belonging to one earner. Maps to a Verifiable Credential as defined in the [[VC-DATA-MODEL]]. As described in [[[#data-integrity]]], at least one proof mechanism, and the details necessary to evaluate that proof, MUST be expressed for a credential to be a verifiable credential. In the case of an embedded proof, the credential MUST append the proof in the `proof` property."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AchievementCredential {
    #[doc = "Timestamp of when the credential was awarded. `issuanceDate` is used to determine the most recent version of a Credential in conjunction with `issuer` and `id`. Consequently, the only way to update a Credental is to update the `issuanceDate`, losing the date when the Credential was originally awarded. `awardedDate` is meant to keep this original date."]
    #[serde(
        rename = "awardedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub awarded_date: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[serde(rename = "@context")]
    pub context: Vec<Context>,
    #[serde(
        rename = "credentialSchema",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub credential_schema: Option<AchievementCredentialCredentialSchema>,
    #[serde(
        rename = "credentialStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub credential_status: Option<CredentialStatus>,
    #[serde(rename = "credentialSubject")]
    pub credential_subject: AchievementSubject,
    #[doc = "The short description of the credential for display purposes in wallets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endorsement: Vec<EndorsementCredential>,
    #[serde(
        rename = "endorsementJwt",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub endorsement_jwt: Vec<AchievementCredentialEndorsementJwtItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<Evidence>,
    #[doc = "If the credential has some notion of expiry, this indicates a timestamp when a credential should no longer be considered valid. After this time, the credential should be considered expired."]
    #[serde(
        rename = "expirationDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiration_date: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "Unambiguous reference to the credential."]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    #[doc = "Timestamp of when the credential was issued."]
    #[serde(rename = "issuanceDate")]
    pub issuance_date: chrono::DateTime<chrono::offset::Utc>,
    pub issuer: Profile,
    #[doc = "The name of the credential for display purposes in wallets. For example, in a list of credentials and in detail views."]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof: Option<AchievementCredentialProof>,
    #[serde(
        rename = "refreshService",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub refresh_service: Option<RefreshService>,
    #[serde(
        rename = "termsOfUse",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub terms_of_use: Option<AchievementCredentialTermsOfUse>,
    #[serde(rename = "type")]
    pub type_: AchievementCredentialType,
}
impl From<&AchievementCredential> for AchievementCredential {
    fn from(value: &AchievementCredential) -> Self {
        value.clone()
    }
}
impl AchievementCredential {
    pub fn builder() -> builder::AchievementCredential {
        builder::AchievementCredential::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AchievementCredentialCredentialSchema {
    Variant0(CredentialSchema),
    Variant1(Vec<CredentialSchema>),
}
impl From<&AchievementCredentialCredentialSchema> for AchievementCredentialCredentialSchema {
    fn from(value: &AchievementCredentialCredentialSchema) -> Self {
        value.clone()
    }
}
impl From<CredentialSchema> for AchievementCredentialCredentialSchema {
    fn from(value: CredentialSchema) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<CredentialSchema>> for AchievementCredentialCredentialSchema {
    fn from(value: Vec<CredentialSchema>) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "Allows endorsers to make specific claims about the credential, and the achievement and profiles in the credential. These endorsements are signed with the VC-JWT proof format."]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AchievementCredentialEndorsementJwtItem(String);
impl std::ops::Deref for AchievementCredentialEndorsementJwtItem {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AchievementCredentialEndorsementJwtItem> for String {
    fn from(value: AchievementCredentialEndorsementJwtItem) -> Self {
        value.0
    }
}
impl From<&AchievementCredentialEndorsementJwtItem> for AchievementCredentialEndorsementJwtItem {
    fn from(value: &AchievementCredentialEndorsementJwtItem) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for AchievementCredentialEndorsementJwtItem {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new("^[a-zA-Z0-9_-]+\\.[a-zA-Z0-9_-]*\\.[a-zA-Z0-9_-]+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err(
                "doesn't match pattern \"^[a-zA-Z0-9_-]+\\.[a-zA-Z0-9_-]*\\.[a-zA-Z0-9_-]+$\"",
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for AchievementCredentialEndorsementJwtItem {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AchievementCredentialEndorsementJwtItem {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AchievementCredentialEndorsementJwtItem {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for AchievementCredentialEndorsementJwtItem {
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
pub enum AchievementCredentialProof {
    Variant0(Proof),
    Variant1(Vec<Proof>),
}
impl From<&AchievementCredentialProof> for AchievementCredentialProof {
    fn from(value: &AchievementCredentialProof) -> Self {
        value.clone()
    }
}
impl From<Proof> for AchievementCredentialProof {
    fn from(value: Proof) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<Proof>> for AchievementCredentialProof {
    fn from(value: Vec<Proof>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AchievementCredentialTermsOfUse {
    Variant0(TermsOfUse),
    Variant1(Vec<TermsOfUse>),
}
impl From<&AchievementCredentialTermsOfUse> for AchievementCredentialTermsOfUse {
    fn from(value: &AchievementCredentialTermsOfUse) -> Self {
        value.clone()
    }
}
impl From<TermsOfUse> for AchievementCredentialTermsOfUse {
    fn from(value: TermsOfUse) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<TermsOfUse>> for AchievementCredentialTermsOfUse {
    fn from(value: Vec<TermsOfUse>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AchievementCredentialType {
    Variant0(String),
    Variant1(Vec<String>),
}
impl From<&AchievementCredentialType> for AchievementCredentialType {
    fn from(value: &AchievementCredentialType) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for AchievementCredentialType {
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "Allows endorsers to make specific claims about the Achievement. These endorsements are signed with the VC-JWT proof format."]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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
        if regress::Regex::new("^[a-zA-Z0-9_-]+\\.[a-zA-Z0-9_-]*\\.[a-zA-Z0-9_-]+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err(
                "doesn't match pattern \"^[a-zA-Z0-9_-]+\\.[a-zA-Z0-9_-]*\\.[a-zA-Z0-9_-]+$\"",
            );
        }
        Ok(Self(value.to_string()))
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
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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
#[doc = "A collection of information about the recipient of an achievement. Maps to Credential Subject in [[VC-DATA-MODEL]]."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AchievementSubject {
    pub achievement: Achievement,
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
    pub identifier: Vec<IdentityObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
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
    pub result: Vec<ResultTemp>,
    #[doc = "Role, position, or title of the learner when demonstrating or performing the achievement or evidence of learning being asserted. Examples include 'Student President', 'Intern', 'Captain', etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Profile>,
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
    Variant0(String),
    Variant1(Vec<String>),
}
impl From<&AchievementSubjectType> for AchievementSubjectType {
    fn from(value: &AchievementSubjectType) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for AchievementSubjectType {
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AchievementType {
    Variant0(String),
    Variant1(Vec<String>),
}
impl From<&AchievementType> for AchievementType {
    fn from(value: &AchievementType) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for AchievementType {
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "An address for the described entity."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Address {
    #[doc = "A country."]
    #[serde(
        rename = "addressCountry",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub address_country: Option<String>,
    #[doc = "A country code. The value must be a ISO 3166-1 alpha-2 country code [[ISO3166-1]]."]
    #[serde(
        rename = "addressCountryCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub address_country_code: Option<String>,
    #[doc = "A locality within the region."]
    #[serde(
        rename = "addressLocality",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub address_locality: Option<String>,
    #[doc = "A region within the country."]
    #[serde(
        rename = "addressRegion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub address_region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geo: Option<GeoCoordinates>,
    #[doc = "A post office box number for PO box addresses."]
    #[serde(
        rename = "postOfficeBoxNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub post_office_box_number: Option<String>,
    #[doc = "A postal code."]
    #[serde(
        rename = "postalCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub postal_code: Option<String>,
    #[doc = "A street address within the locality."]
    #[serde(
        rename = "streetAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub street_address: Option<String>,
    #[serde(rename = "type")]
    pub type_: AddressType,
}
impl From<&Address> for Address {
    fn from(value: &Address) -> Self {
        value.clone()
    }
}
impl Address {
    pub fn builder() -> builder::Address {
        builder::Address::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AddressType {
    Variant0(String),
    Variant1(Vec<String>),
}
impl From<&AddressType> for AddressType {
    fn from(value: &AddressType) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for AddressType {
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "Describes an alignment between an achievement and a node in an educational framework."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Alignment {
    #[doc = "If applicable, a locally unique string identifier that identifies the alignment target within its framework and/or targetUrl."]
    #[serde(
        rename = "targetCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_code: Option<String>,
    #[doc = "Short description of the alignment target."]
    #[serde(
        rename = "targetDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_description: Option<String>,
    #[doc = "Name of the framework the alignment target."]
    #[serde(
        rename = "targetFramework",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_framework: Option<String>,
    #[doc = "Name of the alignment."]
    #[serde(rename = "targetName")]
    pub target_name: String,
    #[doc = "The type of the alignment target node."]
    #[serde(
        rename = "targetType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_type: Option<AlignmentTargetType>,
    #[doc = "URL linking to the official description of the alignment target, for example an individual standard within an educational framework."]
    #[serde(rename = "targetUrl")]
    pub target_url: String,
    #[serde(rename = "type")]
    pub type_: AlignmentType,
}
impl From<&Alignment> for Alignment {
    fn from(value: &Alignment) -> Self {
        value.clone()
    }
}
impl Alignment {
    pub fn builder() -> builder::Alignment {
        builder::Alignment::default()
    }
}
#[doc = "The type of the alignment target node."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AlignmentTargetType {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<AlignmentTargetTypeSubtype0>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<AlignmentTargetTypeSubtype1>,
}
impl From<&AlignmentTargetType> for AlignmentTargetType {
    fn from(value: &AlignmentTargetType) -> Self {
        value.clone()
    }
}
impl AlignmentTargetType {
    pub fn builder() -> builder::AlignmentTargetType {
        builder::AlignmentTargetType::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AlignmentTargetTypeSubtype0 {
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
impl From<&AlignmentTargetTypeSubtype0> for AlignmentTargetTypeSubtype0 {
    fn from(value: &AlignmentTargetTypeSubtype0) -> Self {
        value.clone()
    }
}
impl ToString for AlignmentTargetTypeSubtype0 {
    fn to_string(&self) -> String {
        match *self {
            Self::CeasnCompetency => "ceasn:Competency".to_string(),
            Self::CetermsCredential => "ceterms:Credential".to_string(),
            Self::CfItem => "CFItem".to_string(),
            Self::CfRubric => "CFRubric".to_string(),
            Self::CfRubricCriterion => "CFRubricCriterion".to_string(),
            Self::CfRubricCriterionLevel => "CFRubricCriterionLevel".to_string(),
            Self::Ctdl => "CTDL".to_string(),
        }
    }
}
impl std::str::FromStr for AlignmentTargetTypeSubtype0 {
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
impl std::convert::TryFrom<&str> for AlignmentTargetTypeSubtype0 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AlignmentTargetTypeSubtype0 {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AlignmentTargetTypeSubtype0 {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AlignmentTargetTypeSubtype1(String);
impl std::ops::Deref for AlignmentTargetTypeSubtype1 {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AlignmentTargetTypeSubtype1> for String {
    fn from(value: AlignmentTargetTypeSubtype1) -> Self {
        value.0
    }
}
impl From<&AlignmentTargetTypeSubtype1> for AlignmentTargetTypeSubtype1 {
    fn from(value: &AlignmentTargetTypeSubtype1) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for AlignmentTargetTypeSubtype1 {
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
impl std::convert::TryFrom<&str> for AlignmentTargetTypeSubtype1 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AlignmentTargetTypeSubtype1 {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AlignmentTargetTypeSubtype1 {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for AlignmentTargetTypeSubtype1 {
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
pub enum AlignmentType {
    Variant0(String),
    Variant1(Vec<String>),
}
impl From<&AlignmentType> for AlignmentType {
    fn from(value: &AlignmentType) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for AlignmentType {
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "JSON-LD Context. Either a URI with the context definition or a Map with a local context definition MUST be supplied."]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Context {
    Variant0(serde_json::Map<String, serde_json::Value>),
    Variant1(String),
}
impl From<&Context> for Context {
    fn from(value: &Context) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for Context {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "Identify the type and location of a data schema."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CredentialSchema {
    #[doc = "The value MUST be a URI identifying the schema file. One instance of `CredentialSchema` MUST have an `id` that is the URL of the JSON Schema for this credential defined by this specification."]
    pub id: String,
    #[doc = "The value MUST identify the type of data schema validation. One instance of `CredentialSchema` MUST have a `type` of 'JsonSchemaValidator2019'."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&CredentialSchema> for CredentialSchema {
    fn from(value: &CredentialSchema) -> Self {
        value.clone()
    }
}
impl CredentialSchema {
    pub fn builder() -> builder::CredentialSchema {
        builder::CredentialSchema::default()
    }
}
#[doc = "The information in CredentialStatus is used to discover information about the current status of a verifiable credential, such as whether it is suspended or revoked."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CredentialStatus {
    #[doc = "The value MUST be the URL of the issuer's credential status method."]
    pub id: String,
    #[doc = "The name of the credential status method."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&CredentialStatus> for CredentialStatus {
    fn from(value: &CredentialStatus) -> Self {
        value.clone()
    }
}
impl CredentialStatus {
    pub fn builder() -> builder::CredentialStatus {
        builder::CredentialStatus::default()
    }
}
#[doc = "Descriptive metadata about the achievements necessary to be recognized with an assertion of a particular achievement. This data is added to the Achievement class so that it may be rendered when the achievement assertion is displayed, instead of simply a link to human-readable criteria external to the achievement. Embedding criteria allows either enhancement of an external criteria page or increased portability and ease of use by allowing issuers to skip hosting the formerly-required external criteria page altogether. Criteria is used to allow would-be recipients to learn what is required of them to be recognized with an assertion of a particular achievement. It is also used after the assertion is awarded to a recipient to let those inspecting earned achievements know the general requirements that the recipients met in order to earn it."]
#[derive(Clone, Debug, Deserialize, Serialize)]
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
impl Criteria {
    pub fn builder() -> builder::Criteria {
        builder::Criteria::default()
    }
}
#[doc = "A verifiable credential that asserts a claim about an entity. As described in [[[#data-integrity]]], at least one proof mechanism, and the details necessary to evaluate that proof, MUST be expressed for a credential to be a verifiable credential. In the case of an embedded proof, the credential MUST append the proof in the `proof` property."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EndorsementCredential {
    #[doc = "Timestamp of when the credential was awarded. `issuanceDate` is used to determine the most recent version of a Credential in conjunction with `issuer` and `id`. Consequently, the only way to update a Credental is to update the `issuanceDate`, losing the date when the Credential was originally awarded. `awardedDate` is meant to keep this original date."]
    #[serde(
        rename = "awardedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub awarded_date: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[serde(rename = "@context")]
    pub context: Vec<Context>,
    #[serde(
        rename = "credentialSchema",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub credential_schema: Option<EndorsementCredentialCredentialSchema>,
    #[serde(
        rename = "credentialStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub credential_status: Option<CredentialStatus>,
    #[serde(rename = "credentialSubject")]
    pub credential_subject: EndorsementSubject,
    #[doc = "The short description of the credential for display purposes in wallets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "If the credential has some notion of expiry, this indicates a timestamp when a credential should no longer be considered valid. After this time, the credential should be considered expired."]
    #[serde(
        rename = "expirationDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiration_date: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "Unambiguous reference to the credential."]
    pub id: String,
    #[doc = "Timestamp of when the credential was issued."]
    #[serde(rename = "issuanceDate")]
    pub issuance_date: chrono::DateTime<chrono::offset::Utc>,
    pub issuer: Profile,
    #[doc = "The name of the credential for display purposes in wallets. For example, in a list of credentials and in detail views."]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof: Option<EndorsementCredentialProof>,
    #[serde(
        rename = "refreshService",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub refresh_service: Option<RefreshService>,
    #[serde(
        rename = "termsOfUse",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub terms_of_use: Option<EndorsementCredentialTermsOfUse>,
    #[serde(rename = "type")]
    pub type_: EndorsementCredentialType,
}
impl From<&EndorsementCredential> for EndorsementCredential {
    fn from(value: &EndorsementCredential) -> Self {
        value.clone()
    }
}
impl EndorsementCredential {
    pub fn builder() -> builder::EndorsementCredential {
        builder::EndorsementCredential::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EndorsementCredentialCredentialSchema {
    Variant0(CredentialSchema),
    Variant1(Vec<CredentialSchema>),
}
impl From<&EndorsementCredentialCredentialSchema> for EndorsementCredentialCredentialSchema {
    fn from(value: &EndorsementCredentialCredentialSchema) -> Self {
        value.clone()
    }
}
impl From<CredentialSchema> for EndorsementCredentialCredentialSchema {
    fn from(value: CredentialSchema) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<CredentialSchema>> for EndorsementCredentialCredentialSchema {
    fn from(value: Vec<CredentialSchema>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EndorsementCredentialProof {
    Variant0(Proof),
    Variant1(Vec<Proof>),
}
impl From<&EndorsementCredentialProof> for EndorsementCredentialProof {
    fn from(value: &EndorsementCredentialProof) -> Self {
        value.clone()
    }
}
impl From<Proof> for EndorsementCredentialProof {
    fn from(value: Proof) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<Proof>> for EndorsementCredentialProof {
    fn from(value: Vec<Proof>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EndorsementCredentialTermsOfUse {
    Variant0(TermsOfUse),
    Variant1(Vec<TermsOfUse>),
}
impl From<&EndorsementCredentialTermsOfUse> for EndorsementCredentialTermsOfUse {
    fn from(value: &EndorsementCredentialTermsOfUse) -> Self {
        value.clone()
    }
}
impl From<TermsOfUse> for EndorsementCredentialTermsOfUse {
    fn from(value: TermsOfUse) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<TermsOfUse>> for EndorsementCredentialTermsOfUse {
    fn from(value: Vec<TermsOfUse>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EndorsementCredentialType {
    Variant0(String),
    Variant1(Vec<String>),
}
impl From<&EndorsementCredentialType> for EndorsementCredentialType {
    fn from(value: &EndorsementCredentialType) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for EndorsementCredentialType {
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "A collection of information about the subject of the endorsement."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EndorsementSubject {
    #[doc = "Allows endorsers to make a simple claim in writing about the entity."]
    #[serde(
        rename = "endorsementComment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub endorsement_comment: Option<String>,
    #[doc = "The identifier of the individual, entity, organization, assertion, or achievement that is endorsed."]
    pub id: String,
    #[serde(rename = "type")]
    pub type_: EndorsementSubjectType,
}
impl From<&EndorsementSubject> for EndorsementSubject {
    fn from(value: &EndorsementSubject) -> Self {
        value.clone()
    }
}
impl EndorsementSubject {
    pub fn builder() -> builder::EndorsementSubject {
        builder::EndorsementSubject::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EndorsementSubjectType {
    Variant0(String),
    Variant1(Vec<String>),
}
impl From<&EndorsementSubjectType> for EndorsementSubjectType {
    fn from(value: &EndorsementSubjectType) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for EndorsementSubjectType {
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "Descriptive metadata about evidence related to the achievement assertion. Each instance of the evidence class present in an assertion corresponds to one entity, though a single entry can describe a set of items collectively. There may be multiple evidence entries referenced from an assertion. The narrative property is also in scope of the assertion class to provide an overall description of the achievement related to the assertion in rich text. It is used here to provide a narrative of achievement of the specific entity described. If both the description and narrative properties are present, displayers can assume the narrative value goes into more detail and is not simply a recapitulation of description."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Evidence {
    #[doc = "A description of the intended audience for a piece of evidence."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[doc = "A longer description of the evidence."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A string that describes the type of evidence. For example, Poetry, Prose, Film."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    #[doc = "The URL of a webpage presenting evidence of achievement or the evidence encoded as a Data URI. The schema of the webpage is undefined."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "A descriptive title of the evidence."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A narrative that describes the evidence and process of achievement that led to an assertion."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrative: Option<String>,
    #[serde(rename = "type")]
    pub type_: EvidenceType,
}
impl From<&Evidence> for Evidence {
    fn from(value: &Evidence) -> Self {
        value.clone()
    }
}
impl Evidence {
    pub fn builder() -> builder::Evidence {
        builder::Evidence::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EvidenceType {
    Variant0(String),
    Variant1(Vec<String>),
}
impl From<&EvidenceType> for EvidenceType {
    fn from(value: &EvidenceType) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for EvidenceType {
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "The geographic coordinates of a location."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GeoCoordinates {
    pub latitude: f64,
    pub longitude: f64,
    #[doc = "The value of the type property MUST be an unordered set. One of the items MUST be the IRI 'GeoCoordinates'."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&GeoCoordinates> for GeoCoordinates {
    fn from(value: &GeoCoordinates) -> Self {
        value.clone()
    }
}
impl GeoCoordinates {
    pub fn builder() -> builder::GeoCoordinates {
        builder::GeoCoordinates::default()
    }
}
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
#[doc = "A Profile is a collection of information that describes the entity or organization using Open Badges. Issuers must be represented as Profiles, and endorsers, or other entities may also be represented using this vocabulary. Each Profile that represents an Issuer may be referenced in many BadgeClasses that it has defined. Anyone can create and host an Issuer file to start issuing Open Badges. Issuers may also serve as recipients of Open Badges, often identified within an Assertion by specific properties, like their url or contact email address."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Profile {
    #[doc = "Additional name. Includes what is often referred to as 'middle name' in the western world."]
    #[serde(
        rename = "additionalName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[doc = "Birthdate of the person."]
    #[serde(
        rename = "dateOfBirth",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub date_of_birth: Option<chrono::naive::NaiveDate>,
    #[doc = "A short description of the issuer entity or organization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "An email address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endorsement: Vec<EndorsementCredential>,
    #[serde(
        rename = "endorsementJwt",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub endorsement_jwt: Vec<ProfileEndorsementJwtItem>,
    #[doc = "Family name. In the western world, often referred to as the 'last name' of a person."]
    #[serde(
        rename = "familyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub family_name: Option<String>,
    #[doc = "Family name prefix. As used in some locales, this is the leading part of a family name (e.g. 'de' in the name 'de Boer')."]
    #[serde(
        rename = "familyNamePrefix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub family_name_prefix: Option<String>,
    #[doc = "Given name. In the western world, often referred to as the 'first name' of a person."]
    #[serde(rename = "givenName", default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "Honorific prefix(es) preceding a person's name (e.g. 'Dr', 'Mrs' or 'Mr')."]
    #[serde(
        rename = "honorificPrefix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub honorific_prefix: Option<String>,
    #[doc = "Honorific suffix(es) following a person's name (e.g. 'M.D, PhD')."]
    #[serde(
        rename = "honorificSuffix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub honorific_suffix: Option<String>,
    #[doc = "Unique URI for the Issuer/Profile file."]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    #[doc = "The name of the entity or organization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "If the entity is an organization, `official` is the name of an authorized official of the organization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub official: Option<String>,
    #[serde(
        rename = "otherIdentifier",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub other_identifier: Vec<IdentifierEntry>,
    #[serde(rename = "parentOrg", default)]
    pub parent_org: Box<Option<Profile>>,
    #[doc = "Patronymic name."]
    #[serde(
        rename = "patronymicName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub patronymic_name: Option<String>,
    #[doc = "A phone number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "type")]
    pub type_: ProfileType,
    #[doc = "The homepage or social media profile of the entity, whether individual or institutional. Should be a URL/URI Accessible via HTTP."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&Profile> for Profile {
    fn from(value: &Profile) -> Self {
        value.clone()
    }
}
impl Profile {
    pub fn builder() -> builder::Profile {
        builder::Profile::default()
    }
}
#[doc = "Allows endorsers to make specific claims about the individual or organization represented by this profile. These endorsements are signed with the VC-JWT proof format."]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ProfileEndorsementJwtItem(String);
impl std::ops::Deref for ProfileEndorsementJwtItem {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ProfileEndorsementJwtItem> for String {
    fn from(value: ProfileEndorsementJwtItem) -> Self {
        value.0
    }
}
impl From<&ProfileEndorsementJwtItem> for ProfileEndorsementJwtItem {
    fn from(value: &ProfileEndorsementJwtItem) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for ProfileEndorsementJwtItem {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new("^[a-zA-Z0-9_-]+\\.[a-zA-Z0-9_-]*\\.[a-zA-Z0-9_-]+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err(
                "doesn't match pattern \"^[a-zA-Z0-9_-]+\\.[a-zA-Z0-9_-]*\\.[a-zA-Z0-9_-]+$\"",
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for ProfileEndorsementJwtItem {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ProfileEndorsementJwtItem {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ProfileEndorsementJwtItem {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for ProfileEndorsementJwtItem {
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
pub enum ProfileType {
    Variant0(String),
    Variant1(Vec<String>),
}
impl From<&ProfileType> for ProfileType {
    fn from(value: &ProfileType) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for ProfileType {
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "A JSON-LD Linked Data proof."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Proof {
    #[doc = "A value chosen by the verifier to mitigate authentication proof replay attacks."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub challenge: Option<String>,
    #[doc = "NaiveDate the proof was created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "The suite used to create the proof."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cryptosuite: Option<String>,
    #[doc = "The domain of the proof to restrict its use to a particular target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[doc = "A value chosen by the creator of proof to randomize proof values for privacy purposes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[doc = "The purpose of the proof to be used with `verificationMethod`. MUST be 'assertionMethod'."]
    #[serde(
        rename = "proofPurpose",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub proof_purpose: Option<String>,
    #[doc = "Value of the proof."]
    #[serde(
        rename = "proofValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub proof_value: Option<String>,
    #[doc = "Signature suite used to produce proof."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The URL of the public key that can verify the signature."]
    #[serde(
        rename = "verificationMethod",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verification_method: Option<String>,
}
impl From<&Proof> for Proof {
    fn from(value: &Proof) -> Self {
        value.clone()
    }
}
impl Proof {
    pub fn builder() -> builder::Proof {
        builder::Proof::default()
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
    Variant0(String),
    Variant1(Vec<String>),
}
impl From<&RelatedType> for RelatedType {
    fn from(value: &RelatedType) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for RelatedType {
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "Describes a possible achievement result."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResultDescription {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alignment: Vec<Alignment>,
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
    pub result_type: ResultDescriptionResultType,
    #[serde(
        rename = "rubricCriterionLevel",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rubric_criterion_level: Vec<RubricCriterionLevel>,
    #[serde(rename = "type")]
    pub type_: ResultDescriptionType,
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
#[doc = "The type of result this description represents. This is an extensible enumerated vocabulary."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResultDescriptionResultType {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<ResultDescriptionResultTypeSubtype0>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<ResultDescriptionResultTypeSubtype1>,
}
impl From<&ResultDescriptionResultType> for ResultDescriptionResultType {
    fn from(value: &ResultDescriptionResultType) -> Self {
        value.clone()
    }
}
impl ResultDescriptionResultType {
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
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ResultDescriptionType {
    Variant0(String),
    Variant1(Vec<String>),
}
impl From<&ResultDescriptionType> for ResultDescriptionType {
    fn from(value: &ResultDescriptionType) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for ResultDescriptionType {
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
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
    pub alignment: Vec<Alignment>,
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
    Variant0(String),
    Variant1(Vec<String>),
}
impl From<&ResultTempType> for ResultTempType {
    fn from(value: &ResultTempType) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for ResultTempType {
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "Describes a rubric criterion level."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RubricCriterionLevel {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alignment: Vec<Alignment>,
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
    Variant0(String),
    Variant1(Vec<String>),
}
impl From<&RubricCriterionLevelType> for RubricCriterionLevelType {
    fn from(value: &RubricCriterionLevelType) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for RubricCriterionLevelType {
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
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
    pub struct Achievement {
        achievement_type: Result<Option<super::AchievementAchievementType>, String>,
        alignment: Result<Vec<super::Alignment>, String>,
        creator: Result<Option<super::Profile>, String>,
        credits_available: Result<Option<f64>, String>,
        criteria: Result<super::Criteria, String>,
        description: Result<String, String>,
        endorsement: Result<Vec<super::EndorsementCredential>, String>,
        endorsement_jwt: Result<Vec<super::AchievementEndorsementJwtItem>, String>,
        field_of_study: Result<Option<String>, String>,
        human_code: Result<Option<String>, String>,
        id: Result<String, String>,
        image: Result<Option<super::Image>, String>,
        language: Result<Option<super::AchievementLanguage>, String>,
        name: Result<String, String>,
        other_identifier: Result<Vec<super::IdentifierEntry>, String>,
        related: Result<Vec<super::Related>, String>,
        result_description: Result<Vec<super::ResultDescription>, String>,
        specialization: Result<Option<String>, String>,
        tag: Result<Vec<String>, String>,
        type_: Result<super::AchievementType, String>,
        version: Result<Option<String>, String>,
    }
    impl Default for Achievement {
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
    impl Achievement {
        pub fn achievement_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AchievementAchievementType>>,
            T::Error: std::fmt::Display,
        {
            self.achievement_type = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for achievement_type: {}",
                    e
                )
            });
            self
        }
        pub fn alignment<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Alignment>>,
            T::Error: std::fmt::Display,
        {
            self.alignment = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alignment: {}", e));
            self
        }
        pub fn creator<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Profile>>,
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
            self.credits_available = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for credits_available: {}",
                    e
                )
            });
            self
        }
        pub fn criteria<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Criteria>,
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
        pub fn endorsement<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::EndorsementCredential>>,
            T::Error: std::fmt::Display,
        {
            self.endorsement = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for endorsement: {}", e));
            self
        }
        pub fn endorsement_jwt<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::AchievementEndorsementJwtItem>>,
            T::Error: std::fmt::Display,
        {
            self.endorsement_jwt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for endorsement_jwt: {}", e));
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
            T: std::convert::TryInto<Option<super::Image>>,
            T::Error: std::fmt::Display,
        {
            self.image = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image: {}", e));
            self
        }
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AchievementLanguage>>,
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
        pub fn other_identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::IdentifierEntry>>,
            T::Error: std::fmt::Display,
        {
            self.other_identifier = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for other_identifier: {}",
                    e
                )
            });
            self
        }
        pub fn related<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Related>>,
            T::Error: std::fmt::Display,
        {
            self.related = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for related: {}", e));
            self
        }
        pub fn result_description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::ResultDescription>>,
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
        pub fn tag<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.tag = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tag: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AchievementType>,
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
    impl std::convert::TryFrom<Achievement> for super::Achievement {
        type Error = String;
        fn try_from(value: Achievement) -> Result<Self, String> {
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
    impl From<super::Achievement> for Achievement {
        fn from(value: super::Achievement) -> Self {
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
    #[derive(Clone, Debug)]
    pub struct AchievementAchievementType {
        subtype_0: Result<Option<super::AchievementAchievementTypeSubtype0>, String>,
        subtype_1: Result<Option<super::AchievementAchievementTypeSubtype1>, String>,
    }
    impl Default for AchievementAchievementType {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl AchievementAchievementType {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AchievementAchievementTypeSubtype0>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AchievementAchievementTypeSubtype1>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AchievementAchievementType> for super::AchievementAchievementType {
        type Error = String;
        fn try_from(value: AchievementAchievementType) -> Result<Self, String> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl From<super::AchievementAchievementType> for AchievementAchievementType {
        fn from(value: super::AchievementAchievementType) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AchievementCredential {
        awarded_date: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        context: Result<Vec<super::Context>, String>,
        credential_schema: Result<Option<super::AchievementCredentialCredentialSchema>, String>,
        credential_status: Result<Option<super::CredentialStatus>, String>,
        credential_subject: Result<super::AchievementSubject, String>,
        description: Result<Option<String>, String>,
        endorsement: Result<Vec<super::EndorsementCredential>, String>,
        endorsement_jwt: Result<Vec<super::AchievementCredentialEndorsementJwtItem>, String>,
        evidence: Result<Vec<super::Evidence>, String>,
        expiration_date: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        id: Result<String, String>,
        image: Result<Option<super::Image>, String>,
        issuance_date: Result<chrono::DateTime<chrono::offset::Utc>, String>,
        issuer: Result<super::Profile, String>,
        name: Result<String, String>,
        proof: Result<Option<super::AchievementCredentialProof>, String>,
        refresh_service: Result<Option<super::RefreshService>, String>,
        terms_of_use: Result<Option<super::AchievementCredentialTermsOfUse>, String>,
        type_: Result<super::AchievementCredentialType, String>,
    }
    impl Default for AchievementCredential {
        fn default() -> Self {
            Self {
                awarded_date: Ok(Default::default()),
                context: Err("no value supplied for context".to_string()),
                credential_schema: Ok(Default::default()),
                credential_status: Ok(Default::default()),
                credential_subject: Err("no value supplied for credential_subject".to_string()),
                description: Ok(Default::default()),
                endorsement: Ok(Default::default()),
                endorsement_jwt: Ok(Default::default()),
                evidence: Ok(Default::default()),
                expiration_date: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                image: Ok(Default::default()),
                issuance_date: Err("no value supplied for issuance_date".to_string()),
                issuer: Err("no value supplied for issuer".to_string()),
                name: Err("no value supplied for name".to_string()),
                proof: Ok(Default::default()),
                refresh_service: Ok(Default::default()),
                terms_of_use: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AchievementCredential {
        pub fn awarded_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self.awarded_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for awarded_date: {}", e));
            self
        }
        pub fn context<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Context>>,
            T::Error: std::fmt::Display,
        {
            self.context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for context: {}", e));
            self
        }
        pub fn credential_schema<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AchievementCredentialCredentialSchema>>,
            T::Error: std::fmt::Display,
        {
            self.credential_schema = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for credential_schema: {}",
                    e
                )
            });
            self
        }
        pub fn credential_status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CredentialStatus>>,
            T::Error: std::fmt::Display,
        {
            self.credential_status = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for credential_status: {}",
                    e
                )
            });
            self
        }
        pub fn credential_subject<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AchievementSubject>,
            T::Error: std::fmt::Display,
        {
            self.credential_subject = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for credential_subject: {}",
                    e
                )
            });
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
        pub fn endorsement<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::EndorsementCredential>>,
            T::Error: std::fmt::Display,
        {
            self.endorsement = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for endorsement: {}", e));
            self
        }
        pub fn endorsement_jwt<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::AchievementCredentialEndorsementJwtItem>>,
            T::Error: std::fmt::Display,
        {
            self.endorsement_jwt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for endorsement_jwt: {}", e));
            self
        }
        pub fn evidence<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Evidence>>,
            T::Error: std::fmt::Display,
        {
            self.evidence = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for evidence: {}", e));
            self
        }
        pub fn expiration_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self.expiration_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expiration_date: {}", e));
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
            T: std::convert::TryInto<Option<super::Image>>,
            T::Error: std::fmt::Display,
        {
            self.image = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image: {}", e));
            self
        }
        pub fn issuance_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
            T::Error: std::fmt::Display,
        {
            self.issuance_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for issuance_date: {}", e));
            self
        }
        pub fn issuer<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Profile>,
            T::Error: std::fmt::Display,
        {
            self.issuer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for issuer: {}", e));
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
        pub fn proof<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AchievementCredentialProof>>,
            T::Error: std::fmt::Display,
        {
            self.proof = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for proof: {}", e));
            self
        }
        pub fn refresh_service<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::RefreshService>>,
            T::Error: std::fmt::Display,
        {
            self.refresh_service = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for refresh_service: {}", e));
            self
        }
        pub fn terms_of_use<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AchievementCredentialTermsOfUse>>,
            T::Error: std::fmt::Display,
        {
            self.terms_of_use = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for terms_of_use: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AchievementCredentialType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AchievementCredential> for super::AchievementCredential {
        type Error = String;
        fn try_from(value: AchievementCredential) -> Result<Self, String> {
            Ok(Self {
                awarded_date: value.awarded_date?,
                context: value.context?,
                credential_schema: value.credential_schema?,
                credential_status: value.credential_status?,
                credential_subject: value.credential_subject?,
                description: value.description?,
                endorsement: value.endorsement?,
                endorsement_jwt: value.endorsement_jwt?,
                evidence: value.evidence?,
                expiration_date: value.expiration_date?,
                id: value.id?,
                image: value.image?,
                issuance_date: value.issuance_date?,
                issuer: value.issuer?,
                name: value.name?,
                proof: value.proof?,
                refresh_service: value.refresh_service?,
                terms_of_use: value.terms_of_use?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::AchievementCredential> for AchievementCredential {
        fn from(value: super::AchievementCredential) -> Self {
            Self {
                awarded_date: Ok(value.awarded_date),
                context: Ok(value.context),
                credential_schema: Ok(value.credential_schema),
                credential_status: Ok(value.credential_status),
                credential_subject: Ok(value.credential_subject),
                description: Ok(value.description),
                endorsement: Ok(value.endorsement),
                endorsement_jwt: Ok(value.endorsement_jwt),
                evidence: Ok(value.evidence),
                expiration_date: Ok(value.expiration_date),
                id: Ok(value.id),
                image: Ok(value.image),
                issuance_date: Ok(value.issuance_date),
                issuer: Ok(value.issuer),
                name: Ok(value.name),
                proof: Ok(value.proof),
                refresh_service: Ok(value.refresh_service),
                terms_of_use: Ok(value.terms_of_use),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AchievementSubject {
        achievement: Result<super::Achievement, String>,
        activity_end_date: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        activity_start_date: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        credits_earned: Result<Option<f64>, String>,
        id: Result<Option<String>, String>,
        identifier: Result<Vec<super::IdentityObject>, String>,
        image: Result<Option<super::Image>, String>,
        license_number: Result<Option<String>, String>,
        narrative: Result<Option<String>, String>,
        result: Result<Vec<super::ResultTemp>, String>,
        role: Result<Option<String>, String>,
        source: Result<Option<super::Profile>, String>,
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
            T: std::convert::TryInto<super::Achievement>,
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
            T: std::convert::TryInto<Vec<super::IdentityObject>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for identifier: {}", e));
            self
        }
        pub fn image<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Image>>,
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
            T: std::convert::TryInto<Vec<super::ResultTemp>>,
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
            T: std::convert::TryInto<Option<super::Profile>>,
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
    #[derive(Clone, Debug)]
    pub struct Address {
        address_country: Result<Option<String>, String>,
        address_country_code: Result<Option<String>, String>,
        address_locality: Result<Option<String>, String>,
        address_region: Result<Option<String>, String>,
        geo: Result<Option<super::GeoCoordinates>, String>,
        post_office_box_number: Result<Option<String>, String>,
        postal_code: Result<Option<String>, String>,
        street_address: Result<Option<String>, String>,
        type_: Result<super::AddressType, String>,
    }
    impl Default for Address {
        fn default() -> Self {
            Self {
                address_country: Ok(Default::default()),
                address_country_code: Ok(Default::default()),
                address_locality: Ok(Default::default()),
                address_region: Ok(Default::default()),
                geo: Ok(Default::default()),
                post_office_box_number: Ok(Default::default()),
                postal_code: Ok(Default::default()),
                street_address: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Address {
        pub fn address_country<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.address_country = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for address_country: {}", e));
            self
        }
        pub fn address_country_code<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.address_country_code = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for address_country_code: {}",
                    e
                )
            });
            self
        }
        pub fn address_locality<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.address_locality = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for address_locality: {}",
                    e
                )
            });
            self
        }
        pub fn address_region<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.address_region = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for address_region: {}", e));
            self
        }
        pub fn geo<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GeoCoordinates>>,
            T::Error: std::fmt::Display,
        {
            self.geo = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for geo: {}", e));
            self
        }
        pub fn post_office_box_number<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.post_office_box_number = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for post_office_box_number: {}",
                    e
                )
            });
            self
        }
        pub fn postal_code<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.postal_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for postal_code: {}", e));
            self
        }
        pub fn street_address<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.street_address = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for street_address: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AddressType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Address> for super::Address {
        type Error = String;
        fn try_from(value: Address) -> Result<Self, String> {
            Ok(Self {
                address_country: value.address_country?,
                address_country_code: value.address_country_code?,
                address_locality: value.address_locality?,
                address_region: value.address_region?,
                geo: value.geo?,
                post_office_box_number: value.post_office_box_number?,
                postal_code: value.postal_code?,
                street_address: value.street_address?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Address> for Address {
        fn from(value: super::Address) -> Self {
            Self {
                address_country: Ok(value.address_country),
                address_country_code: Ok(value.address_country_code),
                address_locality: Ok(value.address_locality),
                address_region: Ok(value.address_region),
                geo: Ok(value.geo),
                post_office_box_number: Ok(value.post_office_box_number),
                postal_code: Ok(value.postal_code),
                street_address: Ok(value.street_address),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Alignment {
        target_code: Result<Option<String>, String>,
        target_description: Result<Option<String>, String>,
        target_framework: Result<Option<String>, String>,
        target_name: Result<String, String>,
        target_type: Result<Option<super::AlignmentTargetType>, String>,
        target_url: Result<String, String>,
        type_: Result<super::AlignmentType, String>,
    }
    impl Default for Alignment {
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
    impl Alignment {
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
            self.target_description = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for target_description: {}",
                    e
                )
            });
            self
        }
        pub fn target_framework<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.target_framework = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for target_framework: {}",
                    e
                )
            });
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
            T: std::convert::TryInto<Option<super::AlignmentTargetType>>,
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
            T: std::convert::TryInto<super::AlignmentType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Alignment> for super::Alignment {
        type Error = String;
        fn try_from(value: Alignment) -> Result<Self, String> {
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
    impl From<super::Alignment> for Alignment {
        fn from(value: super::Alignment) -> Self {
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
    #[derive(Clone, Debug)]
    pub struct AlignmentTargetType {
        subtype_0: Result<Option<super::AlignmentTargetTypeSubtype0>, String>,
        subtype_1: Result<Option<super::AlignmentTargetTypeSubtype1>, String>,
    }
    impl Default for AlignmentTargetType {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl AlignmentTargetType {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AlignmentTargetTypeSubtype0>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AlignmentTargetTypeSubtype1>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AlignmentTargetType> for super::AlignmentTargetType {
        type Error = String;
        fn try_from(value: AlignmentTargetType) -> Result<Self, String> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl From<super::AlignmentTargetType> for AlignmentTargetType {
        fn from(value: super::AlignmentTargetType) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CredentialSchema {
        id: Result<String, String>,
        type_: Result<String, String>,
    }
    impl Default for CredentialSchema {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl CredentialSchema {
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
    impl std::convert::TryFrom<CredentialSchema> for super::CredentialSchema {
        type Error = String;
        fn try_from(value: CredentialSchema) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::CredentialSchema> for CredentialSchema {
        fn from(value: super::CredentialSchema) -> Self {
            Self {
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CredentialStatus {
        id: Result<String, String>,
        type_: Result<String, String>,
    }
    impl Default for CredentialStatus {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl CredentialStatus {
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
    impl std::convert::TryFrom<CredentialStatus> for super::CredentialStatus {
        type Error = String;
        fn try_from(value: CredentialStatus) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::CredentialStatus> for CredentialStatus {
        fn from(value: super::CredentialStatus) -> Self {
            Self {
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Criteria {
        id: Result<Option<String>, String>,
        narrative: Result<Option<String>, String>,
    }
    impl Default for Criteria {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                narrative: Ok(Default::default()),
            }
        }
    }
    impl Criteria {
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
    impl std::convert::TryFrom<Criteria> for super::Criteria {
        type Error = String;
        fn try_from(value: Criteria) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                narrative: value.narrative?,
            })
        }
    }
    impl From<super::Criteria> for Criteria {
        fn from(value: super::Criteria) -> Self {
            Self {
                id: Ok(value.id),
                narrative: Ok(value.narrative),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EndorsementCredential {
        awarded_date: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        context: Result<Vec<super::Context>, String>,
        credential_schema: Result<Option<super::EndorsementCredentialCredentialSchema>, String>,
        credential_status: Result<Option<super::CredentialStatus>, String>,
        credential_subject: Result<super::EndorsementSubject, String>,
        description: Result<Option<String>, String>,
        expiration_date: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        id: Result<String, String>,
        issuance_date: Result<chrono::DateTime<chrono::offset::Utc>, String>,
        issuer: Result<super::Profile, String>,
        name: Result<String, String>,
        proof: Result<Option<super::EndorsementCredentialProof>, String>,
        refresh_service: Result<Option<super::RefreshService>, String>,
        terms_of_use: Result<Option<super::EndorsementCredentialTermsOfUse>, String>,
        type_: Result<super::EndorsementCredentialType, String>,
    }
    impl Default for EndorsementCredential {
        fn default() -> Self {
            Self {
                awarded_date: Ok(Default::default()),
                context: Err("no value supplied for context".to_string()),
                credential_schema: Ok(Default::default()),
                credential_status: Ok(Default::default()),
                credential_subject: Err("no value supplied for credential_subject".to_string()),
                description: Ok(Default::default()),
                expiration_date: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                issuance_date: Err("no value supplied for issuance_date".to_string()),
                issuer: Err("no value supplied for issuer".to_string()),
                name: Err("no value supplied for name".to_string()),
                proof: Ok(Default::default()),
                refresh_service: Ok(Default::default()),
                terms_of_use: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl EndorsementCredential {
        pub fn awarded_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self.awarded_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for awarded_date: {}", e));
            self
        }
        pub fn context<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Context>>,
            T::Error: std::fmt::Display,
        {
            self.context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for context: {}", e));
            self
        }
        pub fn credential_schema<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::EndorsementCredentialCredentialSchema>>,
            T::Error: std::fmt::Display,
        {
            self.credential_schema = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for credential_schema: {}",
                    e
                )
            });
            self
        }
        pub fn credential_status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CredentialStatus>>,
            T::Error: std::fmt::Display,
        {
            self.credential_status = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for credential_status: {}",
                    e
                )
            });
            self
        }
        pub fn credential_subject<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::EndorsementSubject>,
            T::Error: std::fmt::Display,
        {
            self.credential_subject = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for credential_subject: {}",
                    e
                )
            });
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
        pub fn expiration_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self.expiration_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expiration_date: {}", e));
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
        pub fn issuance_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
            T::Error: std::fmt::Display,
        {
            self.issuance_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for issuance_date: {}", e));
            self
        }
        pub fn issuer<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Profile>,
            T::Error: std::fmt::Display,
        {
            self.issuer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for issuer: {}", e));
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
        pub fn proof<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::EndorsementCredentialProof>>,
            T::Error: std::fmt::Display,
        {
            self.proof = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for proof: {}", e));
            self
        }
        pub fn refresh_service<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::RefreshService>>,
            T::Error: std::fmt::Display,
        {
            self.refresh_service = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for refresh_service: {}", e));
            self
        }
        pub fn terms_of_use<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::EndorsementCredentialTermsOfUse>>,
            T::Error: std::fmt::Display,
        {
            self.terms_of_use = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for terms_of_use: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::EndorsementCredentialType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<EndorsementCredential> for super::EndorsementCredential {
        type Error = String;
        fn try_from(value: EndorsementCredential) -> Result<Self, String> {
            Ok(Self {
                awarded_date: value.awarded_date?,
                context: value.context?,
                credential_schema: value.credential_schema?,
                credential_status: value.credential_status?,
                credential_subject: value.credential_subject?,
                description: value.description?,
                expiration_date: value.expiration_date?,
                id: value.id?,
                issuance_date: value.issuance_date?,
                issuer: value.issuer?,
                name: value.name?,
                proof: value.proof?,
                refresh_service: value.refresh_service?,
                terms_of_use: value.terms_of_use?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::EndorsementCredential> for EndorsementCredential {
        fn from(value: super::EndorsementCredential) -> Self {
            Self {
                awarded_date: Ok(value.awarded_date),
                context: Ok(value.context),
                credential_schema: Ok(value.credential_schema),
                credential_status: Ok(value.credential_status),
                credential_subject: Ok(value.credential_subject),
                description: Ok(value.description),
                expiration_date: Ok(value.expiration_date),
                id: Ok(value.id),
                issuance_date: Ok(value.issuance_date),
                issuer: Ok(value.issuer),
                name: Ok(value.name),
                proof: Ok(value.proof),
                refresh_service: Ok(value.refresh_service),
                terms_of_use: Ok(value.terms_of_use),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EndorsementSubject {
        endorsement_comment: Result<Option<String>, String>,
        id: Result<String, String>,
        type_: Result<super::EndorsementSubjectType, String>,
    }
    impl Default for EndorsementSubject {
        fn default() -> Self {
            Self {
                endorsement_comment: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl EndorsementSubject {
        pub fn endorsement_comment<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.endorsement_comment = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for endorsement_comment: {}",
                    e
                )
            });
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
            T: std::convert::TryInto<super::EndorsementSubjectType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<EndorsementSubject> for super::EndorsementSubject {
        type Error = String;
        fn try_from(value: EndorsementSubject) -> Result<Self, String> {
            Ok(Self {
                endorsement_comment: value.endorsement_comment?,
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::EndorsementSubject> for EndorsementSubject {
        fn from(value: super::EndorsementSubject) -> Self {
            Self {
                endorsement_comment: Ok(value.endorsement_comment),
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Evidence {
        audience: Result<Option<String>, String>,
        description: Result<Option<String>, String>,
        genre: Result<Option<String>, String>,
        id: Result<Option<String>, String>,
        name: Result<Option<String>, String>,
        narrative: Result<Option<String>, String>,
        type_: Result<super::EvidenceType, String>,
    }
    impl Default for Evidence {
        fn default() -> Self {
            Self {
                audience: Ok(Default::default()),
                description: Ok(Default::default()),
                genre: Ok(Default::default()),
                id: Ok(Default::default()),
                name: Ok(Default::default()),
                narrative: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Evidence {
        pub fn audience<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.audience = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audience: {}", e));
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
        pub fn genre<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.genre = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for genre: {}", e));
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
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::EvidenceType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Evidence> for super::Evidence {
        type Error = String;
        fn try_from(value: Evidence) -> Result<Self, String> {
            Ok(Self {
                audience: value.audience?,
                description: value.description?,
                genre: value.genre?,
                id: value.id?,
                name: value.name?,
                narrative: value.narrative?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Evidence> for Evidence {
        fn from(value: super::Evidence) -> Self {
            Self {
                audience: Ok(value.audience),
                description: Ok(value.description),
                genre: Ok(value.genre),
                id: Ok(value.id),
                name: Ok(value.name),
                narrative: Ok(value.narrative),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GeoCoordinates {
        latitude: Result<f64, String>,
        longitude: Result<f64, String>,
        type_: Result<String, String>,
    }
    impl Default for GeoCoordinates {
        fn default() -> Self {
            Self {
                latitude: Err("no value supplied for latitude".to_string()),
                longitude: Err("no value supplied for longitude".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl GeoCoordinates {
        pub fn latitude<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<f64>,
            T::Error: std::fmt::Display,
        {
            self.latitude = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for latitude: {}", e));
            self
        }
        pub fn longitude<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<f64>,
            T::Error: std::fmt::Display,
        {
            self.longitude = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for longitude: {}", e));
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
    impl std::convert::TryFrom<GeoCoordinates> for super::GeoCoordinates {
        type Error = String;
        fn try_from(value: GeoCoordinates) -> Result<Self, String> {
            Ok(Self {
                latitude: value.latitude?,
                longitude: value.longitude?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::GeoCoordinates> for GeoCoordinates {
        fn from(value: super::GeoCoordinates) -> Self {
            Self {
                latitude: Ok(value.latitude),
                longitude: Ok(value.longitude),
                type_: Ok(value.type_),
            }
        }
    }
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
    pub struct Profile {
        additional_name: Result<Option<String>, String>,
        address: Result<Option<super::Address>, String>,
        date_of_birth: Result<Option<chrono::NaiveDate>, String>,
        description: Result<Option<String>, String>,
        email: Result<Option<String>, String>,
        endorsement: Result<Vec<super::EndorsementCredential>, String>,
        endorsement_jwt: Result<Vec<super::ProfileEndorsementJwtItem>, String>,
        family_name: Result<Option<String>, String>,
        family_name_prefix: Result<Option<String>, String>,
        given_name: Result<Option<String>, String>,
        honorific_prefix: Result<Option<String>, String>,
        honorific_suffix: Result<Option<String>, String>,
        id: Result<String, String>,
        image: Result<Option<super::Image>, String>,
        name: Result<Option<String>, String>,
        official: Result<Option<String>, String>,
        other_identifier: Result<Vec<super::IdentifierEntry>, String>,
        parent_org: Result<Box<Option<super::Profile>>, String>,
        patronymic_name: Result<Option<String>, String>,
        phone: Result<Option<String>, String>,
        type_: Result<super::ProfileType, String>,
        url: Result<Option<String>, String>,
    }
    impl Default for Profile {
        fn default() -> Self {
            Self {
                additional_name: Ok(Default::default()),
                address: Ok(Default::default()),
                date_of_birth: Ok(Default::default()),
                description: Ok(Default::default()),
                email: Ok(Default::default()),
                endorsement: Ok(Default::default()),
                endorsement_jwt: Ok(Default::default()),
                family_name: Ok(Default::default()),
                family_name_prefix: Ok(Default::default()),
                given_name: Ok(Default::default()),
                honorific_prefix: Ok(Default::default()),
                honorific_suffix: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                image: Ok(Default::default()),
                name: Ok(Default::default()),
                official: Ok(Default::default()),
                other_identifier: Ok(Default::default()),
                parent_org: Ok(Default::default()),
                patronymic_name: Ok(Default::default()),
                phone: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                url: Ok(Default::default()),
            }
        }
    }
    impl Profile {
        pub fn additional_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.additional_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for additional_name: {}", e));
            self
        }
        pub fn address<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Address>>,
            T::Error: std::fmt::Display,
        {
            self.address = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for address: {}", e));
            self
        }
        pub fn date_of_birth<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::NaiveDate>>,
            T::Error: std::fmt::Display,
        {
            self.date_of_birth = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date_of_birth: {}", e));
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
        pub fn email<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.email = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for email: {}", e));
            self
        }
        pub fn endorsement<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::EndorsementCredential>>,
            T::Error: std::fmt::Display,
        {
            self.endorsement = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for endorsement: {}", e));
            self
        }
        pub fn endorsement_jwt<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::ProfileEndorsementJwtItem>>,
            T::Error: std::fmt::Display,
        {
            self.endorsement_jwt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for endorsement_jwt: {}", e));
            self
        }
        pub fn family_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.family_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for family_name: {}", e));
            self
        }
        pub fn family_name_prefix<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.family_name_prefix = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for family_name_prefix: {}",
                    e
                )
            });
            self
        }
        pub fn given_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.given_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for given_name: {}", e));
            self
        }
        pub fn honorific_prefix<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.honorific_prefix = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for honorific_prefix: {}",
                    e
                )
            });
            self
        }
        pub fn honorific_suffix<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.honorific_suffix = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for honorific_suffix: {}",
                    e
                )
            });
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
            T: std::convert::TryInto<Option<super::Image>>,
            T::Error: std::fmt::Display,
        {
            self.image = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn official<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.official = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for official: {}", e));
            self
        }
        pub fn other_identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::IdentifierEntry>>,
            T::Error: std::fmt::Display,
        {
            self.other_identifier = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for other_identifier: {}",
                    e
                )
            });
            self
        }
        pub fn parent_org<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<Option<super::Profile>>>,
            T::Error: std::fmt::Display,
        {
            self.parent_org = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parent_org: {}", e));
            self
        }
        pub fn patronymic_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.patronymic_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for patronymic_name: {}", e));
            self
        }
        pub fn phone<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.phone = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for phone: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ProfileType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Profile> for super::Profile {
        type Error = String;
        fn try_from(value: Profile) -> Result<Self, String> {
            Ok(Self {
                additional_name: value.additional_name?,
                address: value.address?,
                date_of_birth: value.date_of_birth?,
                description: value.description?,
                email: value.email?,
                endorsement: value.endorsement?,
                endorsement_jwt: value.endorsement_jwt?,
                family_name: value.family_name?,
                family_name_prefix: value.family_name_prefix?,
                given_name: value.given_name?,
                honorific_prefix: value.honorific_prefix?,
                honorific_suffix: value.honorific_suffix?,
                id: value.id?,
                image: value.image?,
                name: value.name?,
                official: value.official?,
                other_identifier: value.other_identifier?,
                parent_org: value.parent_org?,
                patronymic_name: value.patronymic_name?,
                phone: value.phone?,
                type_: value.type_?,
                url: value.url?,
            })
        }
    }
    impl From<super::Profile> for Profile {
        fn from(value: super::Profile) -> Self {
            Self {
                additional_name: Ok(value.additional_name),
                address: Ok(value.address),
                date_of_birth: Ok(value.date_of_birth),
                description: Ok(value.description),
                email: Ok(value.email),
                endorsement: Ok(value.endorsement),
                endorsement_jwt: Ok(value.endorsement_jwt),
                family_name: Ok(value.family_name),
                family_name_prefix: Ok(value.family_name_prefix),
                given_name: Ok(value.given_name),
                honorific_prefix: Ok(value.honorific_prefix),
                honorific_suffix: Ok(value.honorific_suffix),
                id: Ok(value.id),
                image: Ok(value.image),
                name: Ok(value.name),
                official: Ok(value.official),
                other_identifier: Ok(value.other_identifier),
                parent_org: Ok(value.parent_org),
                patronymic_name: Ok(value.patronymic_name),
                phone: Ok(value.phone),
                type_: Ok(value.type_),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Proof {
        challenge: Result<Option<String>, String>,
        created: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        cryptosuite: Result<Option<String>, String>,
        domain: Result<Option<String>, String>,
        nonce: Result<Option<String>, String>,
        proof_purpose: Result<Option<String>, String>,
        proof_value: Result<Option<String>, String>,
        type_: Result<String, String>,
        verification_method: Result<Option<String>, String>,
    }
    impl Default for Proof {
        fn default() -> Self {
            Self {
                challenge: Ok(Default::default()),
                created: Ok(Default::default()),
                cryptosuite: Ok(Default::default()),
                domain: Ok(Default::default()),
                nonce: Ok(Default::default()),
                proof_purpose: Ok(Default::default()),
                proof_value: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                verification_method: Ok(Default::default()),
            }
        }
    }
    impl Proof {
        pub fn challenge<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.challenge = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for challenge: {}", e));
            self
        }
        pub fn created<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self.created = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created: {}", e));
            self
        }
        pub fn cryptosuite<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.cryptosuite = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cryptosuite: {}", e));
            self
        }
        pub fn domain<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.domain = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for domain: {}", e));
            self
        }
        pub fn nonce<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.nonce = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nonce: {}", e));
            self
        }
        pub fn proof_purpose<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.proof_purpose = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for proof_purpose: {}", e));
            self
        }
        pub fn proof_value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.proof_value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for proof_value: {}", e));
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
        pub fn verification_method<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.verification_method = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for verification_method: {}",
                    e
                )
            });
            self
        }
    }
    impl std::convert::TryFrom<Proof> for super::Proof {
        type Error = String;
        fn try_from(value: Proof) -> Result<Self, String> {
            Ok(Self {
                challenge: value.challenge?,
                created: value.created?,
                cryptosuite: value.cryptosuite?,
                domain: value.domain?,
                nonce: value.nonce?,
                proof_purpose: value.proof_purpose?,
                proof_value: value.proof_value?,
                type_: value.type_?,
                verification_method: value.verification_method?,
            })
        }
    }
    impl From<super::Proof> for Proof {
        fn from(value: super::Proof) -> Self {
            Self {
                challenge: Ok(value.challenge),
                created: Ok(value.created),
                cryptosuite: Ok(value.cryptosuite),
                domain: Ok(value.domain),
                nonce: Ok(value.nonce),
                proof_purpose: Ok(value.proof_purpose),
                proof_value: Ok(value.proof_value),
                type_: Ok(value.type_),
                verification_method: Ok(value.verification_method),
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
    #[derive(Clone, Debug)]
    pub struct ResultDescription {
        alignment: Result<Vec<super::Alignment>, String>,
        allowed_value: Result<Vec<String>, String>,
        id: Result<String, String>,
        name: Result<String, String>,
        required_level: Result<Option<String>, String>,
        required_value: Result<Option<String>, String>,
        result_type: Result<super::ResultDescriptionResultType, String>,
        rubric_criterion_level: Result<Vec<super::RubricCriterionLevel>, String>,
        type_: Result<super::ResultDescriptionType, String>,
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
            T: std::convert::TryInto<Vec<super::Alignment>>,
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
            T: std::convert::TryInto<super::ResultDescriptionResultType>,
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
            T: std::convert::TryInto<super::ResultDescriptionType>,
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
    impl std::convert::TryFrom<ResultDescriptionResultType> for super::ResultDescriptionResultType {
        type Error = String;
        fn try_from(value: ResultDescriptionResultType) -> Result<Self, String> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl From<super::ResultDescriptionResultType> for ResultDescriptionResultType {
        fn from(value: super::ResultDescriptionResultType) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResultTemp {
        achieved_level: Result<Option<String>, String>,
        alignment: Result<Vec<super::Alignment>, String>,
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
            T: std::convert::TryInto<Vec<super::Alignment>>,
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
        alignment: Result<Vec<super::Alignment>, String>,
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
            T: std::convert::TryInto<Vec<super::Alignment>>,
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
