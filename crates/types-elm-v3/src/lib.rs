use chrono::{DateTime, Utc};
use serde::{de, Deserialize, Serialize};
use std::{collections::HashMap, ops};
use types_common::{DurationType, Email, EnumDeserialize, ObjectOrVector, PositiveInteger, TagType};

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
    pub credential_profiles: Option<ObjectOrVector<Concept>>,
    ///One or more schemas that validate the Verifiable Credential.
    pub credential_schema: ObjectOrVector<CredentialSchema>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_status: Option<EuropassEdcCredentialCredentialStatus>,

    pub credential_subject: ObjectOrVector<AgentOrPersonOrOrganisation>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_parameter: Option<DisplayParameter>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence: Option<ObjectOrVector<Evidence>>,
    ///Globally unique identifier for the issued credential. It can be a UUID or another globally unique identifier.
    pub id: String,
    ///DID of the credential issuer
    pub issuer: Organisation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terms_of_use: Option<ObjectOrVector<TermsOfUseValue>>,
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
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(rename = "dateIssued", default, skip_serializing_if = "Option::is_none")]
    pub date_issued: Option<DateTime<Utc>>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTime<Utc>>,
    #[serde(rename = "dcType")]
    pub dc_type: Concept,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPair>,
    #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifier>,
    #[serde(rename = "landingPage", default, skip_serializing_if = "Option::is_none")]
    pub landing_page: Option<ObjectOrVector<WebResource>>,
    #[serde(rename = "limitCredentialType", default, skip_serializing_if = "Option::is_none")]
    pub limit_credential_type: Option<ObjectOrVector<Concept>>,
    #[serde(rename = "limitEQFLevel", default, skip_serializing_if = "Option::is_none")]
    pub limit_eqf_level: Option<ObjectOrVector<Concept>>,
    #[serde(rename = "limitField", default, skip_serializing_if = "Option::is_none")]
    pub limit_field: Option<ObjectOrVector<Concept>>,
    #[serde(rename = "limitJurisdiction", default, skip_serializing_if = "Option::is_none")]
    pub limit_jurisdiction: Option<ObjectOrVector<Concept>>,
    #[serde(rename = "limitQualification", default, skip_serializing_if = "Option::is_none")]
    pub limit_qualification: Option<Qualification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organisation: Option<ObjectOrVector<Organisation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report: Option<WebResource>,
    #[serde(rename = "reviewDate", default, skip_serializing_if = "Option::is_none")]
    pub review_date: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "supplementaryDocument", default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    pub title: LangKVPair,
    #[serde(rename = "type")]
    pub type_: ObjectOrVector<AccreditationTag>,
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
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<LangKVPair>,
    #[serde(rename = "contactPoint", default, skip_serializing_if = "Option::is_none")]
    pub contact_point: Option<ObjectOrVector<ContactPoint>>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTime<Utc>>,
    #[serde(rename = "groupMemberOf", default, skip_serializing_if = "Option::is_none")]
    pub group_member_of: Option<ObjectOrVector<Group>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ObjectOrVector<IdentifierOrLegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<ObjectOrVector<Location>>,
    #[serde(rename = "prefLabel", default, skip_serializing_if = "Option::is_none")]
    pub pref_label: Option<LangKVPair>,
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
    pub awarding_body: ObjectOrVector<AgentOrPersonOrOrganisation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ObjectOrVector<IdentifierOrLegalIdentifier>>,
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
    pub additional_note: Option<ObjectOrVector<Note>>,
    pub awarding_body: ObjectOrVector<AgentOrPersonOrOrganisation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub awarding_date: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub awards: Option<ObjectOrVector<ClaimNode>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub educational_system_note: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ObjectOrVector<IdentifierOrLegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used: Option<ObjectOrVector<LearningAssessment>>,
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
    pub additional_note: Option<ObjectOrVector<Note>>,
    pub awarded_by: AwardingProcess,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ObjectOrVector<IdentifierOrLegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    pub title: LangKVPair,
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
    pub definition: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(rename = "inScheme", default, skip_serializing_if = "Option::is_none")]
    pub in_scheme: Option<ConceptScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notation: Option<Literal>,
    #[serde(rename = "prefLabel", default, skip_serializing_if = "Option::is_none")]
    pub pref_label: Option<LangKVPair>,
    #[serde(rename = "type")]
    pub type_: ConceptTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct ContactPoint {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<ObjectOrVector<Address>>,
    #[serde(rename = "contactForm", default, skip_serializing_if = "Option::is_none")]
    pub contact_form: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPair>,
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<ObjectOrVector<Mailbox>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<ObjectOrVector<Phone>>,
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

///DisplayParameterType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "individualDisplay",
///    "language",
///    "primaryLanguage",
///    "title"
///  ],
///  "properties": {
///    "description": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "id": {
///      "$ref": "#/$defs/UriTypeType"
///    },
///    "individualDisplay": {
///      "$ref": "#/$defs/Many!IndividualDisplayType"
///    },
///    "language": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "primaryLanguage": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "summaryDisplay": {
///      "$ref": "#/$defs/StringType"
///    },
///    "title": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "type": {
///      "const": "DisplayParameter"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DisplayParameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub individual_display: Option<ObjectOrVector<IndividualDisplay>>,
    pub language: ObjectOrVector<Concept>,
    pub primary_language: Concept,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary_display: Option<String>,
    pub title: LangKVPair,
    #[serde(rename = "type")]
    pub type_: DisplayParameterTag,
}

