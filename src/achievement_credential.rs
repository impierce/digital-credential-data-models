use super::{achievement_subject, endorsement, general, profile, proof_evidence};
use serde::{Deserialize, Serialize};

#[doc = "AchievementCredentials are representations of an awarded achievement, used to share information about a achievement belonging to one earner. Maps to a Verifiable Credential as defined in the [[VC-DATA-MODEL]]. As described in [[[#data-integrity]]], at least one proof mechanism, and the details necessary to evaluate that proof, MUST be expressed for a credential to be a verifiable credential. In the case of an embedded proof, the credential MUST append the proof in the `proof` property."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AchievementCredential {
    #[serde(rename = "@context")]
    pub context: Vec<general::Context>,
    #[doc = "Unambiguous reference to the credential."]
    pub id: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<AchievementCredentialType>,
    #[doc = "The name of the credential for display purposes in wallets. For example, in a list of credentials and in detail views."]
    pub name: String,
    #[doc = "The short description of the credential for display purposes in wallets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<general::Image>,
    #[serde(rename = "credentialSubject")]
    pub credential_subject: achievement_subject::AchievementSubject,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endorsement: Vec<endorsement::EndorsementCredential>,
    #[serde(rename = "endorsementJwt", default, skip_serializing_if = "Vec::is_empty")]
    pub endorsement_jwt: Vec<AchievementCredentialEndorsementJwtItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<proof_evidence::Evidence>,
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
    pub proof: Option<AchievementCredentialProof>,
    #[serde(rename = "credentialSchema", default, skip_serializing_if = "Option::is_none")]
    pub credential_schema: Option<AchievementCredentialSchema>,
    #[serde(rename = "credentialStatus", default, skip_serializing_if = "Option::is_none")]
    pub credential_status: Option<CredentialStatus>,
    #[serde(rename = "refreshService", default, skip_serializing_if = "Option::is_none")]
    pub refresh_service: Option<general::RefreshService>,
    #[serde(rename = "termsOfUse", default, skip_serializing_if = "Option::is_none")]
    pub terms_of_use: Option<AchievementCredentialTermsOfUse>,
}

