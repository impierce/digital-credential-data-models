#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};
use super::{achievement_credential, general, profile, proof_evidence};


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
    pub context: Vec<general::Context>,
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
    pub credential_status: Option<achievement_credential::CredentialStatus>,
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
    pub issuer: profile::Profile,
    #[doc = "The name of the credential for display purposes in wallets. For example, in a list of credentials and in detail views."]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof: Option<EndorsementCredentialProof>,
    #[serde(
        rename = "refreshService",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub refresh_service: Option<general::RefreshService>,
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
    Variant0(achievement_credential::CredentialSchema),
    Variant1(Vec<achievement_credential::CredentialSchema>),
}
impl From<&EndorsementCredentialCredentialSchema> for EndorsementCredentialCredentialSchema {
    fn from(value: &EndorsementCredentialCredentialSchema) -> Self {
        value.clone()
    }
}
impl From<achievement_credential::CredentialSchema> for EndorsementCredentialCredentialSchema {
    fn from(value: achievement_credential::CredentialSchema) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<achievement_credential::CredentialSchema>> for EndorsementCredentialCredentialSchema {
    fn from(value: Vec<achievement_credential::CredentialSchema>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EndorsementCredentialProof {
    Variant0(proof_evidence::Proof),
    Variant1(Vec<proof_evidence::Proof>),
}
impl From<&EndorsementCredentialProof> for EndorsementCredentialProof {
    fn from(value: &EndorsementCredentialProof) -> Self {
        value.clone()
    }
}
impl From<proof_evidence::Proof> for EndorsementCredentialProof {
    fn from(value: proof_evidence::Proof) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<proof_evidence::Proof>> for EndorsementCredentialProof {
    fn from(value: Vec<proof_evidence::Proof>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EndorsementCredentialTermsOfUse {
    Variant0(general::TermsOfUse),
    Variant1(Vec<general::TermsOfUse>),
}
impl From<&EndorsementCredentialTermsOfUse> for EndorsementCredentialTermsOfUse {
    fn from(value: &EndorsementCredentialTermsOfUse) -> Self {
        value.clone()
    }
}
impl From<general::TermsOfUse> for EndorsementCredentialTermsOfUse {
    fn from(value: general::TermsOfUse) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<general::TermsOfUse>> for EndorsementCredentialTermsOfUse {
    fn from(value: Vec<general::TermsOfUse>) -> Self {
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

pub mod builder {
    use crate::json_schema::{general, profile, achievement_credential};

    
    #[derive(Clone, Debug)]
    pub struct EndorsementCredential {
        awarded_date: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        context: Result<Vec<general::Context>, String>,
        credential_schema: Result<Option<super::EndorsementCredentialCredentialSchema>, String>,
        credential_status: Result<Option<achievement_credential::CredentialStatus>, String>,
        credential_subject: Result<super::EndorsementSubject, String>,
        description: Result<Option<String>, String>,
        expiration_date: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        id: Result<String, String>,
        issuance_date: Result<chrono::DateTime<chrono::offset::Utc>, String>,
        issuer: Result<profile::Profile, String>,
        name: Result<String, String>,
        proof: Result<Option<super::EndorsementCredentialProof>, String>,
        refresh_service: Result<Option<general::RefreshService>, String>,
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
            T: std::convert::TryInto<Vec<general::Context>>,
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
            T: std::convert::TryInto<Option<achievement_credential::CredentialStatus>>,
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
}
