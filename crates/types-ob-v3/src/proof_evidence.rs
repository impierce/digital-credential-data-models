use serde::{Deserialize, Serialize};
use types_common::{GenPaths, SchemaList};

use crate::endorsement::EndorsementCredentialProof;

#[doc = "Descriptive metadata about evidence related to the achievement assertion. Each instance of the evidence class present in an assertion corresponds to one entity, though a single entry can describe a set of items collectively. There may be multiple evidence entries referenced from an assertion. The narrative property is also in scope of the assertion class to provide an overall description of the achievement related to the assertion in rich text. It is used here to provide a narrative of achievement of the specific entity described. If both the description and narrative properties are present, displayers can assume the narrative value goes into more detail and is not simply a recapitulation of description."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
pub struct Evidence {
    #[doc = "The URL of a webpage presenting evidence of achievement or the evidence encoded as a Data URI. The schema of the webpage is undefined."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_: EvidenceType,
    #[doc = "A narrative that describes the evidence and process of achievement that led to an assertion."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrative: Option<String>,
    #[doc = "A descriptive title of the evidence."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A longer description of the evidence."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A string that describes the type of evidence. For example, Poetry, Prose, Film."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    #[doc = "A description of the intended audience for a piece of evidence."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
}
impl From<&Evidence> for Evidence {
    fn from(value: &Evidence) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
#[serde(untagged)]
pub enum EvidenceType {
    String(String),
    VecString(Vec<String>),
}
impl From<&EvidenceType> for EvidenceType {
    fn from(value: &EvidenceType) -> Self {
        value.clone()
    }
}
impl From<String> for EvidenceType {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
impl From<&str> for EvidenceType {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}
impl From<Vec<String>> for EvidenceType {
    fn from(value: Vec<String>) -> Self {
        Self::VecString(value)
    }
}
impl From<Vec<&str>> for EvidenceType {
    fn from(value: Vec<&str>) -> Self {
        let v = value.iter().map(|v| v.to_string()).collect();
        Self::VecString(v)
    }
}

#[doc = "A JSON-LD Linked Data proof."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, GenPaths)]
pub struct Proof {
    #[doc = "Signature suite used to produce proof."]
    #[serde(rename = "type")]
    pub type_: String,
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
    #[serde(rename = "proofPurpose", default, skip_serializing_if = "Option::is_none")]
    pub proof_purpose: Option<String>,
    #[doc = "Value of the proof."]
    #[serde(rename = "proofValue", default, skip_serializing_if = "Option::is_none")]
    pub proof_value: Option<String>,
    #[doc = "The URL of the public key that can verify the signature."]
    #[serde(rename = "verificationMethod", default, skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<String>,
}
impl From<&Proof> for Proof {
    fn from(value: &Proof) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvidenceBuilder {
    audience: Result<Option<String>, String>,
    description: Result<Option<String>, String>,
    genre: Result<Option<String>, String>,
    id: Result<Option<String>, String>,
    name: Result<Option<String>, String>,
    narrative: Result<Option<String>, String>,
    type_: Result<EvidenceType, String>,
}
impl Default for EvidenceBuilder {
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
impl EvidenceBuilder {
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
        T: std::convert::TryInto<EvidenceType>,
        T::Error: std::fmt::Display,
    {
        self.type_ = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for type_: {}", e));
        self
    }
}
impl std::convert::TryFrom<EvidenceBuilder> for Evidence {
    type Error = String;
    fn try_from(value: EvidenceBuilder) -> Result<Self, String> {
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
impl From<Evidence> for EvidenceBuilder {
    fn from(value: Evidence) -> Self {
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

#[derive(Clone, Debug, PartialEq)]
pub struct ProofBuilder {
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
impl Default for ProofBuilder {
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
impl ProofBuilder {
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
        T: AsRef<str>,
    {
        self.created = value
            .as_ref()
            .parse::<chrono::DateTime<chrono::offset::Utc>>()
            .map(Some)
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
        self.verification_method = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for verification_method: {}", e));
        self
    }
}
impl std::convert::TryFrom<ProofBuilder> for Proof {
    type Error = String;
    fn try_from(value: ProofBuilder) -> Result<Self, String> {
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
impl std::convert::TryFrom<ProofBuilder> for Option<EndorsementCredentialProof> {
    type Error = String;
    fn try_from(value: ProofBuilder) -> Result<Self, String> {
        Ok(Some(EndorsementCredentialProof::Proof(Proof {
            challenge: value.challenge?,
            created: value.created?,
            cryptosuite: value.cryptosuite?,
            domain: value.domain?,
            nonce: value.nonce?,
            proof_purpose: value.proof_purpose?,
            proof_value: value.proof_value?,
            type_: value.type_?,
            verification_method: value.verification_method?,
        })))
    }
}
impl From<Proof> for ProofBuilder {
    fn from(value: Proof) -> Self {
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
