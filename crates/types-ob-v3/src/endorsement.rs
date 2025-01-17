use super::{achievement_credential, general, profile, proof_evidence};
use serde::{Deserialize, Serialize};
use types_common::{GenPaths, SchemaList};

#[doc = "A verifiable credential that asserts a claim about an entity. As described in [[[#data-integrity]]], at least one proof mechanism, and the details necessary to evaluate that proof, MUST be expressed for a credential to be a verifiable credential. In the case of an embedded proof, the credential MUST append the proof in the `proof` property."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
pub struct EndorsementCredential {
    #[serde(rename = "@context")]
    pub context: Vec<general::Context>,
    #[serde(rename = "type")]
    pub type_: EndorsementCredentialType,
    #[doc = "Unambiguous reference to the credential."]
    pub id: String,
    #[doc = "The name of the credential for display purposes in wallets. For example, in a list of credentials and in detail views."]
    pub name: String,
    #[doc = "The short description of the credential for display purposes in wallets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "credentialSubject")]
    pub credential_subject: EndorsementSubject,
    pub issuer: profile::Profile,
    #[doc = "Timestamp of when the credential was issued."]
    #[serde(rename = "issuanceDate")]
    pub issuance_date: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "If the credential has some notion of expiry, this indicates a timestamp when a credential should no longer be considered valid. After this time, the credential should be considered expired."]
    #[serde(rename = "expirationDate", default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "Timestamp of when the credential was awarded. `issuanceDate` is used to determine the most recent version of a Credential in conjunction with `issuer` and `id`. Consequently, the only way to update a Credental is to update the `issuanceDate`, losing the date when the Credential was originally awarded. `awardedDate` is meant to keep this original date."]
    #[serde(rename = "awardedDate", default, skip_serializing_if = "Option::is_none")]
    pub awarded_date: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof: Option<EndorsementCredentialProof>,
    #[serde(rename = "credentialSchema", default, skip_serializing_if = "Option::is_none")]
    pub credential_schema: Option<EndorsementCredentialSchema>,
    #[serde(rename = "credentialStatus", default, skip_serializing_if = "Option::is_none")]
    pub credential_status: Option<achievement_credential::CredentialStatus>,
    #[serde(rename = "refreshService", default, skip_serializing_if = "Option::is_none")]
    pub refresh_service: Option<general::RefreshService>,
    #[serde(rename = "termsOfUse", default, skip_serializing_if = "Option::is_none")]
    pub terms_of_use: Option<EndorsementCredentialTermsOfUse>,
}