///EuropassEdcCredentialContext
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "const": "https://www.w3.org/ns/credentials/v2"
///    },
///    {
///      "description": "Semantic context for the issued credential. First element MUST be https://www.w3.org/ns/credentials/v2",
///      "type": "array",
///      "items": {
///        "type": "string",
///        "format": "uri"
///      },
///      "minItems": 1,
///      "uniqueItems": true
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EuropassEdcCredentialContext {
    Object(serde_json::Value),
    Vector(Vec<String>),
}

impl From<&EuropassEdcCredentialContext> for EuropassEdcCredentialContext {
    fn from(value: &EuropassEdcCredentialContext) -> Self {
        value.clone()
    }
}

impl From<serde_json::Value> for EuropassEdcCredentialContext {
    fn from(value: serde_json::Value) -> Self {
        Self::Object(value)
    }
}

impl From<Vec<String>> for EuropassEdcCredentialContext {
    fn from(value: Vec<String>) -> Self {
        Self::Vector(value)
    }
}

///Defines suspension and/or revocation details for the issued credential. Further redefined by the type extension
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Defines suspension and/or revocation details for the issued credential. Further redefined by the type extension",
///  "type": "object",
///  "required": [
///    "id",
///    "type"
///  ],
///  "properties": {
///    "id": {
///      "description": "Exact identity for the credential status",
///      "type": "string",
///      "format": "uri"
///    },
///    "type": {
///      "description": "Defines the revocation type extension",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EuropassEdcCredentialCredentialStatus {
    ///Exact identity for the credential status
    pub id: String,
    ///Defines the revocation type extension
    #[serde(rename = "type")]
    pub revocation_type: String,
}

