use chrono::{DateTime, Utc};
use serde::{de, Deserialize, Serialize};
use std::{collections::HashMap, ops};
use types_common::{DurationType, EmailAddress, EnumDeserialize, OneOrMany, PositiveInteger, TagType};

/// Error types.
pub mod error {
    /// Error from a TryFrom or FromStr implementation.
    pub struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EuropassEdcCredential {
    #[serde(rename = "@context")]
    pub context: EuropassEdcCredentialContext,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_profiles: Option<OneOrMany<Concept>>,
    ///One or more schemas that validate the Verifiable Credential.
    pub credential_schema: OneOrMany<CredentialSchema>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_status: Option<EuropassEdcCredentialCredentialStatus>,

    pub credential_subject: OneOrMany<AgentOrPersonOrOrganisation>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_parameter: Option<DisplayParameter>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence: Option<OneOrMany<Evidence>>,
    ///Globally unique identifier for the issued credential. It can be a UUID or another globally unique identifier.
    pub id: String,
    ///DID of the credential issuer
    pub issuer: Organisation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terms_of_use: Option<OneOrMany<TermsOfUseValue>>,
    ///Full type chain, used to identify the credential base types
    #[serde(rename = "type")]
    pub type_: Vec<String>,
    ///Defines the earliest point when the credential becomes valid.
    pub valid_from: DateTime<Utc>,
    ///Defines the latest point when the credential ceases to be valid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct Accreditation {
    #[serde(rename = "accreditingAgent")]
    pub accrediting_agent: Organisation,
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<OneOrMany<Note>>,
    #[serde(rename = "dateIssued", default, skip_serializing_if = "Option::is_none")]
    pub date_issued: Option<DateTime<Utc>>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTime<Utc>>,
    #[serde(rename = "dcType")]
    pub dc_type: Concept,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPairs>,
    #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<OneOrMany<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifier>,
    #[serde(rename = "landingPage", default, skip_serializing_if = "Option::is_none")]
    pub landing_page: Option<OneOrMany<WebResource>>,
    #[serde(rename = "limitCredentialType", default, skip_serializing_if = "Option::is_none")]
    pub limit_credential_type: Option<OneOrMany<Concept>>,
    #[serde(rename = "limitEQFLevel", default, skip_serializing_if = "Option::is_none")]
    pub limit_eqf_level: Option<OneOrMany<Concept>>,
    #[serde(rename = "limitField", default, skip_serializing_if = "Option::is_none")]
    pub limit_field: Option<OneOrMany<Concept>>,
    #[serde(rename = "limitJurisdiction", default, skip_serializing_if = "Option::is_none")]
    pub limit_jurisdiction: Option<OneOrMany<Concept>>,
    #[serde(rename = "limitQualification", default, skip_serializing_if = "Option::is_none")]
    pub limit_qualification: Option<Qualification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organisation: Option<OneOrMany<Organisation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report: Option<WebResource>,
    #[serde(rename = "reviewDate", default, skip_serializing_if = "Option::is_none")]
    pub review_date: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "supplementaryDocument", default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<OneOrMany<WebResource>>,
    pub title: LangKVPairs,
    #[serde(rename = "type")]
    pub type_: OneOrMany<AccreditationTag>,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct Address {
    #[serde(rename = "countryCode")]
    pub country_code: Concept,
    #[serde(rename = "fullAddress", default, skip_serializing_if = "Option::is_none")]
    pub full_address: Option<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifier>,
    #[serde(rename = "type")]
    pub type_: AddressTag,
}

#[derive(Clone, Debug, Serialize, EnumDeserialize)]
#[serde(untagged)]
pub enum AgentOrPersonOrOrganisation {
    Agent(Box<Agent>),
    Person(Box<Person>),
    Organisation(Box<Organisation>),
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct Agent {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<OneOrMany<Note>>,
    #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<LangKVPairs>,
    #[serde(rename = "contactPoint", default, skip_serializing_if = "Option::is_none")]
    pub contact_point: Option<OneOrMany<ContactPoint>>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTime<Utc>>,
    #[serde(rename = "groupMemberOf", default, skip_serializing_if = "Option::is_none")]
    pub group_member_of: Option<OneOrMany<Group>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<OneOrMany<IdentifierOrLegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<OneOrMany<Location>>,
    #[serde(rename = "prefLabel", default, skip_serializing_if = "Option::is_none")]
    pub pref_label: Option<LangKVPairs>,
    #[serde(rename = "type")]
    pub type_: AgentTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct Amount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    pub unit: Concept,
    pub value: f32,
    #[serde(rename = "type")]
    pub type_: AmountTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct AwardingOpportunity {
    #[serde(rename = "awardingBody")]
    pub awarding_body: OneOrMany<AgentOrPersonOrOrganisation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<OneOrMany<IdentifierOrLegalIdentifier>>,
    #[serde(rename = "learningAchievementSpecification", default)]
    pub learning_achievement_specification: Option<LearningAchievementSpecificationOrQualification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporal: Option<PeriodOfTime>,
    #[serde(rename = "type")]
    pub type_: AwardingOpportunityTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AwardingProcess {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<OneOrMany<Note>>,
    pub awarding_body: OneOrMany<AgentOrPersonOrOrganisation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub awarding_date: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub awards: Option<OneOrMany<ClaimNode>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub educational_system_note: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<OneOrMany<IdentifierOrLegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used: Option<OneOrMany<LearningAssessment>>,
    #[serde(rename = "type")]
    pub type_: AwardingProcessTag,
}

#[derive(Clone, Debug, Serialize, EnumDeserialize)]
#[serde(untagged)]
pub enum ClaimNode {
    LearningAchievement(Box<LearningAchievement>),
    LearningActivity(Box<LearningActivity>),
    LearningAssessment(Box<LearningAssessment>),
    LearningEntitlement(Box<LearningEntitlement>),
    ClaimNodeType(Box<ClaimTypeNode>),
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ClaimTypeNode {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<OneOrMany<Note>>,
    pub awarded_by: AwardingProcess,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<OneOrMany<IdentifierOrLegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<OneOrMany<WebResource>>,
    pub title: LangKVPairs,
    #[serde(rename = "type")]
    pub type_: ClaimTypeNodeTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct ConceptScheme {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(rename = "type")]
    pub type_: ConceptSchemeTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct Concept {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(rename = "inScheme", default, skip_serializing_if = "Option::is_none")]
    pub in_scheme: Option<ConceptScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notation: Option<Literal>,
    #[serde(rename = "prefLabel", default, skip_serializing_if = "Option::is_none")]
    pub pref_label: Option<LangKVPairs>,
    #[serde(rename = "type")]
    pub type_: ConceptTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct ContactPoint {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<OneOrMany<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<OneOrMany<Address>>,
    #[serde(rename = "contactForm", default, skip_serializing_if = "Option::is_none")]
    pub contact_form: Option<OneOrMany<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPairs>,
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<OneOrMany<Mailbox>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<OneOrMany<Phone>>,
    #[serde(rename = "type")]
    pub type_: ContactPointTag,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum CredentialSchemaType {
    JsonSchema,
    ShaclValidator2017,
}

impl ToString for CredentialSchemaType {
    fn to_string(&self) -> String {
        match *self {
            Self::JsonSchema => "JsonSchema".to_string(),
            Self::ShaclValidator2017 => "ShaclValidator2017".to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct CredentialStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(rename = "type")]
    pub type_: CredentialStatusTag,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CredentialSubject {
    ///Defines the DID of the subject that is described by the issued credential
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct CreditPoint {
    pub framework: Concept,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    pub point: String,
    #[serde(rename = "type")]
    pub type_: CreditPointTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct DisplayDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    pub image: MediaObject,
    pub page: PositiveInteger,
    #[serde(rename = "type")]
    pub type_: DisplayDetailTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DisplayParameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    pub individual_display: OneOrMany<IndividualDisplay>,
    pub language: OneOrMany<Concept>,
    pub primary_language: Concept,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary_display: Option<String>,
    pub title: LangKVPairs,
    #[serde(rename = "type")]
    pub type_: DisplayParameterTag,
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum EuropassEdcCredentialContext {
    One(String),
    Many(Vec<String>),
}

impl<'de> de::Deserialize<'de> for EuropassEdcCredentialContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        let must_contain = "https://www.w3.org/ns/credentials/v2";

        match value {
            serde_json::Value::Array(arr) => {
                let val = serde_json::Value::Array(arr);
                let many: Vec<String> = Vec::deserialize(val).map_err(de::Error::custom)?;

                if many.is_empty() {
                    return Err(de::Error::invalid_value(de::Unexpected::Seq, &"Array cannot be empty"));
                }

                if &must_contain != &many[0] {
                    Err(de::Error::invalid_value(
                        de::Unexpected::Str(&many[0]),
                        &format!("First value must be: {}", must_contain).as_str(),
                    ))
                } else {
                    Ok(Self::Many(many))
                }
            }
            serde_json::Value::String(one) => {
                if &must_contain == &one {
                    Ok(Self::One(one))
                } else {
                    Err(de::Error::invalid_value(de::Unexpected::Str(&one), &must_contain))
                }
            }
            _ => Err(de::Error::invalid_type(
                de::Unexpected::Other(&value.to_string()),
                &"An array of string or a string",
            )),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EuropassEdcCredentialCredentialStatus {
    ///Exact identity for the credential status
    pub id: UriType,
    ///Defines the revocation type extension
    #[serde(rename = "type")]
    pub revocation_type: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EuropassEdcCredentialIssuer {
    String(String),
    Object {
        ///DID of the credential issuer
        id: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum DataOrUri {
    Data(AgentOrPersonOrOrganisation),
    GenericId(UriType),
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct EuropeanDigitalCredential {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment: Option<OneOrMany<MediaObject>>,
    #[serde(rename = "credentialProfiles")]
    pub credential_profiles: OneOrMany<Concept>,
    #[serde(rename = "credentialSchema")]
    pub credential_schema: OneOrMany<CredentialSchema>,
    #[serde(rename = "credentialStatus", default, skip_serializing_if = "Option::is_none")]
    pub credential_status: Option<OneOrMany<CredentialStatus>>,
    #[serde(rename = "credentialSubject")]
    pub credential_subject: AgentOrPersonOrOrganisation,
    #[serde(rename = "displayParameter")]
    pub display_parameter: DisplayParameter,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence: Option<OneOrMany<Evidence>>,
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<OneOrMany<DateTime<Utc>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holder: Option<OneOrMany<AgentOrPersonOrOrganisation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<OneOrMany<IdentifierOrLegalIdentifier>>,
    #[serde(rename = "issuanceDate", default, skip_serializing_if = "Option::is_none")]
    pub issuance_date: Option<DateTime<Utc>>,
    pub issued: DateTime<Utc>,
    pub issuer: DataOrUri,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof: Option<OneOrMany<Proof>>,
    #[serde(rename = "termsOfUse", default, skip_serializing_if = "Option::is_none")]
    pub terms_of_use: Option<OneOrMany<TermsOfUseValue>>,
    #[serde(rename = "validFrom")]
    pub valid_from: DateTime<Utc>,
    #[serde(rename = "validUntil", default, skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<DateTime<Utc>>,
    #[serde(rename = "type")]
    pub type_: EuropeanDigitalCredentialTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct EuropeanDigitalPresentation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holder: Option<OneOrMany<AgentOrPersonOrOrganisation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof: Option<OneOrMany<Proof>>,
    #[serde(rename = "verifiableCredential", default, skip_serializing_if = "Option::is_none")]
    pub verifiable_credential: Option<OneOrMany<EuropeanDigitalCredential>>,
    #[serde(rename = "verificationCheck", default, skip_serializing_if = "Option::is_none")]
    pub verification_check: Option<OneOrMany<VerificationCheck>>,
    #[serde(rename = "type")]
    pub type_: EuropeanDigitalPresentationTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Evidence {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accreditation: Option<Box<Accreditation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embedded_evidence: Option<OneOrMany<MediaObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_statement: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_target: Option<AgentOrPersonOrOrganisation>,
    ///If present, it MUST contain a URL that points to where more information about this instance of evidence can be found.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<Concept>,
    #[serde(rename = "type")]
    pub type_: EvidenceTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct Geometry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latitude: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<String>,
    #[serde(rename = "type")]
    pub type_: GeometryTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct GradingScheme {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<OneOrMany<IdentifierOrLegalIdentifier>>,
    #[serde(rename = "supplementaryDocument", default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<OneOrMany<WebResource>>,
    pub title: LangKVPairs,
    #[serde(rename = "type")]
    pub type_: GradingSchemeTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct Grant {
    #[serde(rename = "contentURL", default, skip_serializing_if = "Option::is_none")]
    pub content_url: Option<UriType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(rename = "supplementaryDocument", default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<OneOrMany<WebResource>>,
    pub title: LangKVPairs,
    #[serde(rename = "type")]
    pub type_: GrantTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Group {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<OneOrMany<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_point: Option<OneOrMany<ContactPoint>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<OneOrMany<Location>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<OneOrMany<AgentOrPersonOrOrganisation>>,
    pub pref_label: LangKVPairs,
    #[serde(rename = "type")]
    pub type_: GroupTag,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct HtmlType(pub String);
impl std::ops::Deref for HtmlType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

#[derive(Clone, Debug, Serialize, EnumDeserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum IdentifierOrLegalIdentifier {
    Identifier(Box<Identifier>),
    LegalIdentifier(Box<LegalIdentifier>),
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Identifier {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<IriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_issued: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    pub notation: Literal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme_agency: Option<LangKV>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme_id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme_version: Option<String>,
    #[serde(rename = "type")]
    pub type_: IdentifierTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct IndividualDisplay {
    pub display_detail: OneOrMany<DisplayDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    pub language: Concept,
    #[serde(rename = "type")]
    pub type_: IndividualDisplayTag,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IriType(pub String);
impl std::ops::Deref for IriType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct IssuerNode {
    pub eidas_legal_identifier: LegalIdentifier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(rename = "type")]
    pub type_: IssuerNodeTag,
}

#[derive(Clone, Debug, Serialize)]
pub struct LangKV(pub serde_json::Map<String, serde_json::Value>);

impl LangKV {
    pub fn new(kv_pair: serde_json::Map<String, serde_json::Value>) -> Option<Self> {
        if 1 == kv_pair.len() {
            Some(Self(kv_pair))
        } else {
            None
        }
    }
}

impl std::ops::Deref for LangKV {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}

impl<'de> Deserialize<'de> for LangKV {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let map = serde_json::Map::deserialize(deserializer)?;

        if let Some(kv_pair) = LangKV::new(map) {
            Ok(kv_pair)
        } else {
            Err(<D::Error as de::Error>::missing_field("Requires 1 lang key value pair"))
        }
    }
}

#[derive(Clone, Debug, Serialize, EnumDeserialize)]
#[serde(untagged)]
pub enum LearningAchievementSpecificationOrQualification {
    LearningAchievementSpecification(Box<LearningAchievementSpecification>),
    Qualification(Box<Qualification>),
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LearningAchievementSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<OneOrMany<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub awarding_opportunity: Option<OneOrMany<AwardingOpportunity>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit_point: Option<OneOrMany<CreditPoint>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub education_level: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub education_subject: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entitles_to: Option<OneOrMany<LearningEntitlementSpecification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_requirement: Option<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generalisation_of: Option<OneOrMany<LearningAchievementSpecificationOrQualification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<OneOrMany<LearningAchievementSpecificationOrQualification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<OneOrMany<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub influenced_by: Option<Box<OneOrMany<LearningActivitySpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<OneOrMany<LearningAchievementSpecificationOrQualification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_outcome: Option<OneOrMany<LearningOutcome>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_outcome_summary: Option<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_setting: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_duration: Option<DurationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proven_by: Option<Box<OneOrMany<LearningAssessmentSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specialisation_of: Option<OneOrMany<LearningAchievementSpecificationOrQualification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<OneOrMany<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_group: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thematic_area: Option<OneOrMany<Concept>>,
    pub title: LangKVPairs,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_of_learning: Option<DurationType>,
    #[serde(rename = "type")]
    pub type_: LearningAchievementSpecificationTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LearningAchievement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<OneOrMany<Note>>,
    pub awarded_by: Box<AwardingProcess>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit_received: Option<OneOrMany<CreditPoint>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entitles_to: Option<OneOrMany<LearningEntitlement>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<Box<OneOrMany<LearningAchievement>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub influenced_by: Option<OneOrMany<LearningActivity>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<Box<OneOrMany<LearningAchievement>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_opportunity: Option<LearningOpportunity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proven_by: Option<Box<OneOrMany<LearningAssessment>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specified_by: Option<LearningAchievementSpecificationOrQualification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<OneOrMany<WebResource>>,
    pub title: LangKVPairs,
    #[serde(rename = "type")]
    pub type_: LearningAchievementTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LearningActivitySpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<OneOrMany<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_hour: Option<OneOrMany<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generalisation_of: Option<OneOrMany<LearningActivitySpecification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<OneOrMany<LearningActivitySpecification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<OneOrMany<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<OneOrMany<IdentifierOrLegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub influences: Option<OneOrMany<LearningAchievementSpecificationOrQualification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<OneOrMany<LearningActivitySpecification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specialisation_of: Option<OneOrMany<LearningActivitySpecification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<OneOrMany<WebResource>>,
    pub title: LangKVPairs,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_of_learning: Option<DurationType>,
    #[serde(rename = "type")]
    pub type_: LearningActivitySpecificationTag,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LearningActivity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<OneOrMany<Note>>,
    pub awarded_by: Box<AwardingProcess>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directed_by: Option<OneOrMany<AgentOrPersonOrOrganisation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<Box<OneOrMany<LearningActivity>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<OneOrMany<IdentifierOrLegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub influences: Option<Box<OneOrMany<LearningAchievement>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<Box<OneOrMany<LearningActivity>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_opportunity: Option<LearningOpportunity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level_of_completion: Option<Percentage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<OneOrMany<Location>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specified_by: Option<LearningActivitySpecification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<OneOrMany<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporal: Option<OneOrMany<PeriodOfTime>>,
    pub title: LangKVPairs,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workload: Option<DurationType>,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct LearningAssessmentSpecification {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<OneOrMany<Note>>,
    #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<LangKVPairs>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTime<Utc>>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPairs>,
    #[serde(rename = "generalisationOf", default)]
    pub generalisation_of: Option<Box<OneOrMany<LearningAssessmentSpecification>>>,
    #[serde(rename = "gradingScheme", default, skip_serializing_if = "Option::is_none")]
    pub grading_scheme: Option<GradingScheme>,
    #[serde(rename = "hasPart", default)]
    pub has_part: Option<Box<OneOrMany<LearningAssessmentSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<OneOrMany<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<OneOrMany<IdentifierOrLegalIdentifier>>,
    #[serde(rename = "isPartOf", default)]
    pub is_part_of: Option<Box<OneOrMany<LearningAssessmentSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proves: Option<OneOrMany<LearningAchievementSpecificationOrQualification>>,
    #[serde(rename = "specialisationOf", default)]
    pub specialisation_of: Option<Box<OneOrMany<LearningAssessmentSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "supplementaryDocument", default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<OneOrMany<WebResource>>,
    pub title: LangKVPairs,
    #[serde(rename = "type")]
    pub type_: LearningAssessmentSpecificationTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LearningAssessment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<OneOrMany<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assessed_by: Option<OneOrMany<AgentOrPersonOrOrganisation>>,
    pub awarded_by: Box<AwardingProcess>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_issued: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPairs>,
    pub grade: Note,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade_status: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<Box<OneOrMany<LearningAssessment>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_verification: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<OneOrMany<IdentifierOrLegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<Box<OneOrMany<LearningAssessment>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proves: Option<OneOrMany<LearningAchievement>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_distribution: Option<ResultDistribution>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shortened_grading: Option<ShortenedGrading>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specified_by: Option<OneOrMany<LearningAssessmentSpecification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<OneOrMany<WebResource>>,
    pub title: LangKVPairs,
    #[serde(rename = "type")]
    pub type_: LearningAssessmentTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LearningEntitlementSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<OneOrMany<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTime<Utc>>,
    pub dc_type: OneOrMany<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entitled_by: Option<OneOrMany<LearningAchievementSpecificationOrQualification>>,
    pub entitlement_status: Concept,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generalisation_of: Option<Box<OneOrMany<LearningEntitlementSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<Box<OneOrMany<LearningEntitlementSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<OneOrMany<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<OneOrMany<IdentifierOrLegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<Box<OneOrMany<LearningEntitlementSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_jurisdiction: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_national_occupation: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_occupation: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_organisation: Option<Box<OneOrMany<Organisation>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specialisation_of: Option<Box<OneOrMany<LearningEntitlementSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<OneOrMany<WebResource>>,
    pub title: LangKVPairs,
    #[serde(rename = "type")]
    pub type_: LearningEntitlementSpecificationTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LearningEntitlement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<OneOrMany<Note>>,
    pub awarded_by: Box<AwardingProcess>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_issued: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entitled_by: Option<Box<OneOrMany<LearningAchievement>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<Box<OneOrMany<LearningEntitlement>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<OneOrMany<IdentifierOrLegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<Box<OneOrMany<LearningEntitlement>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specified_by: Option<OneOrMany<LearningEntitlementSpecification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<OneOrMany<WebResource>>,
    pub title: LangKVPairs,
    #[serde(rename = "type")]
    pub type_: LearningEntitlementTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LearningOpportunity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<OneOrMany<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admission_procedure: Option<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_deadline: Option<OneOrMany<DateTime<Utc>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner_image: Option<MediaObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_language: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_html: Option<OneOrMany<HtmlType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<DurationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant: Option<OneOrMany<Grant>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<OneOrMany<LearningOpportunity>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<OneOrMany<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<OneOrMany<IdentifierOrLegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<OneOrMany<LearningOpportunity>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_achievement_specification: Option<LearningAchievementSpecificationOrQualification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_activity_specification: Option<LearningActivitySpecification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_schedule: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<OneOrMany<Location>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_detail: Option<OneOrMany<PriceDetail>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provided_by: Option<Box<Organisation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_information: Option<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<OneOrMany<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporal: Option<PeriodOfTime>,
    pub title: LangKVPairs,
    #[serde(rename = "type")]
    pub type_: LearningOpportunityTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LearningOutcome {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<OneOrMany<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifier>,
    #[serde(rename = "relatedESCOSkill", default, skip_serializing_if = "Option::is_none")]
    pub related_esco_skill: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_skill: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reusability_level: Option<Concept>,
    pub title: LangKVPairs,
    #[serde(rename = "type")]
    pub type_: LearningOutcomeTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LegalIdentifier {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<IriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_issued: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    pub notation: Literal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme_agency: Option<LangKV>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme_id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme_version: Option<String>,
    pub spatial: Concept,
    #[serde(rename = "type")]
    pub type_: LegalIdentifierTag,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Literal(pub String);

impl std::ops::Deref for Literal {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Location {
    pub address: Option<OneOrMany<Address>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geographic_name: Option<OneOrMany<Address>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geometry: Option<OneOrMany<Geometry>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spatial_code: Option<OneOrMany<Concept>>,
    #[serde(rename = "type")]
    pub type_: LocationTag,
}

#[derive(Debug, Clone)]
pub struct MailTo(String);

impl ops::Deref for MailTo {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for MailTo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for MailTo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let email_str = String::deserialize(deserializer)?;

        let mail_to_regex = regex::Regex::new("^mailto:[^@]*[^\\.]@[^\\.]($|[^@]*[^\\.]$)").unwrap();

        if mail_to_regex.is_match(&email_str) {
            Ok(Self(email_str))
        } else {
            Err(de::Error::invalid_value(
                de::Unexpected::Str(&email_str),
                &"A valid email format",
            ))
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Email {
    EmailAddress(EmailAddress),
    MailTo(MailTo),
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct Mailbox {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Email>,
    #[serde(rename = "type")]
    pub type_: MailboxTag,
}

#[derive(Clone, Debug, Serialize)]
pub struct LangKVPairs(HashMap<LangKey, serde_json::Value>);

impl LangKVPairs {
    pub fn new(kv_pairs: HashMap<LangKey, serde_json::Value>) -> Option<Self> {
        if !kv_pairs.is_empty() {
            Some(LangKVPairs(kv_pairs))
        } else {
            None
        }
    }
}

impl std::ops::Deref for LangKVPairs {
    type Target = std::collections::HashMap<LangKey, serde_json::Value>;
    fn deref(&self) -> &std::collections::HashMap<LangKey, serde_json::Value> {
        &self.0
    }
}

impl<'de> Deserialize<'de> for LangKVPairs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let kv_pairs = HashMap::deserialize(deserializer)?;

        if let Some(lang_kv_pairs) = LangKVPairs::new(kv_pairs) {
            Ok(lang_kv_pairs)
        } else {
            Err(<D::Error as serde::de::Error>::missing_field(
                "Requires at least one lang pair",
            ))
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct LangKey(String);
impl std::ops::Deref for LangKey {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for LangKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        let regex_str =
            "^(aa|ab|ae|af|ak|am|an|ar|as|av|ay|az|ba|be|bg|bh|bi|bm|bn|bo|br|bs|ca|ce|ch|co|cr|cs|cu|cv|cy|da|de|dv|dz|ee|el|en|eo|es|et|eu|fa|ff|fi|fj|fo|fr|fy|ga|gd|gl|gn|gu|gv|ha|he|hi|ho|hr|ht|hu|hy|hz|ia|id|ie|ig|ii|ik|in|io|is|it|iu|iw|ja|ji|jv|jw|ka|kg|ki|kj|kk|kl|km|kn|ko|kr|ks|ku|kv|kw|ky|la|lb|lg|li|ln|lo|lt|lu|lv|mg|mh|mi|mk|ml|mn|mo|mr|ms|mt|my|na|nb|nd|ne|ng|nl|nn|no|nr|nv|ny|oc|oj|om|or|os|pa|pi|pl|ps|pt|qu|rm|rn|ro|ru|rw|sa|sc|sd|se|sg|sh|si|sk|sl|sm|sn|so|sq|sr|ss|st|su|sv|sw|ta|te|tg|th|ti|tk|tl|tn|to|tr|ts|tt|tw|ty|ug|uk|ur|uz|ve|vi|vo|wa|wo|xh|yi|yo|za|zh|zu)$";

        if regex::Regex::new(regex_str).unwrap().is_match(value) {
            Ok(Self(value.to_string()))
        } else {
            Err(format!("Doesn't match the pattern: \"{value}\"").into())
        }
    }
}

impl<'de> serde::Deserialize<'de> for LangKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MediaObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_type: Option<Concept>,
    pub content: String,
    pub content_encoding: Concept,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_size: Option<i64>,
    pub content_type: Concept,
    #[serde(rename = "contentURL", default, skip_serializing_if = "Option::is_none")]
    pub content_url: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<LangKVPairs>,
    #[serde(rename = "type")]
    pub type_: MediaObjectTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Note {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note_format: Option<Concept>,
    pub note_literal: LangKVPairs,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<Concept>,
    #[serde(rename = "type")]
    pub type_: NoteTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Organisation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accreditation: Option<OneOrMany<Accreditation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<OneOrMany<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_point: Option<OneOrMany<ContactPoint>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<OneOrMany<Concept>>,
    #[serde(rename = "eIDASIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub e_idas_identifier: Option<LegalIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_member_of: Option<OneOrMany<Group>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_member: Option<OneOrMany<Person>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_sub_organization: Option<Box<OneOrMany<Organisation>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<OneOrMany<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<OneOrMany<IdentifierOrLegalIdentifier>>,
    pub legal_name: LangKVPairs,
    pub location: OneOrMany<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo: Option<MediaObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration: Option<LegalIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_organization_of: Option<Box<Organisation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_identifier: Option<OneOrMany<LegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vat_identifier: Option<OneOrMany<LegalIdentifier>>,
    #[serde(rename = "type")]
    pub type_: OrganisationTag,
}

#[derive(Clone, Debug, Serialize)]
pub struct Percentage(u32);

impl Percentage {
    /// Returns None if num > 100
    pub fn new(num: u32) -> Option<Self> {
        if num <= 100 {
            Some(Self(num))
        } else {
            None
        }
    }
}

impl ops::Deref for Percentage {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Percentage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let num = u32::deserialize(deserializer)?;

        if let Some(percentage) = Self::new(num) {
            Ok(percentage)
        } else {
            Err(<D::Error as serde::de::Error>::invalid_value(
                de::Unexpected::Unsigned(num.into()),
                &"A number between 0 and 100",
            ))
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct PeriodOfTime {
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(rename = "prefLabel", default, skip_serializing_if = "Option::is_none")]
    pub pref_label: Option<LangKVPairs>,
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateTime<Utc>>,
    #[serde(rename = "type")]
    pub type_: PeriodOfTimeTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Person {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birth_name: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citizenship_country: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_point: Option<OneOrMany<ContactPoint>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family_name: Option<LangKV>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<LangKV>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<LangKV>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_member_of: Option<OneOrMany<Group>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_claim: Option<OneOrMany<ClaimNode>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<OneOrMany<EuropeanDigitalCredential>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<OneOrMany<IdentifierOrLegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_of: Option<Box<OneOrMany<Organisation>>>,
    #[serde(rename = "nationalID", default, skip_serializing_if = "Option::is_none")]
    pub national_id: Option<LegalIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patronymic_name: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place_of_birth: Option<Location>,
    #[serde(rename = "type")]
    pub type_: PersonTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Phone {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_dialing: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_dialing: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dial_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "type")]
    pub type_: PhoneTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PriceDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<OneOrMany<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<OneOrMany<IdentifierOrLegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pref_label: Option<LangKVPairs>,
    #[serde(rename = "type")]
    pub type_: PriceDetailTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct Proof {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(rename = "type")]
    pub type_: ProofTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Qualification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accreditation: Option<OneOrMany<Accreditation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<OneOrMany<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub awarding_opportunity: Option<OneOrMany<AwardingOpportunity>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit_point: Option<OneOrMany<CreditPoint>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub education_level: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub education_subject: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entitles_to: Option<OneOrMany<LearningEntitlementSpecification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_requirement: Option<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eqf_level: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generalisation_of: Option<OneOrMany<Qualification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<OneOrMany<Qualification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<OneOrMany<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub influenced_by: Option<OneOrMany<LearningActivitySpecification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<OneOrMany<Qualification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_partial_qualification: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_outcome: Option<OneOrMany<LearningOutcome>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_outcome_summary: Option<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_setting: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_duration: Option<DurationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nqf_level: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proven_by: Option<Box<OneOrMany<LearningAssessmentSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qualification_code: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specialisation_of: Option<OneOrMany<Qualification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<OneOrMany<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_group: Option<OneOrMany<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thematic_area: Option<OneOrMany<Concept>>,
    pub title: LangKVPairs,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_of_learning: Option<DurationType>,
    #[serde(rename = "type")]
    pub type_: QualificationTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ResultCategory {
    pub count: PositiveInteger,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_score: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_score: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
    #[serde(rename = "type")]
    pub type_: ResultCategoryTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ResultDistribution {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_category: Option<OneOrMany<ResultCategory>>,
    #[serde(rename = "type")]
    pub type_: ResultDistributionTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct CredentialSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(rename = "type")]
    pub type_: CredentialSchemaType,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ShortenedGrading {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    pub percentage_equal: i32,
    pub percentage_higher: i32,
    pub percentage_lower: i32,
    #[serde(rename = "type")]
    pub type_: ShortenedGradingTag,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TermsOfUseValue {
    ///Contains a URL that points to where more information about this instance of terms of use can be found.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Defines the type extension
    #[serde(rename = "type")]
    pub type_extension: String,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UriType(pub fluent_uri::Uri<String>);

impl ops::Deref for UriType {
    type Target = fluent_uri::Uri<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for UriType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for UriType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        let uri = fluent_uri::Uri::parse(value.clone());

        match uri {
            Ok(uri) => Ok(UriType(uri)),
            Err(err) => Err(<D::Error as de::Error>::custom(err)),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VerificationCheck {
    pub dc_type: Concept,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPairs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elm_subject: Option<EuropeanDigitalCredential>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    pub subject: serde_json::Value,
    pub verification_status: Concept,
    #[serde(rename = "type")]
    pub type_: VerificationCheckTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct WebResource {
    #[serde(rename = "contentURL")]
    pub content_url: UriType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<LangKVPairs>,
    #[serde(rename = "type")]
    pub type_: WebResourceTag,
}