impl From<&AchievementCredential> for AchievementCredential {
    fn from(value: &AchievementCredential) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum AchievementCredentialSchema {
    Schema(CredentialSchema),
    VecSchema(Vec<CredentialSchema>),
}

impl From<&AchievementCredentialSchema> for AchievementCredentialSchema {
    fn from(value: &AchievementCredentialSchema) -> Self {
        value.clone()
    }
}
impl From<CredentialSchema> for AchievementCredentialSchema {
    fn from(value: CredentialSchema) -> Self {
        Self::Schema(value)
    }
}
impl From<CredentialSchema> for Option<AchievementCredentialSchema> {
    fn from(value: CredentialSchema) -> Self {
        Some(AchievementCredentialSchema::Schema(value))
    }
}
impl From<CredentialSchemaBuilder> for Option<AchievementCredentialSchema> {
    fn from(value: CredentialSchemaBuilder) -> Self {
        Some(AchievementCredentialSchema::Schema(value.try_into().unwrap()))
    }
}
impl From<Vec<CredentialSchema>> for AchievementCredentialSchema {
    fn from(value: Vec<CredentialSchema>) -> Self {
        Self::VecSchema(value)
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
            return Err("doesn't match pattern \"^[a-zA-Z0-9_-]+\\.[a-zA-Z0-9_-]*\\.[a-zA-Z0-9_-]+$\"");
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum AchievementCredentialProof {
    Proof(proof_evidence::Proof),
    VecProof(Vec<proof_evidence::Proof>),
}

impl From<&AchievementCredentialProof> for AchievementCredentialProof {
    fn from(value: &AchievementCredentialProof) -> Self {
        value.clone()
    }
}

impl From<proof_evidence::Proof> for AchievementCredentialProof {
    fn from(value: proof_evidence::Proof) -> Self {
        Self::Proof(value)
    }
}

impl From<Vec<proof_evidence::Proof>> for AchievementCredentialProof {
    fn from(value: Vec<proof_evidence::Proof>) -> Self {
        Self::VecProof(value)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum AchievementCredentialTermsOfUse {
    TermsOfUse(general::TermsOfUse),
    VecTermsOfUse(Vec<general::TermsOfUse>),
}

impl From<&AchievementCredentialTermsOfUse> for AchievementCredentialTermsOfUse {
    fn from(value: &AchievementCredentialTermsOfUse) -> Self {
        value.clone()
    }
}

impl From<general::TermsOfUse> for AchievementCredentialTermsOfUse {
    fn from(value: general::TermsOfUse) -> Self {
        Self::TermsOfUse(value)
    }
}

impl From<Vec<general::TermsOfUse>> for AchievementCredentialTermsOfUse {
    fn from(value: Vec<general::TermsOfUse>) -> Self {
        Self::VecTermsOfUse(value)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum AchievementCredentialType {
    String(String),
    VecString(Vec<String>),
}
impl From<&AchievementCredentialType> for AchievementCredentialType {
    fn from(value: &AchievementCredentialType) -> Self {
        value.clone()
    }
}
impl From<String> for AchievementCredentialType {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
impl From<&str> for AchievementCredentialType {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}
impl From<Vec<String>> for AchievementCredentialType {
    fn from(value: Vec<String>) -> Self {
        Self::VecString(value)
    }
}
impl From<Vec<&str>> for AchievementCredentialType {
    fn from(value: Vec<&str>) -> Self {
        let v = value.iter().map(|v| v.to_string()).collect();
        Self::VecString(v)
    }
}

#[doc = "Identify the type and location of a data schema."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
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

#[doc = "The information in CredentialStatus is used to discover information about the current status of a verifiable credential, such as whether it is suspended or revoked."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct AchievementCredentialBuilder {
    awarded_date: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
    context: Result<Vec<general::Context>, String>,
    credential_schema: Result<Option<AchievementCredentialSchema>, String>,
    credential_status: Result<Option<CredentialStatus>, String>,
    credential_subject: Result<achievement_subject::AchievementSubject, String>,
    description: Result<Option<String>, String>,
    endorsement: Result<Vec<endorsement::EndorsementCredential>, String>,
    endorsement_jwt: Result<Vec<AchievementCredentialEndorsementJwtItem>, String>,
    evidence: Result<Vec<proof_evidence::Evidence>, String>,
    expiration_date: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
    id: Result<String, String>,
    image: Result<Option<general::Image>, String>,
    issuance_date: Result<chrono::DateTime<chrono::offset::Utc>, String>,
    issuer: Result<profile::Profile, String>,
    name: Result<String, String>,
    proof: Result<Option<AchievementCredentialProof>, String>,
    refresh_service: Result<Option<general::RefreshService>, String>,
    terms_of_use: Result<Option<AchievementCredentialTermsOfUse>, String>,
    type_: Result<Option<AchievementCredentialType>, String>,
}
impl Default for AchievementCredentialBuilder {
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
            type_: Ok(Default::default()),
        }
    }
}
impl AchievementCredentialBuilder {
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
        T: std::convert::TryInto<Option<AchievementCredentialSchema>>,
        T::Error: std::fmt::Display,
    {
        self.credential_schema = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for credential_schema: {}", e));
        self
    }
    pub fn credential_status<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<CredentialStatus>>,
        T::Error: std::fmt::Display,
    {
        self.credential_status = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for credential_status: {}", e));
        self
    }
    pub fn credential_subject<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<achievement_subject::AchievementSubject>,
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
    pub fn endorsement_jwt<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Vec<AchievementCredentialEndorsementJwtItem>>,
        T::Error: std::fmt::Display,
    {
        self.endorsement_jwt = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for endorsement_jwt: {}", e));
        self
    }
    pub fn evidence<T>(mut self, value: Vec<T>) -> Self
    where
        T: std::convert::TryInto<proof_evidence::Evidence>,
        T::Error: std::fmt::Display,
    {
        self.evidence = value
            .into_iter()
            .map(|value| {
                value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for alignment: {}", e))
            })
            .collect();
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
        T: std::convert::TryInto<Option<general::Image>>,
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
        T: std::convert::TryInto<Option<AchievementCredentialProof>>,
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
        T: std::convert::TryInto<Option<AchievementCredentialTermsOfUse>>,
        T::Error: std::fmt::Display,
    {
        self.terms_of_use = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for terms_of_use: {}", e));
        self
    }
    pub fn type_<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<AchievementCredentialType>>,
        T::Error: std::fmt::Display,
    {
        self.type_ = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for type_: {}", e));
        self
    }
}
impl std::convert::TryFrom<AchievementCredentialBuilder> for AchievementCredential {
    type Error = String;
    fn try_from(value: AchievementCredentialBuilder) -> Result<Self, String> {
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
impl From<AchievementCredential> for AchievementCredentialBuilder {
    fn from(value: AchievementCredential) -> Self {
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
#[derive(Clone, Debug, PartialEq)]
pub struct CredentialSchemaBuilder {
    id: Result<String, String>,
    type_: Result<String, String>,
}
impl Default for CredentialSchemaBuilder {
    fn default() -> Self {
        Self {
            id: Err("no value supplied for id".to_string()),
            type_: Err("no value supplied for type_".to_string()),
        }
    }
}
impl CredentialSchemaBuilder {
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
impl std::convert::TryFrom<CredentialSchemaBuilder> for CredentialSchema {
    type Error = String;
    fn try_from(value: CredentialSchemaBuilder) -> Result<Self, String> {
        Ok(Self {
            id: value.id?,
            type_: value.type_?,
        })
    }
}
impl From<CredentialSchema> for CredentialSchemaBuilder {
    fn from(value: CredentialSchema) -> Self {
        Self {
            id: Ok(value.id),
            type_: Ok(value.type_),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CredentialStatusBuilder {
    id: Result<String, String>,
    type_: Result<String, String>,
}
impl Default for CredentialStatusBuilder {
    fn default() -> Self {
        Self {
            id: Err("no value supplied for id".to_string()),
            type_: Err("no value supplied for type_".to_string()),
        }
    }
}
impl CredentialStatusBuilder {
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
impl std::convert::TryFrom<CredentialStatusBuilder> for CredentialStatus {
    type Error = String;
    fn try_from(value: CredentialStatusBuilder) -> Result<Self, String> {
        Ok(Self {
            id: value.id?,
            type_: value.type_?,
        })
    }
}
impl std::convert::TryFrom<CredentialStatusBuilder> for Option<CredentialStatus> {
    type Error = String;
    fn try_from(value: CredentialStatusBuilder) -> Result<Self, String> {
        Ok(Some(CredentialStatus {
            id: value.id?,
            type_: value.type_?,
        }))
    }
}
impl From<CredentialStatus> for CredentialStatusBuilder {
    fn from(value: CredentialStatus) -> Self {
        Self {
            id: Ok(value.id),
            type_: Ok(value.type_),
        }
    }
}