///DID of the credential issuer
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "DID of the credential issuer",
///  "oneOf": [
///    {
///      "type": "string",
///      "format": "uri"
///    },
///    {
///      "type": "object",
///      "required": [
///        "id"
///      ],
///      "properties": {
///        "id": {
///          "description": "DID of the credential issuer",
///          "type": "string",
///          "format": "uri"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EuropassEdcCredentialIssuer {
    String(String),
    Object {
        ///DID of the credential issuer
        id: String,
    },
}
impl From<&EuropassEdcCredentialIssuer> for EuropassEdcCredentialIssuer {
    fn from(value: &EuropassEdcCredentialIssuer) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct EuropeanDigitalCredential {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment: Option<ObjectOrVector<MediaObject>>,
    #[serde(rename = "credentialProfiles")]
    pub credential_profiles: ObjectOrVector<Concept>,
    #[serde(rename = "credentialSchema")]
    pub credential_schema: ObjectOrVector<CredentialSchema>,
    #[serde(rename = "credentialStatus", default, skip_serializing_if = "Option::is_none")]
    pub credential_status: Option<CredentialStatus>,
    #[serde(rename = "credentialSubject")]
    pub credential_subject: AgentOrPersonOrOrganisation,
    #[serde(rename = "displayParameter")]
    pub display_parameter: DisplayParameter,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence: Option<ObjectOrVector<Evidence>>,
    #[serde(rename = "expirationDate")]
    pub expiration_date: ObjectOrVector<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holder: Option<ObjectOrVector<AgentOrPersonOrOrganisation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ObjectOrVector<IdentifierOrLegalIdentifier>>,
    #[serde(rename = "issuanceDate", default, skip_serializing_if = "Option::is_none")]
    pub issuance_date: Option<DateTime<Utc>>,
    pub issued: DateTime<Utc>,
    pub issuer: AgentOrPersonOrOrganisation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof: Option<ObjectOrVector<Proof>>,
    #[serde(rename = "termsOfUse", default, skip_serializing_if = "Option::is_none")]
    pub terms_of_use: Option<ObjectOrVector<TermsOfUseValue>>,
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
    pub holder: Option<ObjectOrVector<AgentOrPersonOrOrganisation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof: Option<ObjectOrVector<Proof>>,
    #[serde(rename = "verifiableCredential", default, skip_serializing_if = "Option::is_none")]
    pub verifiable_credential: Option<ObjectOrVector<EuropeanDigitalCredential>>,
    #[serde(rename = "verificationCheck", default, skip_serializing_if = "Option::is_none")]
    pub verification_check: Option<ObjectOrVector<VerificationCheck>>,
    #[serde(rename = "type")]
    pub type_: EuropeanDigitalPresentationTag,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Evidence {
    ///If present, it MUST contain a URL that points to where more information about this instance of evidence can be found.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub dc_type: Concept,
    #[serde(rename = "type")]
    pub type_: ObjectOrVector<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct EvidenceType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accreditation: Option<Box<Accreditation>>,
    #[serde(rename = "embeddedEvidence", default, skip_serializing_if = "Option::is_none")]
    pub embedded_evidence: Option<ObjectOrVector<MediaObject>>,
    #[serde(rename = "evidenceStatement", default, skip_serializing_if = "Option::is_none")]
    pub evidence_statement: Option<String>,
    #[serde(rename = "evidenceTarget", default, skip_serializing_if = "Option::is_none")]
    pub evidence_target: Option<AgentOrPersonOrOrganisation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(rename = "type")]
    pub type_: EvidenceTypeTag,
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

///GradingSchemeType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "title"
///  ],
///  "properties": {
///    "description": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "id": {
///      "$ref": "#/$defs/UriTypeType"
///    },
///    "identifier": {
///      "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
///    },
///    "supplementaryDocument": {
///      "$ref": "#/$defs/Many!WebResourceType"
///    },
///    "title": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "type": {
///      "const": "GradingScheme"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct GradingScheme {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ObjectOrVector<IdentifierOrLegalIdentifier>>,
    #[serde(rename = "supplementaryDocument", default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    pub title: LangKVPair,
    #[serde(rename = "type")]
    pub type_: GradingSchemeTag,
}

///GrantType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "title"
///  ],
///  "properties": {
///    "contentURL": {
///      "$ref": "#/$defs/URIType"
///    },
///    "dcType": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "description": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "id": {
///      "$ref": "#/$defs/UriTypeType"
///    },
///    "supplementaryDocument": {
///      "$ref": "#/$defs/Many!WebResourceType"
///    },
///    "title": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "type": {
///      "const": "Grant"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct Grant {
    #[serde(rename = "contentURL", default, skip_serializing_if = "Option::is_none")]
    pub content_url: Option<UriType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(rename = "supplementaryDocument", default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    pub title: LangKVPair,
    #[serde(rename = "type")]
    pub type_: GrantTag,
}

///GroupType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "prefLabel"
///  ],
///  "properties": {
///    "additionalNote": {
///      "$ref": "#/$defs/Many!NoteType"
///    },
///    "altLabel": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "contactPoint": {
///      "$ref": "#/$defs/Many!ContactPointType"
///    },
///    "id": {
///      "$ref": "#/$defs/UriTypeType"
///    },
///    "location": {
///      "$ref": "#/$defs/Many!LocationType"
///    },
///    "member": {
///      "$ref": "#/$defs/Many!AgentOrPersonOrOrganisationType"
///    },
///    "prefLabel": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "type": {
///      "const": "Group"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Group {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_point: Option<ObjectOrVector<ContactPoint>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<ObjectOrVector<Location>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<ObjectOrVector<AgentOrPersonOrOrganisation>>,
    pub pref_label: LangKVPair,
    #[serde(rename = "type")]
    pub type_: GroupTag,
}

///HtmlType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string"
///}
/// ```
/// </details>
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