impl From<&EndorsementCredential> for EndorsementCredential {
    fn from(value: &EndorsementCredential) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
#[serde(untagged)]
pub enum EndorsementCredentialSchema {
    Schema(achievement_credential::CredentialSchema),
    VecSchema(Vec<achievement_credential::CredentialSchema>),
}
impl From<&EndorsementCredentialSchema> for EndorsementCredentialSchema {
    fn from(value: &EndorsementCredentialSchema) -> Self {
        value.clone()
    }
}
impl From<achievement_credential::CredentialSchema> for EndorsementCredentialSchema {
    fn from(value: achievement_credential::CredentialSchema) -> Self {
        Self::Schema(value)
    }
}
impl From<Vec<achievement_credential::CredentialSchema>> for EndorsementCredentialSchema {
    fn from(value: Vec<achievement_credential::CredentialSchema>) -> Self {
        Self::VecSchema(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
#[serde(untagged)]
pub enum EndorsementCredentialProof {
    Proof(proof_evidence::Proof),
    VecProof(Vec<proof_evidence::Proof>),
}
impl From<&EndorsementCredentialProof> for EndorsementCredentialProof {
    fn from(value: &EndorsementCredentialProof) -> Self {
        value.clone()
    }
}
impl From<proof_evidence::Proof> for EndorsementCredentialProof {
    fn from(value: proof_evidence::Proof) -> Self {
        Self::Proof(value)
    }
}
impl From<Vec<proof_evidence::Proof>> for EndorsementCredentialProof {
    fn from(value: Vec<proof_evidence::Proof>) -> Self {
        Self::VecProof(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
#[serde(untagged)]
pub enum EndorsementCredentialTermsOfUse {
    TermsOfUse(general::TermsOfUse),
    VecTermsOfUse(Vec<general::TermsOfUse>),
}
impl From<&EndorsementCredentialTermsOfUse> for EndorsementCredentialTermsOfUse {
    fn from(value: &EndorsementCredentialTermsOfUse) -> Self {
        value.clone()
    }
}
impl From<general::TermsOfUse> for EndorsementCredentialTermsOfUse {
    fn from(value: general::TermsOfUse) -> Self {
        Self::TermsOfUse(value)
    }
}
impl From<Vec<general::TermsOfUse>> for EndorsementCredentialTermsOfUse {
    fn from(value: Vec<general::TermsOfUse>) -> Self {
        Self::VecTermsOfUse(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
#[serde(untagged)]
pub enum EndorsementCredentialType {
    String(String),
    VecString(Vec<String>),
}
impl From<&EndorsementCredentialType> for EndorsementCredentialType {
    fn from(value: &EndorsementCredentialType) -> Self {
        value.clone()
    }
}
impl From<String> for EndorsementCredentialType {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
impl From<&str> for EndorsementCredentialType {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}
impl From<Vec<String>> for EndorsementCredentialType {
    fn from(value: Vec<String>) -> Self {
        Self::VecString(value)
    }
}
impl From<Vec<&str>> for EndorsementCredentialType {
    fn from(value: Vec<&str>) -> Self {
        let v = value.iter().map(|v| v.to_string()).collect();
        Self::VecString(v)
    }
}
#[doc = "A collection of information about the subject of the endorsement."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
pub struct EndorsementSubject {
    #[doc = "The identifier of the individual, entity, organization, assertion, or achievement that is endorsed."]
    pub id: String,
    #[serde(rename = "type")]
    pub type_: EndorsementSubjectType,
    #[doc = "Allows endorsers to make a simple claim in writing about the entity."]
    #[serde(rename = "endorsementComment", default, skip_serializing_if = "Option::is_none")]
    pub endorsement_comment: Option<String>,
}
impl From<&EndorsementSubject> for EndorsementSubject {
    fn from(value: &EndorsementSubject) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
#[serde(untagged)]
pub enum EndorsementSubjectType {
    String(String),
    VecString(Vec<String>),
}
impl From<&EndorsementSubjectType> for EndorsementSubjectType {
    fn from(value: &EndorsementSubjectType) -> Self {
        value.clone()
    }
}
impl From<String> for EndorsementSubjectType {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
impl From<&str> for EndorsementSubjectType {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}
impl From<Vec<String>> for EndorsementSubjectType {
    fn from(value: Vec<String>) -> Self {
        Self::VecString(value)
    }
}
impl From<Vec<&str>> for EndorsementSubjectType {
    fn from(value: Vec<&str>) -> Self {
        let v = value.iter().map(|v| v.to_string()).collect();
        Self::VecString(v)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EndorsementCredentialBuilder {
    awarded_date: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
    context: Result<Vec<general::Context>, String>,
    credential_schema: Result<Option<EndorsementCredentialSchema>, String>,
    credential_status: Result<Option<achievement_credential::CredentialStatus>, String>,
    credential_subject: Result<EndorsementSubject, String>,
    description: Result<Option<String>, String>,
    expiration_date: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
    id: Result<String, String>,
    issuance_date: Result<chrono::DateTime<chrono::offset::Utc>, String>,
    issuer: Result<profile::Profile, String>,
    name: Result<String, String>,
    proof: Result<Option<EndorsementCredentialProof>, String>,
    refresh_service: Result<Option<general::RefreshService>, String>,
    terms_of_use: Result<Option<EndorsementCredentialTermsOfUse>, String>,
    type_: Result<EndorsementCredentialType, String>,
}
impl Default for EndorsementCredentialBuilder {
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
impl EndorsementCredentialBuilder {
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
    pub fn context<T>(mut self, value: Vec<T>) -> Self
    where
        T: std::convert::TryInto<general::Context>,
        T::Error: std::fmt::Display,
    {
        self.context = value
            .into_iter()
            .map(|value| {
                value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for context: {}", e))
            })
            .collect();
        self
    }
    pub fn credential_schema<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<EndorsementCredentialSchema>,
        T::Error: std::fmt::Display,
    {
        self.credential_schema = value
            .try_into()
            .map(Some)
            .map_err(|e| format!("error converting supplied value for credential_schema: {}", e));
        self
    }
    pub fn credential_status<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<achievement_credential::CredentialStatus>>,
        T::Error: std::fmt::Display,
    {
        self.credential_status = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for credential_status: {}", e));
        self
    }
    pub fn credential_subject<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<EndorsementSubject>,
        T::Error: std::fmt::Display,
    {
        self.credential_subject = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for credential_subject: {}", e));
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
        T: AsRef<str>,
    {
        self.expiration_date = value
            .as_ref()
            .parse::<chrono::DateTime<chrono::offset::Utc>>()
            .map(Some)
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
        T: AsRef<str>,
    {
        self.issuance_date = value
            .as_ref()
            .parse::<chrono::DateTime<chrono::offset::Utc>>()
            .map_err(|e| format!("error converting supplied value for awarded_date: {}", e));
        self
    }
    pub fn issuer<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<profile::Profile>,
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
        T: std::convert::TryInto<EndorsementCredentialProof>,
        T::Error: std::fmt::Display,
    {
        self.proof = value
            .try_into()
            .map(Some)
            .map_err(|e| format!("error converting supplied value for proof: {}", e));
        self
    }
    pub fn refresh_service<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<general::RefreshService>>,
        T::Error: std::fmt::Display,
    {
        self.refresh_service = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for refresh_service: {}", e));
        self
    }
    pub fn terms_of_use<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<EndorsementCredentialTermsOfUse>>,
        T::Error: std::fmt::Display,
    {
        self.terms_of_use = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for terms_of_use: {}", e));
        self
    }
    pub fn type_<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<EndorsementCredentialType>,
        T::Error: std::fmt::Display,
    {
        self.type_ = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for type_: {}", e));
        self
    }
}
impl std::convert::TryFrom<EndorsementCredentialBuilder> for EndorsementCredential {
    type Error = String;
    fn try_from(value: EndorsementCredentialBuilder) -> Result<Self, String> {
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
impl From<EndorsementCredential> for EndorsementCredentialBuilder {
    fn from(value: EndorsementCredential) -> Self {
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
#[derive(Clone, Debug, PartialEq)]
pub struct EndorsementSubjectBuilder {
    endorsement_comment: Result<Option<String>, String>,
    id: Result<String, String>,
    type_: Result<EndorsementSubjectType, String>,
}
impl Default for EndorsementSubjectBuilder {
    fn default() -> Self {
        Self {
            endorsement_comment: Ok(Default::default()),
            id: Err("no value supplied for id".to_string()),
            type_: Err("no value supplied for type_".to_string()),
        }
    }
}
impl EndorsementSubjectBuilder {
    pub fn endorsement_comment<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.endorsement_comment = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for endorsement_comment: {}", e));
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
        T: std::convert::TryInto<EndorsementSubjectType>,
        T::Error: std::fmt::Display,
    {
        self.type_ = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for type_: {}", e));
        self
    }
}
impl std::convert::TryFrom<EndorsementSubjectBuilder> for EndorsementSubject {
    type Error = String;
    fn try_from(value: EndorsementSubjectBuilder) -> Result<Self, String> {
        Ok(Self {
            endorsement_comment: value.endorsement_comment?,
            id: value.id?,
            type_: value.type_?,
        })
    }
}
impl From<EndorsementSubject> for EndorsementSubjectBuilder {
    fn from(value: EndorsementSubject) -> Self {
        Self {
            endorsement_comment: Ok(value.endorsement_comment),
            id: Ok(value.id),
            type_: Ok(value.type_),
        }
    }
}