///IdentifierType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "notation"
///  ],
///  "properties": {
///    "creator": {
///      "$ref": "#/$defs/IRIType"
///    },
///    "dateIssued": {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    "dcType": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "id": {
///      "$ref": "#/$defs/UriTypeType"
///    },
///    "notation": {
///      "$ref": "#/$defs/LiteralType"
///    },
///    "schemeAgency": {
///      "$ref": "#/$defs/LangStringType"
///    },
///    "schemeId": {
///      "$ref": "#/$defs/URIType"
///    },
///    "schemeName": {
///      "$ref": "#/$defs/StringType"
///    },
///    "schemeVersion": {
///      "$ref": "#/$defs/StringType"
///    },
///    "type": {
///      "const": "Identifier"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Identifier {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<IriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_issued: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    pub notation: Literal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme_agency: Option<LangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme_id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme_version: Option<String>,
    #[serde(rename = "type")]
    pub type_: IdentifierTag,
}

///IndividualDisplayType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "displayDetail",
///    "language"
///  ],
///  "properties": {
///    "displayDetail": {
///      "$ref": "#/$defs/Many!DisplayDetailType"
///    },
///    "id": {
///      "$ref": "#/$defs/UriTypeType"
///    },
///    "language": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "type": {
///      "const": "IndividualDisplay"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct IndividualDisplay {
    pub display_detail: ObjectOrVector<DisplayDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    pub language: Concept,
    #[serde(rename = "type")]
    pub type_: IndividualDisplayTag,
}

///IntegerType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer"
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IntegerType(pub i64);
impl std::ops::Deref for IntegerType {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}

///IriType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string"
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IriType(pub String);
impl std::ops::Deref for IriType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

///IssuerNodeType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "eidasLegalIdentifier"
///  ],
///  "properties": {
///    "eidasLegalIdentifier": {
///      "$ref": "#/$defs/LegalIdentifierType"
///    },
///    "id": {
///      "$ref": "#/$defs/UriTypeType"
///    },
///    "type": {
///      "const": "IssuerNode"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct IssuerNode {
    #[serde(rename = "eidasLegalIdentifier")]
    pub eidas_legal_identifier: LegalIdentifier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(rename = "type")]
    pub type_: IssuerNodeTag,
}

///LangStringType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "allOf": [
///    {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    {
///      "type": "object",
///      "maxProperties": 1
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LangStringType(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for LangStringType {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
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
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub awarding_opportunity: Option<ObjectOrVector<AwardingOpportunity>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit_point: Option<ObjectOrVector<CreditPoint>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub education_level: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub education_subject: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entitles_to: Option<ObjectOrVector<LearningEntitlementSpecification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_requirement: Option<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generalisation_of: Option<ObjectOrVector<LearningAchievementSpecificationOrQualification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<ObjectOrVector<LearningAchievementSpecificationOrQualification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub influenced_by: Option<Box<ObjectOrVector<LearningActivitySpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<ObjectOrVector<LearningAchievementSpecificationOrQualification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_outcome: Option<ObjectOrVector<LearningOutcome>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_outcome_summary: Option<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_setting: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_duration: Option<DurationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proven_by: Option<Box<ObjectOrVector<LearningAssessmentSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specialisation_of: Option<ObjectOrVector<LearningAchievementSpecificationOrQualification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_group: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thematic_area: Option<ObjectOrVector<Concept>>,
    pub title: LangKVPair,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_of_learning: Option<DurationType>,
    #[serde(rename = "type")]
    pub type_: LearningAchievementSpecificationTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LearningAchievement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    pub awarded_by: Box<AwardingProcess>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit_received: Option<ObjectOrVector<CreditPoint>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entitles_to: Option<ObjectOrVector<LearningEntitlement>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<Box<ObjectOrVector<LearningAchievement>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub influenced_by: Option<ObjectOrVector<LearningActivity>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<Box<ObjectOrVector<LearningAchievement>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_opportunity: Option<LearningOpportunity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proven_by: Option<Box<ObjectOrVector<LearningAssessment>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specified_by: Option<LearningAchievementSpecificationOrQualification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    pub title: LangKVPair,
    #[serde(rename = "type")]
    pub type_: LearningAchievementTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LearningActivitySpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_hour: Option<ObjectOrVector<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generalisation_of: Option<ObjectOrVector<LearningAchievementSpecification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<ObjectOrVector<LearningAchievementSpecification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ObjectOrVector<IdentifierOrLegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub influences: Option<ObjectOrVector<LearningAchievementSpecificationOrQualification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<ObjectOrVector<LearningAchievementSpecification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specialisation_of: Option<ObjectOrVector<LearningAchievementSpecification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    pub title: LangKVPair,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_of_learning: Option<DurationType>,
    #[serde(rename = "type")]
    pub type_: LearningActivitySpecificationTag,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LearningActivity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    pub awarded_by: Box<AwardingProcess>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directed_by: Option<ObjectOrVector<AgentOrPersonOrOrganisation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<Box<ObjectOrVector<LearningActivity>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ObjectOrVector<IdentifierOrLegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub influences: Option<Box<ObjectOrVector<LearningAchievement>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<Box<ObjectOrVector<LearningActivity>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_opportunity: Option<LearningOpportunity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level_of_completion: Option<Percentage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<ObjectOrVector<Location>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specified_by: Option<LearningActivitySpecification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporal: Option<ObjectOrVector<PeriodOfTime>>,
    pub title: LangKVPair,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workload: Option<DurationType>,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct LearningAssessmentSpecification {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<LangKVPair>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTime<Utc>>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPair>,
    #[serde(rename = "generalisationOf", default)]
    pub generalisation_of: Option<Box<ObjectOrVector<LearningAssessmentSpecification>>>,
    #[serde(rename = "gradingScheme", default, skip_serializing_if = "Option::is_none")]
    pub grading_scheme: Option<GradingScheme>,
    #[serde(rename = "hasPart", default)]
    pub has_part: Option<Box<ObjectOrVector<LearningAssessmentSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ObjectOrVector<IdentifierOrLegalIdentifier>>,
    #[serde(rename = "isPartOf", default)]
    pub is_part_of: Option<Box<ObjectOrVector<LearningAssessmentSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proves: Option<ObjectOrVector<LearningAchievementSpecificationOrQualification>>,
    #[serde(rename = "specialisationOf", default)]
    pub specialisation_of: Option<Box<ObjectOrVector<LearningAssessmentSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "supplementaryDocument", default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    pub title: LangKVPair,
    #[serde(rename = "type")]
    pub type_: LearningAssessmentSpecificationTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LearningAssessment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assessed_by: Option<ObjectOrVector<AgentOrPersonOrOrganisation>>,
    pub awarded_by: Box<AwardingProcess>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_issued: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPair>,
    pub grade: Note,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade_status: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<Box<ObjectOrVector<LearningAssessment>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_verification: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ObjectOrVector<IdentifierOrLegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<Box<ObjectOrVector<LearningAssessment>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proves: Option<ObjectOrVector<LearningAchievement>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_distribution: Option<ResultDistribution>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shortened_grading: Option<ShortenedGrading>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specified_by: Option<ObjectOrVector<LearningAssessmentSpecification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    pub title: LangKVPair,
    #[serde(rename = "type")]
    pub type_: LearningAssessmentTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LearningEntitlementSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTime<Utc>>,
    pub dc_type: ObjectOrVector<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entitled_by: Option<ObjectOrVector<LearningAchievementSpecificationOrQualification>>,
    pub entitlement_status: Concept,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generalisation_of: Option<Box<ObjectOrVector<LearningEntitlementSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<Box<ObjectOrVector<LearningEntitlementSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ObjectOrVector<IdentifierOrLegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<Box<ObjectOrVector<LearningEntitlementSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_jurisdiction: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_national_occupation: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_occupation: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_organisation: Option<Box<ObjectOrVector<Organisation>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specialisation_of: Option<Box<ObjectOrVector<LearningEntitlementSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    pub title: LangKVPair,
    #[serde(rename = "type")]
    pub type_: LearningEntitlementSpecificationTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LearningEntitlement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    pub awarded_by: Box<AwardingProcess>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_issued: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entitled_by: Option<Box<ObjectOrVector<LearningAchievement>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<Box<ObjectOrVector<LearningEntitlement>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ObjectOrVector<IdentifierOrLegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<Box<ObjectOrVector<LearningEntitlement>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specified_by: Option<ObjectOrVector<LearningEntitlementSpecification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    pub title: LangKVPair,
    #[serde(rename = "type")]
    pub type_: LearningEntitlementTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LearningOpportunity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admission_procedure: Option<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_deadline: Option<ObjectOrVector<DateTime<Utc>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner_image: Option<MediaObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_language: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_html: Option<ObjectOrVector<HtmlType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<DurationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant: Option<ObjectOrVector<Grant>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<ObjectOrVector<LearningOpportunity>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ObjectOrVector<IdentifierOrLegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<ObjectOrVector<LearningOpportunity>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_achievement_specification: Option<LearningAchievementSpecificationOrQualification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_activity_specification: Option<LearningActivitySpecification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_schedule: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<ObjectOrVector<Location>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_detail: Option<ObjectOrVector<PriceDetail>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provided_by: Option<Box<Organisation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_information: Option<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporal: Option<PeriodOfTime>,
    pub title: LangKVPair,
    #[serde(rename = "type")]
    pub type_: LearningOpportunityTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LearningOutcome {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifier>,
    #[serde(rename = "relatedESCOSkill", default, skip_serializing_if = "Option::is_none")]
    pub related_esco_skill: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_skill: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reusability_level: Option<Concept>,
    pub title: LangKVPair,
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
    pub dc_type: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    pub notation: Literal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme_agency: Option<LangStringType>,
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
    pub address: Option<ObjectOrVector<Address>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geographic_name: Option<ObjectOrVector<Address>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geometry: Option<ObjectOrVector<Geometry>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spatial_code: Option<ObjectOrVector<Concept>>,
    #[serde(rename = "type")]
    pub type_: LocationTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct Mailbox {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Email>,
    #[serde(rename = "type")]
    pub type_: MailboxTag,
}

///ManyLangStringType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "minProperties": 1,
///  "propertyNames": {
///    "pattern": "^(aa|ab|ae|af|ak|am|an|ar|as|av|ay|az|ba|be|bg|bh|bi|bm|bn|bo|br|bs|ca|ce|ch|co|cr|cs|cu|cv|cy|da|de|dv|dz|ee|el|en|eo|es|et|eu|fa|ff|fi|fj|fo|fr|fy|ga|gd|gl|gn|gu|gv|ha|he|hi|ho|hr|ht|hu|hy|hz|ia|id|ie|ig|ii|ik|in|io|is|it|iu|iw|ja|ji|jv|jw|ka|kg|ki|kj|kk|kl|km|kn|ko|kr|ks|ku|kv|kw|ky|la|lb|lg|li|ln|lo|lt|lu|lv|mg|mh|mi|mk|ml|mn|mo|mr|ms|mt|my|na|nb|nd|ne|ng|nl|nn|no|nr|nv|ny|oc|oj|om|or|os|pa|pi|pl|ps|pt|qu|rm|rn|ro|ru|rw|sa|sc|sd|se|sg|sh|si|sk|sl|sm|sn|so|sq|sr|ss|st|su|sv|sw|ta|te|tg|th|ti|tk|tl|tn|to|tr|ts|tt|tw|ty|ug|uk|ur|uz|ve|vi|vo|wa|wo|xh|yi|yo|za|zh|zu)$"
///  }
///}
/// ```
/// </details>
/// TODO refactor!!!
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LangKVPair(pub HashMap<LangKey, serde_json::Value>);
impl std::ops::Deref for LangKVPair {
    type Target = std::collections::HashMap<LangKey, serde_json::Value>;
    fn deref(&self) -> &std::collections::HashMap<LangKey, serde_json::Value> {
        &self.0
    }
}
impl From<LangKVPair> for std::collections::HashMap<LangKey, serde_json::Value> {
    fn from(value: LangKVPair) -> Self {
        value.0
    }
}
impl From<&LangKVPair> for LangKVPair {
    fn from(value: &LangKVPair) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<LangKey, serde_json::Value>> for LangKVPair {
    fn from(value: std::collections::HashMap<LangKey, serde_json::Value>) -> Self {
        Self(value)
    }
}

///ManyLangStringTypeKey
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": "^(aa|ab|ae|af|ak|am|an|ar|as|av|ay|az|ba|be|bg|bh|bi|bm|bn|bo|br|bs|ca|ce|ch|co|cr|cs|cu|cv|cy|da|de|dv|dz|ee|el|en|eo|es|et|eu|fa|ff|fi|fj|fo|fr|fy|ga|gd|gl|gn|gu|gv|ha|he|hi|ho|hr|ht|hu|hy|hz|ia|id|ie|ig|ii|ik|in|io|is|it|iu|iw|ja|ji|jv|jw|ka|kg|ki|kj|kk|kl|km|kn|ko|kr|ks|ku|kv|kw|ky|la|lb|lg|li|ln|lo|lt|lu|lv|mg|mh|mi|mk|ml|mn|mo|mr|ms|mt|my|na|nb|nd|ne|ng|nl|nn|no|nr|nv|ny|oc|oj|om|or|os|pa|pi|pl|ps|pt|qu|rm|rn|ro|ru|rw|sa|sc|sd|se|sg|sh|si|sk|sl|sm|sn|so|sq|sr|ss|st|su|sv|sw|ta|te|tg|th|ti|tk|tl|tn|to|tr|ts|tt|tw|ty|ug|uk|ur|uz|ve|vi|vo|wa|wo|xh|yi|yo|za|zh|zu)$"
///}
/// ```
/// </details>
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_url: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<LangKVPair>,
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
    pub note_literal: LangKVPair,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<Concept>,
    #[serde(rename = "type")]
    pub type_: NoteTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Organisation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accreditation: Option<ObjectOrVector<Accreditation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_point: Option<ObjectOrVector<ContactPoint>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<Concept>>,
    #[serde(rename = "eIDASIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub e_idas_identifier: Option<LegalIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_member_of: Option<ObjectOrVector<Group>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_member: Option<ObjectOrVector<Person>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_sub_organization: Option<Box<ObjectOrVector<Organisation>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifier>,
    pub legal_name: LangKVPair,
    pub location: ObjectOrVector<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo: Option<MediaObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration: Option<LegalIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_organization_of: Option<Box<Organisation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_identifier: Option<ObjectOrVector<LegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vat_identifier: Option<ObjectOrVector<LegalIdentifier>>,
    #[serde(rename = "type")]
    pub type_: OrganisationTag,
}

///PercentageIntegerType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "maximum": 100,
///  "minimum": 0
///}
/// ```
/// </details>
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

///PeriodOfTimeType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "endDate": {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    "id": {
///      "$ref": "#/$defs/UriTypeType"
///    },
///    "prefLabel": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "startDate": {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    "type": {
///      "const": "PeriodOfTime"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct PeriodOfTime {
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(rename = "prefLabel", default, skip_serializing_if = "Option::is_none")]
    pub pref_label: Option<LangKVPair>,
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateTime<Utc>>,
    #[serde(rename = "type")]
    pub type_: PeriodOfTimeTag,
}

#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Person {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birth_name: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citizenship_country: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_point: Option<ObjectOrVector<ContactPoint>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family_name: Option<LangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<LangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<LangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_member_of: Option<ObjectOrVector<Group>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_claim: Option<ObjectOrVector<ClaimNode>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<ObjectOrVector<EuropeanDigitalCredential>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ObjectOrVector<IdentifierOrLegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_of: Option<Box<ObjectOrVector<Organisation>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub national_id: Option<LegalIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patronymic_name: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place_of_birth: Option<Location>,
    #[serde(rename = "type")]
    pub type_: PersonTag,
}

//pub struct A;

//impl Default for A {
//fn default() -> Self {
//A
//}
//}

///PhoneType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "areaDialing": {
///      "$ref": "#/$defs/StringType"
///    },
///    "countryDialing": {
///      "$ref": "#/$defs/StringType"
///    },
///    "dialNumber": {
///      "$ref": "#/$defs/StringType"
///    },
///    "id": {
///      "$ref": "#/$defs/UriTypeType"
///    },
///    "phoneNumber": {
///      "$ref": "#/$defs/StringType"
///    },
///    "type": {
///      "const": "Phone"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
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

///PriceDetailType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "additionalNote": {
///      "$ref": "#/$defs/Many!NoteType"
///    },
///    "amount": {
///      "$ref": "#/$defs/AmountType"
///    },
///    "description": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "id": {
///      "$ref": "#/$defs/UriTypeType"
///    },
///    "identifier": {
///      "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
///    },
///    "prefLabel": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "type": {
///      "const": "PriceDetail"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PriceDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ObjectOrVector<IdentifierOrLegalIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pref_label: Option<LangKVPair>,
    #[serde(rename = "type")]
    pub type_: PriceDetailTag,
}

///ProofType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "id": {
///      "$ref": "#/$defs/UriTypeType"
///    },
///    "type": {
///      "const": "Proof"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields)]
pub struct Proof {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(rename = "type")]
    pub type_: ProofTag,
}

///QualificationType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "title"
///  ],
///  "properties": {
///    "accreditation": {
///      "$ref": "#/$defs/Many!AccreditationType"
///    },
///    "additionalNote": {
///      "$ref": "#/$defs/Many!NoteType"
///    },
///    "altLabel": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "awardingOpportunity": {
///      "$ref": "#/$defs/Many!AwardingOpportunityType"
///    },
///    "category": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "creditPoint": {
///      "$ref": "#/$defs/Many!CreditPointType"
///    },
///    "dateModified": {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    "dcType": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "description": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "educationLevel": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "educationSubject": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "entitlesTo": {
///      "$ref": "#/$defs/Many!LearningEntitlementSpecificationType"
///    },
///    "entryRequirement": {
///      "$ref": "#/$defs/NoteType"
///    },
///    "eqfLevel": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "generalisationOf": {
///      "$ref": "#/$defs/Many!QualificationType"
///    },
///    "hasPart": {
///      "$ref": "#/$defs/Many!QualificationType"
///    },
///    "homepage": {
///      "$ref": "#/$defs/Many!WebResourceType"
///    },
///    "id": {
///      "$ref": "#/$defs/UriTypeType"
///    },
///    "identifier": {
///      "$ref": "#/$defs/IdentifierOrLegalIdentifierType"
///    },
///    "influencedBy": {
///      "$ref": "#/$defs/Many!LearningActivitySpecificationType"
///    },
///    "isPartOf": {
///      "$ref": "#/$defs/Many!QualificationType"
///    },
///    "isPartialQualification": {
///      "$ref": "#/$defs/BooleanType"
///    },
///    "language": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "learningOutcome": {
///      "$ref": "#/$defs/Many!LearningOutcomeType"
///    },
///    "learningOutcomeSummary": {
///      "$ref": "#/$defs/NoteType"
///    },
///    "learningSetting": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "maximumDuration": {
///      "$ref": "#/$defs/DurationType"
///    },
///    "mode": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "nqfLevel": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "provenBy": {
///      "$ref": "#/$defs/Many!LearningAssessmentSpecificationType"
///    },
///    "qualificationCode": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "specialisationOf": {
///      "$ref": "#/$defs/Many!QualificationType"
///    },
///    "status": {
///      "$ref": "#/$defs/StringType"
///    },
///    "supplementaryDocument": {
///      "$ref": "#/$defs/Many!WebResourceType"
///    },
///    "targetGroup": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "thematicArea": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "title": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "type": {
///      "const": "Qualification"
///    },
///    "volumeOfLearning": {
///      "$ref": "#/$defs/DurationType"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize, TagType)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Qualification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accreditation: Option<ObjectOrVector<Accreditation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub awarding_opportunity: Option<ObjectOrVector<AwardingOpportunity>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit_point: Option<ObjectOrVector<CreditPoint>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub education_level: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub education_subject: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entitles_to: Option<ObjectOrVector<LearningEntitlementSpecification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_requirement: Option<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eqf_level: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generalisation_of: Option<ObjectOrVector<Qualification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<ObjectOrVector<Qualification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub influenced_by: Option<ObjectOrVector<LearningActivitySpecification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<ObjectOrVector<Qualification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_partial_qualification: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_outcome: Option<ObjectOrVector<LearningOutcome>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_outcome_summary: Option<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_setting: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_duration: Option<DurationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nqf_level: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proven_by: Option<Box<ObjectOrVector<LearningAssessmentSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qualification_code: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specialisation_of: Option<ObjectOrVector<Qualification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_group: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thematic_area: Option<ObjectOrVector<Concept>>,
    pub title: LangKVPair,
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
    pub description: Option<LangKVPair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_category: Option<ObjectOrVector<ResultCategory>>,
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
    pub percentage_equal: IntegerType,
    pub percentage_higher: IntegerType,
    pub percentage_lower: IntegerType,
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
    pub description: Option<LangKVPair>,
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
    pub title: Option<LangKVPair>,
    #[serde(rename = "type")]
    pub type_: WebResourceTag,
}
