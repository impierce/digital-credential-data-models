use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use types_common::{ObjectOrVector, BoxObjectOrVector};

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
///AccreditationType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "accreditingAgent",
///    "dcType",
///    "title"
///  ],
///  "properties": {
///    "accreditingAgent": {
///      "$ref": "#/$defs/OrganisationType"
///    },
///    "additionalNote": {
///      "$ref": "#/$defs/Many!NoteType"
///    },
///    "dateIssued": {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    "dateModified": {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    "dcType": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "decision": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "description": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "expiryDate": {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    "homepage": {
///      "$ref": "#/$defs/Many!WebResourceType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "identifier": {
///      "$ref": "#/$defs/IdentifierOrLegalIdentifierType"
///    },
///    "landingPage": {
///      "$ref": "#/$defs/Many!WebResourceType"
///    },
///    "limitCredentialType": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "limitEQFLevel": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "limitField": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "limitJurisdiction": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "limitQualification": {
///      "$ref": "#/$defs/QualificationType"
///    },
///    "organisation": {
///      "$ref": "#/$defs/Many!OrganisationType"
///    },
///    "report": {
///      "$ref": "#/$defs/WebResourceType"
///    },
///    "reviewDate": {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    "status": {
///      "$ref": "#/$defs/StringType"
///    },
///    "supplementaryDocument": {
///      "$ref": "#/$defs/Many!WebResourceType"
///    },
///    "title": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "type": {
///      "const": "Accreditation"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Accreditation {
    #[serde(rename = "accreditingAgent")]
    pub accrediting_agent: Organisation,
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(rename = "dateIssued", default, skip_serializing_if = "Option::is_none")]
    pub date_issued: Option<DateTimeType>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTimeType>,
    #[serde(rename = "dcType")]
    pub dc_type: Concept,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<DateTimeType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifierType>,
    #[serde(rename = "landingPage", default, skip_serializing_if = "Option::is_none")]
    pub landing_page: Option<ObjectOrVector<WebResource>>,
    #[serde(
        rename = "limitCredentialType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_credential_type: Option<ObjectOrVector<Concept>>,
    #[serde(rename = "limitEQFLevel", default, skip_serializing_if = "Option::is_none")]
    pub limit_eqf_level: Option<ObjectOrVector<Concept>>,
    #[serde(rename = "limitField", default, skip_serializing_if = "Option::is_none")]
    pub limit_field: Option<ObjectOrVector<Concept>>,
    #[serde(
        rename = "limitJurisdiction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_jurisdiction: Option<ObjectOrVector<Concept>>,
    #[serde(
        rename = "limitQualification",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_qualification: Option<Qualification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organisation: Option<ObjectOrVector<Organisation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report: Option<WebResource>,
    #[serde(rename = "reviewDate", default, skip_serializing_if = "Option::is_none")]
    pub review_date: Option<DateTimeType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StringType>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///AddressType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "countryCode"
///  ],
///  "properties": {
///    "countryCode": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "fullAddress": {
///      "$ref": "#/$defs/NoteType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "identifier": {
///      "$ref": "#/$defs/IdentifierOrLegalIdentifierType"
///    },
///    "type": {
///      "const": "Address"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Address {
    #[serde(rename = "countryCode")]
    pub country_code: Concept,
    #[serde(rename = "fullAddress", default, skip_serializing_if = "Option::is_none")]
    pub full_address: Option<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifierType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///AgentOrPersonOrOrganisationType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/AgentType"
///    },
///    {
///      "$ref": "#/$defs/PersonType"
///    },
///    {
///      "$ref": "#/$defs/OrganisationType"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AgentOrPersonOrOrganisationType {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub agent_type: Option<AgentType>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub person: Option<Box<Person>>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub organisation: Option<Box<Organisation>>,
}

///AgentType
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
///    "altLabel": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "contactPoint": {
///      "$ref": "#/$defs/Many!ContactPointType"
///    },
///    "dateModified": {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    "groupMemberOf": {
///      "$ref": "#/$defs/Many!GroupType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "identifier": {
///      "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
///    },
///    "location": {
///      "$ref": "#/$defs/Many!LocationType"
///    },
///    "prefLabel": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "type": {
///      "const": "Agent"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AgentType {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<ManyLangStringType>,
    #[serde(rename = "contactPoint", default, skip_serializing_if = "Option::is_none")]
    pub contact_point: Option<ObjectOrVector<ContactPoint>>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTimeType>,
    #[serde(rename = "groupMemberOf", default, skip_serializing_if = "Option::is_none")]
    pub group_member_of: Option<ObjectOrVector<Group>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<ObjectOrVector<Location>>,
    #[serde(rename = "prefLabel", default, skip_serializing_if = "Option::is_none")]
    pub pref_label: Option<ManyLangStringType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///AmountType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "unit",
///    "value"
///  ],
///  "properties": {
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "type": {
///      "const": "Amount"
///    },
///    "unit": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "value": {
///      "$ref": "#/$defs/DecimalType"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AmountType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
    pub unit: Concept,
    pub value: DecimalType,
}

///AwardingOpportunityType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "awardingBody"
///  ],
///  "properties": {
///    "awardingBody": {
///      "$ref": "#/$defs/Many!AgentOrPersonOrOrganisationType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "identifier": {
///      "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
///    },
///    "learningAchievementSpecification": {
///      "$ref": "#/$defs/LearningAchievementSpecificationOrQualificationType"
///    },
///    "location": {
///      "$ref": "#/$defs/LocationType"
///    },
///    "temporal": {
///      "$ref": "#/$defs/PeriodOfTimeType"
///    },
///    "type": {
///      "const": "AwardingOpportunity"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AwardingOpportunity {
    #[serde(rename = "awardingBody")]
    pub awarding_body: ManyAgentOrPersonOrOrganisationType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(rename = "learningAchievementSpecification", default)]
    pub learning_achievement_specification: 
        Option<Box<LearningAchievementSpecificationOrQualificationType>,
    >,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporal: Option<PeriodOfTime>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///AwardingProcessType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "awardingBody"
///  ],
///  "properties": {
///    "additionalNote": {
///      "$ref": "#/$defs/Many!NoteType"
///    },
///    "awardingBody": {
///      "$ref": "#/$defs/Many!AgentOrPersonOrOrganisationType"
///    },
///    "awardingDate": {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    "awards": {
///      "$ref": "#/$defs/Many!ClaimNodeType"
///    },
///    "description": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "educationalSystemNote": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "identifier": {
///      "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
///    },
///    "location": {
///      "$ref": "#/$defs/LocationType"
///    },
///    "type": {
///      "const": "AwardingProcess"
///    },
///    "used": {
///      "$ref": "#/$defs/Many!LearningAssessmentType"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AwardingProcess {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(rename = "awardingBody")]
    pub awarding_body: ManyAgentOrPersonOrOrganisationType,
    #[serde(rename = "awardingDate", default, skip_serializing_if = "Option::is_none")]
    pub awarding_date: Option<DateTimeType>,
    #[serde(default)]
    pub awards: Option<Box<ManyClaimNodeType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(
        rename = "educationalSystemNote",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub educational_system_note: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used: Option<ObjectOrVector<LearningAssessment>>,
}

///BooleanType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": "^(true|false|1|0)$"
///}
/// ```
/// </details>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BooleanType(String);
impl std::ops::Deref for BooleanType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for BooleanType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^(true|false|1|0)$").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \"^(true|false|1|0)$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl std::convert::TryFrom<&str> for BooleanType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for BooleanType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for BooleanType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for BooleanType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}

///ClaimNodeType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/LearningAchievementType"
///    },
///    {
///      "$ref": "#/$defs/LearningActivityType"
///    },
///    {
///      "$ref": "#/$defs/LearningAssessmentType"
///    },
///    {
///      "$ref": "#/$defs/LearningEntitlementType"
///    },
///    {
///      "$ref": "#/$defs/ClaimTypeNodeType"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClaimNode {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub learning_achievement: Option<LearningAchievement>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub learning_activity: Option<LearningActivity>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub learning_assessment: Option<LearningAssessment>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub learning_entitlement: Option<LearningEntitlement>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub claim_node_type: Option<ClaimTypeNodeType>,
}

///ClaimTypeNodeType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "awardedBy",
///    "title"
///  ],
///  "properties": {
///    "additionalNote": {
///      "$ref": "#/$defs/Many!NoteType"
///    },
///    "awardedBy": {
///      "$ref": "#/$defs/AwardingProcessType"
///    },
///    "description": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
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
///      "const": "ClaimTypeNode"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimTypeNodeType {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(rename = "awardedBy")]
    pub awarded_by: AwardingProcess,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///ConceptSchemeType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "type": {
///      "const": "ConceptScheme"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConceptScheme {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///ConceptType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "definition": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "inScheme": {
///      "$ref": "#/$defs/ConceptSchemeType"
///    },
///    "notation": {
///      "$ref": "#/$defs/LiteralType"
///    },
///    "prefLabel": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "type": {
///      "const": "Concept"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Concept {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<ManyLangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "inScheme", default, skip_serializing_if = "Option::is_none")]
    pub in_scheme: Option<ConceptScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notation: Option<Literal>,
    #[serde(rename = "prefLabel", default, skip_serializing_if = "Option::is_none")]
    pub pref_label: Option<ManyLangStringType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///ContactPointType
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
///    "address": {
///      "$ref": "#/$defs/Many!AddressType"
///    },
///    "contactForm": {
///      "$ref": "#/$defs/Many!WebResourceType"
///    },
///    "description": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "emailAddress": {
///      "$ref": "#/$defs/Many!MailboxType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "phone": {
///      "$ref": "#/$defs/Many!PhoneType"
///    },
///    "type": {
///      "const": "ContactPoint"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContactPoint {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<ObjectOrVector<Address>>,
    #[serde(rename = "contactForm", default, skip_serializing_if = "Option::is_none")]
    pub contact_form: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<ObjectOrVector<Mailbox>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<ObjectOrVector<Phone>>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///Contains information about the credential schema on which the issued credential is based
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Contains information about the credential schema on which the issued credential is based",
///  "type": "object",
///  "required": [
///    "id",
///    "type"
///  ],
///  "properties": {
///    "id": {
///      "description": "References the credential schema stored on the Trusted Schemas Registry (TSR)",
///      "type": "string",
///      "format": "uri"
///    },
///    "type": {
///      "description": "Defines credential schema type",
///      "type": "string",
///      "enum": [
///        "JsonSchema",
///        "ShaclValidator2017"
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CredentialSchema {
    ///References the credential schema stored on the Trusted Schemas Registry (TSR)
    pub id: String,
    ///Defines credential schema type
    #[serde(rename = "type")]
    pub type_: CredentialSchemaType,
}

///Defines credential schema type
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Defines credential schema type",
///  "type": "string",
///  "enum": [
///    "JsonSchema",
///    "ShaclValidator2017"
///  ]
///}
/// ```
/// </details>
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum CredentialSchemaType {
    JsonSchema,
    ShaclValidator2017,
}

impl From<&CredentialSchemaType> for CredentialSchemaType {
    fn from(value: &CredentialSchemaType) -> Self {
        value.clone()
    }
}

impl ToString for CredentialSchemaType {
    fn to_string(&self) -> String {
        match *self {
            Self::JsonSchema => "JsonSchema".to_string(),
            Self::ShaclValidator2017 => "ShaclValidator2017".to_string(),
        }
    }
}

impl std::str::FromStr for CredentialSchemaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "JsonSchema" => Ok(Self::JsonSchema),
            "ShaclValidator2017" => Ok(Self::ShaclValidator2017),
            _ => Err("invalid value".into()),
        }
    }
}

impl std::convert::TryFrom<&str> for CredentialSchemaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for CredentialSchemaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for CredentialSchemaType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

///CredentialStatusType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "type": {
///      "const": "CredentialStatus"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CredentialStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///Defines information about the subject that is defined by the type chain
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Defines information about the subject that is defined by the type chain",
///  "type": "object",
///  "properties": {
///    "id": {
///      "description": "Defines the DID of the subject that is described by the issued credential",
///      "type": "string",
///      "format": "uri"
///    }
///  }
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CredentialSubject {
    ///Defines the DID of the subject that is described by the issued credential
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

///CredentialSubjectType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$ref": "#/$defs/AgentOrPersonOrOrganisationType"
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CredentialSubjectType(pub AgentOrPersonOrOrganisationType);
impl std::ops::Deref for CredentialSubjectType {
    type Target = AgentOrPersonOrOrganisationType;
    fn deref(&self) -> &AgentOrPersonOrOrganisationType {
        &self.0
    }
}

impl From<CredentialSubjectType> for AgentOrPersonOrOrganisationType {
    fn from(value: CredentialSubjectType) -> Self {
        value.0
    }
}

impl From<&CredentialSubjectType> for CredentialSubjectType {
    fn from(value: &CredentialSubjectType) -> Self {
        value.clone()
    }
}

impl From<AgentOrPersonOrOrganisationType> for CredentialSubjectType {
    fn from(value: AgentOrPersonOrOrganisationType) -> Self {
        Self(value)
    }
}

///CreditPointType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "framework",
///    "point"
///  ],
///  "properties": {
///    "framework": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "point": {
///      "$ref": "#/$defs/StringType"
///    },
///    "type": {
///      "const": "CreditPoint"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CreditPoint {
    pub framework: Concept,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    pub point: StringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///DateTimeType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "format": "date-time"
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DateTimeType(pub chrono::DateTime<chrono::offset::Utc>);
impl std::ops::Deref for DateTimeType {
    type Target = chrono::DateTime<chrono::offset::Utc>;
    fn deref(&self) -> &chrono::DateTime<chrono::offset::Utc> {
        &self.0
    }
}

impl From<DateTimeType> for chrono::DateTime<chrono::offset::Utc> {
    fn from(value: DateTimeType) -> Self {
        value.0
    }
}

impl From<&DateTimeType> for DateTimeType {
    fn from(value: &DateTimeType) -> Self {
        value.clone()
    }
}

impl From<chrono::DateTime<chrono::offset::Utc>> for DateTimeType {
    fn from(value: chrono::DateTime<chrono::offset::Utc>) -> Self {
        Self(value)
    }
}

impl std::str::FromStr for DateTimeType {
    type Err = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}

impl std::convert::TryFrom<&str> for DateTimeType {
    type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for DateTimeType {
    type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for DateTimeType {
    type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl ToString for DateTimeType {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

///DecimalType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "number"
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DecimalType(pub f64);
impl std::ops::Deref for DecimalType {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}

impl From<DecimalType> for f64 {
    fn from(value: DecimalType) -> Self {
        value.0
    }
}

impl From<&DecimalType> for DecimalType {
    fn from(value: &DecimalType) -> Self {
        value.clone()
    }
}

impl From<f64> for DecimalType {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl std::str::FromStr for DecimalType {
    type Err = <f64 as std::str::FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}

impl std::convert::TryFrom<&str> for DecimalType {
    type Error = <f64 as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for DecimalType {
    type Error = <f64 as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for DecimalType {
    type Error = <f64 as std::str::FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl ToString for DecimalType {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

///DisplayDetailType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "image",
///    "page"
///  ],
///  "properties": {
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "image": {
///      "$ref": "#/$defs/MediaObjectType"
///    },
///    "page": {
///      "$ref": "#/$defs/PositiveIntegerType"
///    },
///    "type": {
///      "const": "DisplayDetail"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DisplayDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    pub image: MediaObject,
    pub page: PositiveIntegerType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
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
///      "$ref": "#/$defs/GenericIdType"
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
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DisplayParameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "individualDisplay")]
    pub individual_display: ObjectOrVector<IndividualDisplay>,
    pub language: ObjectOrVector<Concept>,
    #[serde(rename = "primaryLanguage")]
    pub primary_language: Concept,
    #[serde(rename = "summaryDisplay", default, skip_serializing_if = "Option::is_none")]
    pub summary_display: Option<StringType>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///DurationType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "format": "duration"
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DurationType(pub String);
impl std::ops::Deref for DurationType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl From<DurationType> for String {
    fn from(value: DurationType) -> Self {
        value.0
    }
}

impl From<&DurationType> for DurationType {
    fn from(value: &DurationType) -> Self {
        value.clone()
    }
}

impl From<String> for DurationType {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl std::str::FromStr for DurationType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}

impl ToString for DurationType {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

///EmailType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "anyOf": [
///    {
///      "type": "string",
///      "format": "email"
///    },
///    {
///      "type": "string",
///      "format": "uri",
///      "pattern": "^mailto:[^@]*[^\\.]@[^\\.]($|[^@]*[^\\.]$)"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmailType {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<String>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<String>,
}

///Schema for EDC credential based on ELM 3.2
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Europass EDC credential",
///  "description": "Schema for EDC credential based on ELM 3.2",
///  "type": "object",
///  "allOf": [
///    {
///      "title": "EBSI Verifiable Attestation",
///      "description": "The schema defines a generic structure for any EBSI-related Verifiable Credentials according to the VCDM v2.0",
///      "type": "object",
///      "required": [
///        "@context",
///        "credentialSchema",
///        "credentialSubject",
///        "id",
///        "issuer",
///        "type",
///        "validFrom"
///      ],
///      "properties": {
///        "@context": {
///          "oneOf": [
///            {
///              "const": "https://www.w3.org/ns/credentials/v2"
///            },
///            {
///              "description": "Semantic context for the issued credential. First element MUST be https://www.w3.org/ns/credentials/v2",
///              "type": "array",
///              "items": {
///                "type": "string",
///                "format": "uri"
///              },
///              "minItems": 1,
///              "uniqueItems": true
///            }
///          ]
///        },
///        "credentialSchema": {
///          "description": "One or more schemas that validate the Verifiable Credential.",
///          "anyOf": [
///            {
///              "$ref": "#/$defs/credentialSchema"
///            },
///            {
///              "type": "array",
///              "items": {
///                "$ref": "#/$defs/credentialSchema"
///              }
///            }
///          ]
///        },
///        "credentialStatus": {
///          "description": "Defines suspension and/or revocation details for the issued credential. Further redefined by the type extension",
///          "type": "object",
///          "required": [
///            "id",
///            "type"
///          ],
///          "properties": {
///            "id": {
///              "description": "Exact identity for the credential status",
///              "type": "string",
///              "format": "uri"
///            },
///            "type": {
///              "description": "Defines the revocation type extension",
///              "type": "string"
///            }
///          }
///        },
///        "credentialSubject": {
///          "anyOf": [
///            {
///              "$ref": "#/$defs/credentialSubject"
///            },
///            {
///              "type": "array",
///              "items": {
///                "$ref": "#/$defs/credentialSubject"
///              }
///            }
///          ]
///        },
///        "evidence": {
///          "anyOf": [
///            {
///              "$ref": "#/$defs/evidence"
///            },
///            {
///              "type": "array",
///              "items": {
///                "$ref": "#/$defs/evidence"
///              }
///            }
///          ]
///        },
///        "id": {
///          "description": "Globally unique identifier for the issued credential. It can be a UUID or another globally unique identifier.",
///          "type": "string",
///          "format": "uri"
///        },
///        "issuer": {
///          "description": "DID of the credential issuer",
///          "oneOf": [
///            {
///              "type": "string",
///              "format": "uri"
///            },
///            {
///              "type": "object",
///              "required": [
///                "id"
///              ],
///              "properties": {
///                "id": {
///                  "description": "DID of the credential issuer",
///                  "type": "string",
///                  "format": "uri"
///                }
///              }
///            }
///          ]
///        },
///        "termsOfUse": {
///          "anyOf": [
///            {
///              "$ref": "#/$defs/termsOfUse"
///            },
///            {
///              "type": "array",
///              "items": {
///                "$ref": "#/$defs/termsOfUse"
///              }
///            }
///          ]
///        },
///        "type": {
///          "description": "Full type chain, used to identify the credential base types",
///          "type": "array",
///          "items": {
///            "type": "string"
///          }
///        },
///        "validFrom": {
///          "description": "Defines the earliest point when the credential becomes valid.",
///          "type": "string",
///          "format": "date-time"
///        },
///        "validUntil": {
///          "description": "Defines the latest point when the credential ceases to be valid.",
///          "type": "string",
///          "format": "date-time"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "credentialSubject"
///      ],
///      "properties": {
///        "credentialProfiles": {
///          "$ref": "#/$defs/Many!ConceptType"
///        },
///        "credentialSubject": {
///          "description": "Defines additional information about the subject that is described by the Verifiable Accreditation",
///          "$ref": "#/$defs/CredentialSubjectType"
///        },
///        "displayParameter": {
///          "$ref": "#/$defs/DisplayParameterType"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EuropassEdcCredential {
    #[serde(rename = "@context")]
    pub context: EuropassEdcCredentialContext,
    #[serde(
        rename = "credentialProfiles",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub credential_profiles: Option<ObjectOrVector<Concept>>,
    ///One or more schemas that validate the Verifiable Credential.
    #[serde(rename = "credentialSchema")]
    pub credential_schema: ObjectOrVector<CredentialSchema>,
    #[serde(
        rename = "credentialStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub credential_status: Option<EuropassEdcCredentialCredentialStatus>,
    #[serde(rename = "credentialSubject")]
    pub credential_subject: EuropassEdcCredentialCredentialSubject,
    #[serde(
        rename = "displayParameter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_parameter: Option<DisplayParameter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence: Option<ObjectOrVector<Evidence>>,
    ///Globally unique identifier for the issued credential. It can be a UUID or another globally unique identifier.
    pub id: String,
    ///DID of the credential issuer
    pub issuer: EuropassEdcCredentialIssuer,
    #[serde(rename = "termsOfUse", default, skip_serializing_if = "Option::is_none")]
    pub terms_of_use: Option<ObjectOrVector<TermsOfUse>>,
    ///Full type chain, used to identify the credential base types
    #[serde(rename = "type")]
    pub type_: Vec<String>,
    ///Defines the earliest point when the credential becomes valid.
    #[serde(rename = "validFrom")]
    pub valid_from: DateTime<Utc>,
    ///Defines the latest point when the credential ceases to be valid.
    #[serde(rename = "validUntil", default, skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<DateTime<Utc>>,
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
    pub type_: String,
}

///EuropassEdcCredentialCredentialSubject
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "allOf": [
///    {
///      "anyOf": [
///        {
///          "$ref": "#/$defs/AgentType"
///        },
///        {
///          "$ref": "#/$defs/PersonType"
///        },
///        {
///          "$ref": "#/$defs/OrganisationType"
///        }
///      ]
///    },
///    {
///      "$ref": "#/$defs/credentialSubject"
///    },
///    {
///      "not": {
///        "type": "array",
///        "items": {
///          "$ref": "#/$defs/credentialSubject"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum EuropassEdcCredentialCredentialSubject {
    Variant0 {
        #[serde(
            rename = "additionalNote",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        additional_note: Option<ObjectOrVector<Note>>,
        #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
        alt_label: Option<ManyLangStringType>,
        #[serde(
            rename = "contactPoint",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        contact_point: Option<ObjectOrVector<ContactPoint>>,
        #[serde(
            rename = "dateModified",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        date_modified: Option<DateTimeType>,
        #[serde(
            rename = "groupMemberOf",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        group_member_of: Option<ObjectOrVector<Group>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        identifier: Option<ManyIdentifierOrLegalIdentifierType>,
        #[serde(rename = "prefLabel", default, skip_serializing_if = "Option::is_none")]
        pref_label: Option<ManyLangStringType>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        type_: Option<serde_json::Value>,
    },
    Variant1 {
        #[serde(rename = "birthName", default, skip_serializing_if = "Option::is_none")]
        birth_name: Option<ManyLangStringType>,
        #[serde(
            rename = "citizenshipCountry",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        citizenship_country: Option<ObjectOrVector<Concept>>,
        #[serde(
            rename = "contactPoint",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        contact_point: Option<ObjectOrVector<ContactPoint>>,
        #[serde(
            rename = "dateModified",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        date_modified: Option<DateTimeType>,
        #[serde(
            rename = "dateOfBirth",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        date_of_birth: Option<DateTimeType>,
        #[serde(rename = "familyName", default, skip_serializing_if = "Option::is_none")]
        family_name: Option<LangStringType>,
        #[serde(rename = "fullName", default, skip_serializing_if = "Option::is_none")]
        full_name: Option<LangStringType>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        gender: Option<Concept>,
        #[serde(rename = "givenName", default, skip_serializing_if = "Option::is_none")]
        given_name: Option<LangStringType>,
        #[serde(
            rename = "groupMemberOf",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        group_member_of: Option<ObjectOrVector<Group>>,
        #[serde(rename = "hasClaim", default, skip_serializing_if = "Option::is_none")]
        has_claim: Option<ManyClaimNodeType>,
        #[serde(
            rename = "hasCredential",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        has_credential: Option<ObjectOrVector<EuropeanDigitalCredential>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        identifier: Option<IdentifierOrLegalIdentifierType>,
        #[serde(rename = "memberOf", default, skip_serializing_if = "Option::is_none")]
        member_of: Option<ObjectOrVector<Organisation>>,
        #[serde(rename = "nationalID", default, skip_serializing_if = "Option::is_none")]
        national_id: Option<LegalIdentifier>,
        #[serde(
            rename = "patronymicName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        patronymic_name: Option<ManyLangStringType>,
        #[serde(
            rename = "placeOfBirth",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        place_of_birth: Option<Location>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        type_: Option<serde_json::Value>,
    },
    Variant2 {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        accreditation: Option<BoxObjectOrVector<Accreditation>>,
        #[serde(
            rename = "additionalNote",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        additional_note: Option<ObjectOrVector<Note>>,
        #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
        alt_label: Option<ManyLangStringType>,
        #[serde(
            rename = "contactPoint",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        contact_point: Option<ObjectOrVector<ContactPoint>>,
        #[serde(
            rename = "dateModified",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        date_modified: Option<DateTimeType>,
        #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
        dc_type: Option<ObjectOrVector<Concept>>,
        #[serde(
            rename = "eIDASIdentifier",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        e_idas_identifier: Option<LegalIdentifier>,
        #[serde(
            rename = "groupMemberOf",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        group_member_of: Option<ObjectOrVector<Group>>,
        #[serde(rename = "hasMember", default, skip_serializing_if = "Option::is_none")]
        has_member: Option<ObjectOrVector<Person>>,
        #[serde(
            rename = "hasSubOrganization",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        has_sub_organization: Option<ObjectOrVector<Organisation>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        homepage: Option<ObjectOrVector<WebResource>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        identifier: Option<IdentifierOrLegalIdentifierType>,
        #[serde(rename = "legalName")]
        legal_name: ManyLangStringType,
        location: ObjectOrVector<Location>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        logo: Option<MediaObject>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        registration: Option<LegalIdentifier>,
        #[serde(
            rename = "subOrganizationOf",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        sub_organization_of: Option<Box<Organisation>>,
        #[serde(
            rename = "taxIdentifier",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        tax_identifier: Option<ObjectOrVector<LegalIdentifier>>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        type_: Option<serde_json::Value>,
        #[serde(
            rename = "vatIdentifier",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        vat_identifier: Option<ObjectOrVector<LegalIdentifier>>,
    },
}
impl From<&EuropassEdcCredentialCredentialSubject>
for EuropassEdcCredentialCredentialSubject {
    fn from(value: &EuropassEdcCredentialCredentialSubject) -> Self {
        value.clone()
    }
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

///EuropeanDigitalCredentialType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "credentialProfiles",
///    "credentialSchema",
///    "credentialSubject",
///    "displayParameter",
///    "expirationDate",
///    "issued",
///    "issuer",
///    "validFrom"
///  ],
///  "properties": {
///    "attachment": {
///      "$ref": "#/$defs/Many!MediaObjectType"
///    },
///    "credentialProfiles": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "credentialSchema": {
///      "$ref": "#/$defs/Many!ShaclValidator2017Type"
///    },
///    "credentialStatus": {
///      "$ref": "#/$defs/CredentialStatusType"
///    },
///    "credentialSubject": {
///      "$ref": "#/$defs/AgentOrPersonOrOrganisationType"
///    },
///    "displayParameter": {
///      "$ref": "#/$defs/DisplayParameterType"
///    },
///    "evidence": {
///      "$ref": "#/$defs/Many!EvidenceType"
///    },
///    "expirationDate": {
///      "$ref": "#/$defs/Many!DateTimeType"
///    },
///    "holder": {
///      "$ref": "#/$defs/Many!AgentOrPersonOrOrganisationType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "identifier": {
///      "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
///    },
///    "issuanceDate": {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    "issued": {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    "issuer": {
///      "$ref": "#/$defs/AgentOrPersonOrOrganisationType"
///    },
///    "proof": {
///      "$ref": "#/$defs/Many!ProofType"
///    },
///    "termsOfUse": {
///      "$ref": "#/$defs/Many!TermsOfUseType"
///    },
///    "type": {
///      "const": "EuropeanDigitalCredential"
///    },
///    "validFrom": {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    "validUntil": {
///      "$ref": "#/$defs/DateTimeType"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EuropeanDigitalCredential {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment: Option<ObjectOrVector<MediaObject>>,
    #[serde(rename = "credentialProfiles")]
    pub credential_profiles: ObjectOrVector<Concept>,
    #[serde(rename = "credentialSchema")]
    pub credential_schema: ObjectOrVector<ShaclValidator2017>,
    #[serde(
        rename = "credentialStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub credential_status: Option<CredentialStatus>,
    #[serde(rename = "credentialSubject")]
    pub credential_subject: AgentOrPersonOrOrganisationType,
    #[serde(rename = "displayParameter")]
    pub display_parameter: DisplayParameter,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence: Option<ObjectOrVector<Evidence>>,
    #[serde(rename = "expirationDate")]
    pub expiration_date: ObjectOrVector<DateTimeType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holder: Option<ManyAgentOrPersonOrOrganisationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(rename = "issuanceDate", default, skip_serializing_if = "Option::is_none")]
    pub issuance_date: Option<DateTimeType>,
    pub issued: DateTimeType,
    pub issuer: AgentOrPersonOrOrganisationType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof: Option<ObjectOrVector<Proof>>,
    #[serde(rename = "termsOfUse", default, skip_serializing_if = "Option::is_none")]
    pub terms_of_use: Option<ObjectOrVector<TermsOfUse>>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
    #[serde(rename = "validFrom")]
    pub valid_from: DateTimeType,
    #[serde(rename = "validUntil", default, skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<DateTimeType>,
}

///EuropeanDigitalPresentationType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "holder": {
///      "$ref": "#/$defs/Many!AgentOrPersonOrOrganisationType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "proof": {
///      "$ref": "#/$defs/Many!ProofType"
///    },
///    "type": {
///      "const": "EuropeanDigitalPresentation"
///    },
///    "verifiableCredential": {
///      "$ref": "#/$defs/Many!EuropeanDigitalCredentialType"
///    },
///    "verificationCheck": {
///      "$ref": "#/$defs/Many!VerificationCheckType"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EuropeanDigitalPresentationType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holder: Option<ManyAgentOrPersonOrOrganisationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof: Option<ObjectOrVector<Proof>>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
    #[serde(
        rename = "verifiableCredential",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verifiable_credential: Option<ObjectOrVector<EuropeanDigitalCredential>>,
    #[serde(
        rename = "verificationCheck",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verification_check: Option<ObjectOrVector<VerificationCheck>>,
}

///Evidence
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "id": {
///      "description": "If present, it MUST contain a URL that points to where more information about this instance of evidence can be found.",
///      "type": "string"
///    },
///    "type": {
///      "anyOf": [
///        {
///          "description": "Defines the evidence type extension",
///          "type": "string"
///        },
///        {
///          "description": "Defines the evidence type extension",
///          "type": "array",
///          "items": {
///            "type": "string"
///          }
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Evidence {
    ///If present, it MUST contain a URL that points to where more information about this instance of evidence can be found.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub ev_type: EvidenceType,
}

///EvidenceType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "accreditation": {
///      "$ref": "#/$defs/AccreditationType"
///    },
///    "embeddedEvidence": {
///      "$ref": "#/$defs/Many!MediaObjectType"
///    },
///    "evidenceStatement": {
///      "$ref": "#/$defs/StringType"
///    },
///    "evidenceTarget": {
///      "$ref": "#/$defs/AgentOrPersonOrOrganisationType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "type": {
///      "const": "Evidence"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accreditation: Option<Box<Accreditation>>,
    #[serde(
        rename = "embeddedEvidence",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub embedded_evidence: Option<ObjectOrVector<MediaObject>>,
    #[serde(
        rename = "evidenceStatement",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub evidence_statement: Option<StringType>,
    #[serde(rename = "evidenceTarget", default, skip_serializing_if = "Option::is_none")]
    pub evidence_target: Option<Box<AgentOrPersonOrOrganisationType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///GenericIdType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "allOf": [
///    {
///      "$ref": "#/$defs/URIType"
///    },
///    {
///      "type": "string",
///      "pattern": "^(http:%5C/%5C/data%5C.europe%5C.eu%5C/snb%5C/|http:%5C/%5C/publications%5C.europe%5C.eu%5C/resource%5C/authority%5C/|urn:epass:.+:[0-9]+$|urn:epass:concept(Scheme)?:[0-9A-Za-z%5C%5C-]*$)"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GenericIdType(pub String);
impl std::ops::Deref for GenericIdType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl From<GenericIdType> for String {
    fn from(value: GenericIdType) -> Self {
        value.0
    }
}

impl From<&GenericIdType> for GenericIdType {
    fn from(value: &GenericIdType) -> Self {
        value.clone()
    }
}

impl From<String> for GenericIdType {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl std::str::FromStr for GenericIdType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}

impl ToString for GenericIdType {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

///GeometryType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "latitude": {
///      "$ref": "#/$defs/StringType"
///    },
///    "longitude": {
///      "$ref": "#/$defs/StringType"
///    },
///    "type": {
///      "const": "Geometry"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Geometry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latitude: Option<StringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<StringType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
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
///      "$ref": "#/$defs/GenericIdType"
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
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GradingSchemeType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
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
///      "$ref": "#/$defs/GenericIdType"
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
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Grant {
    #[serde(rename = "contentURL", default, skip_serializing_if = "Option::is_none")]
    pub content_url: Option<UriType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
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
///      "$ref": "#/$defs/GenericIdType"
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
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Group {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<ManyLangStringType>,
    #[serde(rename = "contactPoint", default, skip_serializing_if = "Option::is_none")]
    pub contact_point: Option<ObjectOrVector<ContactPoint>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<ObjectOrVector<Location>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<ManyAgentOrPersonOrOrganisationType>,
    #[serde(rename = "prefLabel")]
    pub pref_label: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
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

impl From<HtmlType> for String {
    fn from(value: HtmlType) -> Self {
        value.0
    }
}

impl From<&HtmlType> for HtmlType {
    fn from(value: &HtmlType) -> Self {
        value.clone()
    }
}

impl From<String> for HtmlType {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl std::str::FromStr for HtmlType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}

impl ToString for HtmlType {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

///IdentifierOrLegalIdentifierType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/IdentifierType"
///    },
///    {
///      "$ref": "#/$defs/LegalIdentifierType"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IdentifierOrLegalIdentifierType {
    IdentifierType(IdentifierType),
    LegalIdentifierType(LegalIdentifier),
}

impl From<&IdentifierOrLegalIdentifierType> for IdentifierOrLegalIdentifierType {
    fn from(value: &IdentifierOrLegalIdentifierType) -> Self {
        value.clone()
    }
}

impl From<IdentifierType> for IdentifierOrLegalIdentifierType {
    fn from(value: IdentifierType) -> Self {
        Self::IdentifierType(value)
    }
}

impl From<LegalIdentifier> for IdentifierOrLegalIdentifierType {
    fn from(value: LegalIdentifier) -> Self {
        Self::LegalIdentifierType(value)
    }
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
///      "$ref": "#/$defs/GenericIdType"
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
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IdentifierType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<IriType>,
    #[serde(rename = "dateIssued", default, skip_serializing_if = "Option::is_none")]
    pub date_issued: Option<DateTimeType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    pub notation: Literal,
    #[serde(rename = "schemeAgency", default, skip_serializing_if = "Option::is_none")]
    pub scheme_agency: Option<LangStringType>,
    #[serde(rename = "schemeId", default, skip_serializing_if = "Option::is_none")]
    pub scheme_id: Option<UriType>,
    #[serde(rename = "schemeName", default, skip_serializing_if = "Option::is_none")]
    pub scheme_name: Option<StringType>,
    #[serde(rename = "schemeVersion", default, skip_serializing_if = "Option::is_none")]
    pub scheme_version: Option<StringType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
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
///      "$ref": "#/$defs/GenericIdType"
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
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IndividualDisplay {
    #[serde(rename = "displayDetail")]
    pub display_detail: ObjectOrVector<DisplayDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    pub language: Concept,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
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
impl From<IntegerType> for i64 {
    fn from(value: IntegerType) -> Self {
        value.0
    }
}
impl From<&IntegerType> for IntegerType {
    fn from(value: &IntegerType) -> Self {
        value.clone()
    }
}
impl From<i64> for IntegerType {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for IntegerType {
    type Err = <i64 as std::str::FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl std::convert::TryFrom<&str> for IntegerType {
    type Error = <i64 as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IntegerType {
    type Error = <i64 as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IntegerType {
    type Error = <i64 as std::str::FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl ToString for IntegerType {
    fn to_string(&self) -> String {
        self.0.to_string()
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
impl From<IriType> for String {
    fn from(value: IriType) -> Self {
        value.0
    }
}
impl From<&IriType> for IriType {
    fn from(value: &IriType) -> Self {
        value.clone()
    }
}
impl From<String> for IriType {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for IriType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for IriType {
    fn to_string(&self) -> String {
        self.0.to_string()
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
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "type": {
///      "const": "IssuerNode"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IssuerNodeType {
    #[serde(rename = "eidasLegalIdentifier")]
    pub eidas_legal_identifier: LegalIdentifier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
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
impl From<LangStringType> for serde_json::Map<String, serde_json::Value> {
    fn from(value: LangStringType) -> Self {
        value.0
    }
}
impl From<&LangStringType> for LangStringType {
    fn from(value: &LangStringType) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for LangStringType {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
///LearningAchievementSpecificationOrQualificationType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/LearningAchievementSpecificationType"
///    },
///    {
///      "$ref": "#/$defs/QualificationType"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LearningAchievementSpecificationOrQualificationType {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<LearningAchievementSpecification>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<Qualification>,
}

///LearningAchievementSpecificationOrSpecificationType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/LearningAchievementSpecificationType"
///    },
///    {
///      "$ref": "#/$defs/QualificationType"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LearningAchievementSpecificationOrSpecificationType {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<LearningAchievementSpecification>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<Qualification>,
}

///LearningAchievementSpecificationType
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
///    "generalisationOf": {
///      "$ref": "#/$defs/Many!LearningAchievementSpecificationOrQualificationType"
///    },
///    "hasPart": {
///      "$ref": "#/$defs/Many!LearningAchievementSpecificationOrQualificationType"
///    },
///    "homepage": {
///      "$ref": "#/$defs/Many!WebResourceType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "identifier": {
///      "$ref": "#/$defs/IdentifierOrLegalIdentifierType"
///    },
///    "influencedBy": {
///      "$ref": "#/$defs/Many!LearningActivitySpecificationType"
///    },
///    "isPartOf": {
///      "$ref": "#/$defs/Many!LearningAchievementSpecificationOrQualificationType"
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
///    "provenBy": {
///      "$ref": "#/$defs/Many!LearningAssessmentSpecificationType"
///    },
///    "specialisationOf": {
///      "$ref": "#/$defs/Many!LearningAchievementSpecificationOrQualificationType"
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
///      "const": "LearningAchievementSpecification"
///    },
///    "volumeOfLearning": {
///      "$ref": "#/$defs/DurationType"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LearningAchievementSpecification {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<ManyLangStringType>,
    #[serde(
        rename = "awardingOpportunity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub awarding_opportunity: Option<ObjectOrVector<AwardingOpportunity>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<ManyLangStringType>,
    #[serde(rename = "creditPoint", default, skip_serializing_if = "Option::is_none")]
    pub credit_point: Option<ObjectOrVector<CreditPoint>>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTimeType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(rename = "educationLevel", default, skip_serializing_if = "Option::is_none")]
    pub education_level: Option<ObjectOrVector<Concept>>,
    #[serde(
        rename = "educationSubject",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub education_subject: Option<ObjectOrVector<Concept>>,
    #[serde(rename = "entitlesTo", default, skip_serializing_if = "Option::is_none")]
    pub entitles_to: Option<ObjectOrVector<LearningEntitlementSpecification>>,
    #[serde(
        rename = "entryRequirement",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub entry_requirement: Option<Note>,
    #[serde(rename = "generalisationOf", default)]
    pub generalisation_of: 
        Option<Box<ManyLearningAchievementSpecificationOrQualificationType>>,
    #[serde(rename = "hasPart", default)]
    pub has_part: Option<Box<ManyLearningAchievementSpecificationOrQualificationType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifierType>,
    #[serde(rename = "influencedBy", default)]
    pub influenced_by: Option<Box<ObjectOrVector<LearningActivitySpecification>>>,
    #[serde(rename = "isPartOf", default)]
    pub is_part_of: Option<Box<ManyLearningAchievementSpecificationOrQualificationType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<ObjectOrVector<Concept>>,
    #[serde(
        rename = "learningOutcome",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub learning_outcome: Option<ObjectOrVector<LearningOutcome>>,
    #[serde(
        rename = "learningOutcomeSummary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub learning_outcome_summary: Option<Note>,
    #[serde(
        rename = "learningSetting",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub learning_setting: Option<Concept>,
    #[serde(
        rename = "maximumDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_duration: Option<DurationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ObjectOrVector<Concept>>,
    #[serde(rename = "provenBy", default)]
    pub proven_by: Box<Option<ObjectOrVector<LearningAssessmentSpecification>>>,
    #[serde(rename = "specialisationOf", default)]
    pub specialisation_of: Box<
        Option<ManyLearningAchievementSpecificationOrQualificationType>,
    >,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StringType>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    #[serde(rename = "targetGroup", default, skip_serializing_if = "Option::is_none")]
    pub target_group: Option<ObjectOrVector<Concept>>,
    #[serde(rename = "thematicArea", default, skip_serializing_if = "Option::is_none")]
    pub thematic_area: Option<ObjectOrVector<Concept>>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
    #[serde(
        rename = "volumeOfLearning",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_of_learning: Option<DurationType>,
}

///LearningAchievementType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "awardedBy",
///    "title"
///  ],
///  "properties": {
///    "additionalNote": {
///      "$ref": "#/$defs/Many!NoteType"
///    },
///    "awardedBy": {
///      "$ref": "#/$defs/AwardingProcessType"
///    },
///    "creditReceived": {
///      "$ref": "#/$defs/Many!CreditPointType"
///    },
///    "dcType": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "description": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "entitlesTo": {
///      "$ref": "#/$defs/Many!LearningEntitlementType"
///    },
///    "hasPart": {
///      "$ref": "#/$defs/Many!LearningAchievementType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "identifier": {
///      "$ref": "#/$defs/IdentifierOrLegalIdentifierType"
///    },
///    "influencedBy": {
///      "$ref": "#/$defs/Many!LearningActivityType"
///    },
///    "isPartOf": {
///      "$ref": "#/$defs/Many!LearningAchievementType"
///    },
///    "learningOpportunity": {
///      "$ref": "#/$defs/LearningOpportunityType"
///    },
///    "provenBy": {
///      "$ref": "#/$defs/Many!LearningAssessmentType"
///    },
///    "specifiedBy": {
///      "$ref": "#/$defs/LearningAchievementSpecificationOrQualificationType"
///    },
///    "supplementaryDocument": {
///      "$ref": "#/$defs/Many!WebResourceType"
///    },
///    "title": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "type": {
///      "const": "LearningAchievement"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LearningAchievement {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(rename = "awardedBy")]
    pub awarded_by: Box<AwardingProcess>,
    #[serde(rename = "creditReceived", default, skip_serializing_if = "Option::is_none")]
    pub credit_received: Option<ObjectOrVector<CreditPoint>>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(rename = "entitlesTo", default, skip_serializing_if = "Option::is_none")]
    pub entitles_to: Option<ObjectOrVector<LearningEntitlement>>,
    #[serde(rename = "hasPart", default)]
    pub has_part: Box<Option<ObjectOrVector<LearningAchievement>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifierType>,
    #[serde(rename = "influencedBy", default, skip_serializing_if = "Option::is_none")]
    pub influenced_by: Option<ObjectOrVector<LearningActivity>>,
    #[serde(rename = "isPartOf", default)]
    pub is_part_of: Option<Box<ObjectOrVector<LearningAchievement>>>,
    #[serde(
        rename = "learningOpportunity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub learning_opportunity: Option<LearningOpportunity>,
    #[serde(rename = "provenBy", default)]
    pub proven_by: Box<Option<ObjectOrVector<LearningAssessment>>>,
    #[serde(rename = "specifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub specified_by: Option<LearningAchievementSpecificationOrQualificationType>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///LearningActivitySpecificationType
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
///    "additionalNote": {
///      "$ref": "#/$defs/Many!NoteType"
///    },
///    "altLabel": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "category": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "contactHour": {
///      "$ref": "#/$defs/Many!StringType"
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
///    "generalisationOf": {
///      "$ref": "#/$defs/Many!LearningAchievementSpecificationType"
///    },
///    "hasPart": {
///      "$ref": "#/$defs/Many!LearningAchievementSpecificationType"
///    },
///    "homepage": {
///      "$ref": "#/$defs/Many!WebResourceType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "identifier": {
///      "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
///    },
///    "influences": {
///      "$ref": "#/$defs/Many!LearningAchievementSpecificationOrQualificationType"
///    },
///    "isPartOf": {
///      "$ref": "#/$defs/Many!LearningAchievementSpecificationType"
///    },
///    "language": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "mode": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "specialisationOf": {
///      "$ref": "#/$defs/Many!LearningAchievementSpecificationType"
///    },
///    "status": {
///      "$ref": "#/$defs/StringType"
///    },
///    "supplementaryDocument": {
///      "$ref": "#/$defs/Many!WebResourceType"
///    },
///    "title": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "type": {
///      "const": "LearningActivitySpecification"
///    },
///    "volumeOfLearning": {
///      "$ref": "#/$defs/DurationType"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LearningActivitySpecification {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<ManyLangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<ManyLangStringType>,
    #[serde(rename = "contactHour", default, skip_serializing_if = "Option::is_none")]
    pub contact_hour: Option<ObjectOrVector<StringType>>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTimeType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(
        rename = "generalisationOf",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub generalisation_of: Option<ObjectOrVector<LearningAchievementSpecification>>,
    #[serde(rename = "hasPart", default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<ObjectOrVector<LearningAchievementSpecification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(default)]
    pub influences: Box<Option<ManyLearningAchievementSpecificationOrQualificationType>>,
    #[serde(rename = "isPartOf", default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<ObjectOrVector<LearningAchievementSpecification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ObjectOrVector<Concept>>,
    #[serde(
        rename = "specialisationOf",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub specialisation_of: Option<ObjectOrVector<LearningAchievementSpecification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StringType>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
    #[serde(
        rename = "volumeOfLearning",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_of_learning: Option<DurationType>,
}

///LearningActivityType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "awardedBy",
///    "title"
///  ],
///  "properties": {
///    "additionalNote": {
///      "$ref": "#/$defs/Many!NoteType"
///    },
///    "awardedBy": {
///      "$ref": "#/$defs/AwardingProcessType"
///    },
///    "dcType": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "description": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "directedBy": {
///      "$ref": "#/$defs/Many!AgentOrPersonOrOrganisationType"
///    },
///    "hasPart": {
///      "$ref": "#/$defs/Many!LearningActivityType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "identifier": {
///      "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
///    },
///    "influences": {
///      "$ref": "#/$defs/Many!LearningAchievementType"
///    },
///    "isPartOf": {
///      "$ref": "#/$defs/Many!LearningActivityType"
///    },
///    "learningOpportunity": {
///      "$ref": "#/$defs/LearningOpportunityType"
///    },
///    "levelOfCompletion": {
///      "$ref": "#/$defs/PercentageIntegerType"
///    },
///    "location": {
///      "$ref": "#/$defs/Many!LocationType"
///    },
///    "specifiedBy": {
///      "$ref": "#/$defs/LearningActivitySpecificationType"
///    },
///    "supplementaryDocument": {
///      "$ref": "#/$defs/Many!WebResourceType"
///    },
///    "temporal": {
///      "$ref": "#/$defs/Many!PeriodOfTimeType"
///    },
///    "title": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "type": {
///      "const": "LearningActivity"
///    },
///    "workload": {
///      "$ref": "#/$defs/DurationType"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LearningActivity {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(rename = "awardedBy")]
    pub awarded_by: Box<AwardingProcess>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(rename = "directedBy", default, skip_serializing_if = "Option::is_none")]
    pub directed_by: Option<ManyAgentOrPersonOrOrganisationType>,
    #[serde(rename = "hasPart", default)]
    pub has_part: Box<Option<ObjectOrVector<LearningActivity>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(default)]
    pub influences: Box<Option<ObjectOrVector<LearningAchievement>>>,
    #[serde(rename = "isPartOf", default)]
    pub is_part_of: Box<Option<ObjectOrVector<LearningActivity>>>,
    #[serde(
        rename = "learningOpportunity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub learning_opportunity: Option<LearningOpportunity>,
    #[serde(
        rename = "levelOfCompletion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub level_of_completion: Option<PercentageInteger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<ObjectOrVector<Location>>,
    #[serde(rename = "specifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub specified_by: Option<LearningActivitySpecification>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporal: Option<ObjectOrVector<PeriodOfTime>>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workload: Option<DurationType>,
}

///LearningAssessmentSpecificationType
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
///    "additionalNote": {
///      "$ref": "#/$defs/Many!NoteType"
///    },
///    "altLabel": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "category": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "dateModified": {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    "dcType": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "description": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "generalisationOf": {
///      "$ref": "#/$defs/Many!LearningAssessmentSpecificationType"
///    },
///    "gradingScheme": {
///      "$ref": "#/$defs/GradingSchemeType"
///    },
///    "hasPart": {
///      "$ref": "#/$defs/Many!LearningAssessmentSpecificationType"
///    },
///    "homepage": {
///      "$ref": "#/$defs/Many!WebResourceType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "identifier": {
///      "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
///    },
///    "isPartOf": {
///      "$ref": "#/$defs/Many!LearningAssessmentSpecificationType"
///    },
///    "language": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "mode": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "proves": {
///      "$ref": "#/$defs/Many!LearningAchievementSpecificationOrQualificationType"
///    },
///    "specialisationOf": {
///      "$ref": "#/$defs/Many!LearningAssessmentSpecificationType"
///    },
///    "status": {
///      "$ref": "#/$defs/StringType"
///    },
///    "supplementaryDocument": {
///      "$ref": "#/$defs/Many!WebResourceType"
///    },
///    "title": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "type": {
///      "const": "LearningAssessmentSpecification"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LearningAssessmentSpecification {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<ManyLangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<ManyLangStringType>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTimeType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(rename = "generalisationOf", default)]
    pub generalisation_of: Box<Option<ObjectOrVector<LearningAssessmentSpecification>>>,
    #[serde(rename = "gradingScheme", default, skip_serializing_if = "Option::is_none")]
    pub grading_scheme: Option<GradingSchemeType>,
    #[serde(rename = "hasPart", default)]
    pub has_part: Box<Option<ObjectOrVector<LearningAssessmentSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(rename = "isPartOf", default)]
    pub is_part_of: Box<Option<ObjectOrVector<LearningAssessmentSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proves: Option<ManyLearningAchievementSpecificationOrQualificationType>,
    #[serde(rename = "specialisationOf", default)]
    pub specialisation_of: Box<Option<ObjectOrVector<LearningAssessmentSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StringType>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///LearningAssessmentType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "awardedBy",
///    "grade",
///    "title"
///  ],
///  "properties": {
///    "additionalNote": {
///      "$ref": "#/$defs/Many!NoteType"
///    },
///    "assessedBy": {
///      "$ref": "#/$defs/Many!AgentOrPersonOrOrganisationType"
///    },
///    "awardedBy": {
///      "$ref": "#/$defs/AwardingProcessType"
///    },
///    "dateIssued": {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    "dcType": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "description": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "grade": {
///      "$ref": "#/$defs/NoteType"
///    },
///    "gradeStatus": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "hasPart": {
///      "$ref": "#/$defs/Many!LearningAssessmentType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "idVerification": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "identifier": {
///      "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
///    },
///    "isPartOf": {
///      "$ref": "#/$defs/Many!LearningAssessmentType"
///    },
///    "location": {
///      "$ref": "#/$defs/LocationType"
///    },
///    "proves": {
///      "$ref": "#/$defs/Many!LearningAchievementType"
///    },
///    "resultDistribution": {
///      "$ref": "#/$defs/ResultDistributionType"
///    },
///    "shortenedGrading": {
///      "$ref": "#/$defs/ShortenedGradingType"
///    },
///    "specifiedBy": {
///      "$ref": "#/$defs/Many!LearningAssessmentSpecificationType"
///    },
///    "supplementaryDocument": {
///      "$ref": "#/$defs/Many!WebResourceType"
///    },
///    "title": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "type": {
///      "const": "LearningAssessment"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LearningAssessment {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(rename = "assessedBy", default, skip_serializing_if = "Option::is_none")]
    pub assessed_by: Option<ManyAgentOrPersonOrOrganisationType>,
    #[serde(rename = "awardedBy")]
    pub awarded_by: Box<AwardingProcess>,
    #[serde(rename = "dateIssued", default, skip_serializing_if = "Option::is_none")]
    pub date_issued: Option<DateTimeType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    pub grade: Note,
    #[serde(rename = "gradeStatus", default, skip_serializing_if = "Option::is_none")]
    pub grade_status: Option<Concept>,
    #[serde(rename = "hasPart", default)]
    pub has_part: Option<Box<ObjectOrVector<LearningAssessment>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "idVerification", default, skip_serializing_if = "Option::is_none")]
    pub id_verification: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(rename = "isPartOf", default)]
    pub is_part_of: Option<Box<ObjectOrVector<LearningAssessment>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proves: Option<ObjectOrVector<LearningAchievement>>,
    #[serde(
        rename = "resultDistribution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_distribution: Option<ResultDistribution>,
    #[serde(
        rename = "shortenedGrading",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shortened_grading: Option<ShortenedGrading>,
    #[serde(rename = "specifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub specified_by: Option<ObjectOrVector<LearningAssessmentSpecification>>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///LearningEntitlementSpecificationType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "dcType",
///    "entitlementStatus",
///    "title"
///  ],
///  "properties": {
///    "additionalNote": {
///      "$ref": "#/$defs/Many!NoteType"
///    },
///    "altLabel": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "category": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "dateModified": {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    "dcType": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "description": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "entitledBy": {
///      "$ref": "#/$defs/Many!LearningAchievementSpecificationOrQualificationType"
///    },
///    "entitlementStatus": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "generalisationOf": {
///      "$ref": "#/$defs/Many!LearningEntitlementSpecificationType"
///    },
///    "hasPart": {
///      "$ref": "#/$defs/Many!LearningEntitlementSpecificationType"
///    },
///    "homepage": {
///      "$ref": "#/$defs/Many!WebResourceType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "identifier": {
///      "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
///    },
///    "isPartOf": {
///      "$ref": "#/$defs/Many!LearningEntitlementSpecificationType"
///    },
///    "limitJurisdiction": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "limitNationalOccupation": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "limitOccupation": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "limitOrganisation": {
///      "$ref": "#/$defs/Many!OrganisationType"
///    },
///    "specialisationOf": {
///      "$ref": "#/$defs/Many!LearningEntitlementSpecificationType"
///    },
///    "status": {
///      "$ref": "#/$defs/StringType"
///    },
///    "supplementaryDocument": {
///      "$ref": "#/$defs/Many!WebResourceType"
///    },
///    "title": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "type": {
///      "const": "LearningEntitlementSpecification"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LearningEntitlementSpecification {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<ManyLangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<ManyLangStringType>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTimeType>,
    #[serde(rename = "dcType")]
    pub dc_type: Concept,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(rename = "entitledBy", default)]
    pub entitled_by: 
        Option<Box<ManyLearningAchievementSpecificationOrQualificationType>>,
    #[serde(rename = "entitlementStatus")]
    pub entitlement_status: Concept,
    #[serde(rename = "generalisationOf", default)]
    pub generalisation_of: Option<Box<ObjectOrVector<LearningEntitlementSpecification>>>,
    #[serde(rename = "hasPart", default)]
    pub has_part: Option<Box<ObjectOrVector<LearningEntitlementSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(rename = "isPartOf", default)]
    pub is_part_of: Box<Option<ObjectOrVector<LearningEntitlementSpecification>>>,
    #[serde(
        rename = "limitJurisdiction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_jurisdiction: Option<ObjectOrVector<Concept>>,
    #[serde(
        rename = "limitNationalOccupation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_national_occupation: Option<ObjectOrVector<Concept>>,
    #[serde(
        rename = "limitOccupation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_occupation: Option<ObjectOrVector<Concept>>,
    #[serde(rename = "limitOrganisation", default)]
    pub limit_organisation: Option<Box<ObjectOrVector<Organisation>>>,
    #[serde(rename = "specialisationOf", default)]
    pub specialisation_of: Option<Box<ObjectOrVector<LearningEntitlementSpecification>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StringType>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///LearningEntitlementType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "awardedBy",
///    "title"
///  ],
///  "properties": {
///    "additionalNote": {
///      "$ref": "#/$defs/Many!NoteType"
///    },
///    "awardedBy": {
///      "$ref": "#/$defs/AwardingProcessType"
///    },
///    "dateIssued": {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    "dcType": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "description": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "entitledBy": {
///      "$ref": "#/$defs/Many!LearningAchievementType"
///    },
///    "expiryDate": {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    "hasPart": {
///      "$ref": "#/$defs/Many!LearningEntitlementType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "identifier": {
///      "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
///    },
///    "isPartOf": {
///      "$ref": "#/$defs/Many!LearningEntitlementType"
///    },
///    "specifiedBy": {
///      "$ref": "#/$defs/Many!LearningEntitlementSpecificationType"
///    },
///    "supplementaryDocument": {
///      "$ref": "#/$defs/Many!WebResourceType"
///    },
///    "title": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "type": {
///      "const": "LearningEntitlement"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LearningEntitlement {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(rename = "awardedBy")]
    pub awarded_by: Box<AwardingProcess>,
    #[serde(rename = "dateIssued", default, skip_serializing_if = "Option::is_none")]
    pub date_issued: Option<DateTimeType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(rename = "entitledBy", default)]
    pub entitled_by: Box<Option<ObjectOrVector<LearningAchievement>>>,
    #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<DateTimeType>,
    #[serde(rename = "hasPart", default)]
    pub has_part: Box<Option<ObjectOrVector<LearningEntitlement>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(rename = "isPartOf", default)]
    pub is_part_of: Box<Option<ObjectOrVector<LearningEntitlement>>>,
    #[serde(rename = "specifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub specified_by: Option<ObjectOrVector<LearningEntitlementSpecification>>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///LearningOpportunityType
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
///    "additionalNote": {
///      "$ref": "#/$defs/Many!NoteType"
///    },
///    "admissionProcedure": {
///      "$ref": "#/$defs/NoteType"
///    },
///    "applicationDeadline": {
///      "$ref": "#/$defs/Many!DateTimeType"
///    },
///    "bannerImage": {
///      "$ref": "#/$defs/MediaObjectType"
///    },
///    "dateModified": {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    "dcType": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "defaultLanguage": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "description": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "descriptionHtml": {
///      "$ref": "#/$defs/Many!HTMLType"
///    },
///    "duration": {
///      "$ref": "#/$defs/DurationType"
///    },
///    "grant": {
///      "$ref": "#/$defs/Many!GrantType"
///    },
///    "hasPart": {
///      "$ref": "#/$defs/Many!LearningOpportunityType"
///    },
///    "homepage": {
///      "$ref": "#/$defs/Many!WebResourceType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "identifier": {
///      "$ref": "#/$defs/Many!IdentifierOrLegalIdentifierType"
///    },
///    "isPartOf": {
///      "$ref": "#/$defs/Many!LearningOpportunityType"
///    },
///    "learningAchievementSpecification": {
///      "$ref": "#/$defs/LearningAchievementSpecificationOrQualificationType"
///    },
///    "learningActivitySpecification": {
///      "$ref": "#/$defs/LearningActivitySpecificationType"
///    },
///    "learningSchedule": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "location": {
///      "$ref": "#/$defs/Many!LocationType"
///    },
///    "mode": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "priceDetail": {
///      "$ref": "#/$defs/Many!PriceDetailType"
///    },
///    "providedBy": {
///      "$ref": "#/$defs/OrganisationType"
///    },
///    "scheduleInformation": {
///      "$ref": "#/$defs/NoteType"
///    },
///    "status": {
///      "$ref": "#/$defs/StringType"
///    },
///    "supplementaryDocument": {
///      "$ref": "#/$defs/Many!WebResourceType"
///    },
///    "temporal": {
///      "$ref": "#/$defs/PeriodOfTimeType"
///    },
///    "title": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "type": {
///      "const": "LearningOpportunity"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LearningOpportunity {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(
        rename = "admissionProcedure",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub admission_procedure: Option<Note>,
    #[serde(
        rename = "applicationDeadline",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub application_deadline: Option<ObjectOrVector<DateTimeType>>,
    #[serde(rename = "bannerImage", default, skip_serializing_if = "Option::is_none")]
    pub banner_image: Option<MediaObject>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTimeType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<Concept>>,
    #[serde(
        rename = "defaultLanguage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_language: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(
        rename = "descriptionHtml",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description_html: Option<ObjectOrVector<HtmlType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<DurationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant: Option<ObjectOrVector<Grant>>,
    #[serde(rename = "hasPart", default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<BoxObjectOrVector<LearningOpportunity>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(rename = "isPartOf", default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<BoxObjectOrVector<LearningOpportunity>>,
    #[serde(
        rename = "learningAchievementSpecification",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub learning_achievement_specification: Option<
        LearningAchievementSpecificationOrQualificationType,
    >,
    #[serde(
        rename = "learningActivitySpecification",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub learning_activity_specification: Option<LearningActivitySpecification>,
    #[serde(
        rename = "learningSchedule",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub learning_schedule: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<ObjectOrVector<Location>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ObjectOrVector<Concept>>,
    #[serde(rename = "priceDetail", default, skip_serializing_if = "Option::is_none")]
    pub price_detail: Option<ObjectOrVector<PriceDetail>>,
    #[serde(rename = "providedBy", default, skip_serializing_if = "Option::is_none")]
    pub provided_by: Option<Box<Organisation>>,
    #[serde(
        rename = "scheduleInformation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub schedule_information: Option<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StringType>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporal: Option<PeriodOfTime>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///LearningOutcomeType
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
///    "additionalNote": {
///      "$ref": "#/$defs/Many!NoteType"
///    },
///    "dcType": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "identifier": {
///      "$ref": "#/$defs/IdentifierOrLegalIdentifierType"
///    },
///    "relatedESCOSkill": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "relatedSkill": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "reusabilityLevel": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "title": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "type": {
///      "const": "LearningOutcome"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LearningOutcome {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifierType>,
    #[serde(
        rename = "relatedESCOSkill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub related_esco_skill: Option<ObjectOrVector<Concept>>,
    #[serde(rename = "relatedSkill", default, skip_serializing_if = "Option::is_none")]
    pub related_skill: Option<ObjectOrVector<Concept>>,
    #[serde(
        rename = "reusabilityLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reusability_level: Option<Concept>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///LegalIdentifierType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "notation",
///    "spatial"
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
///      "$ref": "#/$defs/GenericIdType"
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
///    "spatial": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "type": {
///      "const": "LegalIdentifier"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegalIdentifier {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<IriType>,
    #[serde(rename = "dateIssued", default, skip_serializing_if = "Option::is_none")]
    pub date_issued: Option<DateTimeType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    pub notation: Literal,
    #[serde(rename = "schemeAgency", default, skip_serializing_if = "Option::is_none")]
    pub scheme_agency: Option<LangStringType>,
    #[serde(rename = "schemeId", default, skip_serializing_if = "Option::is_none")]
    pub scheme_id: Option<UriType>,
    #[serde(rename = "schemeName", default, skip_serializing_if = "Option::is_none")]
    pub scheme_name: Option<StringType>,
    #[serde(rename = "schemeVersion", default, skip_serializing_if = "Option::is_none")]
    pub scheme_version: Option<StringType>,
    pub spatial: Concept,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///LiteralType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$ref": "#/$defs/StringType"
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Literal(pub StringType);
impl std::ops::Deref for Literal {
    type Target = StringType;
    fn deref(&self) -> &StringType {
        &self.0
    }
}

impl From<Literal> for StringType {
    fn from(value: Literal) -> Self {
        value.0
    }
}

impl From<&Literal> for Literal {
    fn from(value: &Literal) -> Self {
        value.clone()
    }
}

impl From<StringType> for Literal {
    fn from(value: StringType) -> Self {
        Self(value)
    }
}

impl std::str::FromStr for Literal {
    type Err = <StringType as std::str::FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}

impl std::convert::TryFrom<&str> for Literal {
    type Error = <StringType as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for Literal {
    type Error = <StringType as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for Literal {
    type Error = <StringType as std::str::FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

///LocationType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "address"
///  ],
///  "properties": {
///    "address": {
///      "$ref": "#/$defs/Many!AddressType"
///    },
///    "description": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "geographicName": {
///      "$ref": "#/$defs/Many!AddressType"
///    },
///    "geometry": {
///      "$ref": "#/$defs/Many!GeometryType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "identifier": {
///      "$ref": "#/$defs/IdentifierOrLegalIdentifierType"
///    },
///    "spatialCode": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "type": {
///      "const": "Location"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Location {
    pub address: Option<ObjectOrVector<Address>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(rename = "geographicName", default, skip_serializing_if = "Option::is_none")]
    pub geographic_name: Option<ObjectOrVector<Address>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geometry: Option<ObjectOrVector<Geometry>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifierType>,
    #[serde(rename = "spatialCode", default, skip_serializing_if = "Option::is_none")]
    pub spatial_code: Option<ObjectOrVector<Concept>>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///MailboxType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "id": {
///      "$ref": "#/$defs/EmailType"
///    },
///    "type": {
///      "const": "Mailbox"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Mailbox {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<EmailType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///ManyAgentOrPersonOrOrganisationType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/AgentOrPersonOrOrganisationType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/AgentOrPersonOrOrganisationType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ManyAgentOrPersonOrOrganisationType {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<Box<AgentOrPersonOrOrganisationType>>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<Vec<AgentOrPersonOrOrganisationType>>,
}

///ManyClaimNodeType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/ClaimNodeType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ClaimNodeType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ManyClaimNodeType {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub claim_node: Option<ClaimNode>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub claim_node_vecs: Option<Vec<ClaimNode>>,
}

///ManyIdentifierOrLegalIdentifierType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/IdentifierOrLegalIdentifierType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/IdentifierOrLegalIdentifierType"
///      }
///    }
///  ]
///}
/// ```
/// </details>

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ManyIdentifierOrLegalIdentifierType {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub identifier_or_legal_identifier: Option<IdentifierOrLegalIdentifierType>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub identifier_or_legal_identifier_vecs: Option<Vec<IdentifierOrLegalIdentifierType>>,
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ManyLangStringType(
    pub std::collections::HashMap<ManyLangStringKey, serde_json::Value>,
);
impl std::ops::Deref for ManyLangStringType {
    type Target = std::collections::HashMap<ManyLangStringKey, serde_json::Value>;
    fn deref(
        &self,
    ) -> &std::collections::HashMap<ManyLangStringKey, serde_json::Value> {
        &self.0
    }
}
impl From<ManyLangStringType>
for std::collections::HashMap<ManyLangStringKey, serde_json::Value> {
    fn from(value: ManyLangStringType) -> Self {
        value.0
    }
}
impl From<&ManyLangStringType> for ManyLangStringType {
    fn from(value: &ManyLangStringType) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<ManyLangStringKey, serde_json::Value>>
for ManyLangStringType {
    fn from(
        value: std::collections::HashMap<ManyLangStringKey, serde_json::Value>,
    ) -> Self {
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
pub struct ManyLangStringKey(String);
impl std::ops::Deref for ManyLangStringKey {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ManyLangStringKey> for String {
    fn from(value: ManyLangStringKey) -> Self {
        value.0
    }
}
impl From<&ManyLangStringKey> for ManyLangStringKey {
    fn from(value: &ManyLangStringKey) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for ManyLangStringKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new(
                "^(aa|ab|ae|af|ak|am|an|ar|as|av|ay|az|ba|be|bg|bh|bi|bm|bn|bo|br|bs|ca|ce|ch|co|cr|cs|cu|cv|cy|da|de|dv|dz|ee|el|en|eo|es|et|eu|fa|ff|fi|fj|fo|fr|fy|ga|gd|gl|gn|gu|gv|ha|he|hi|ho|hr|ht|hu|hy|hz|ia|id|ie|ig|ii|ik|in|io|is|it|iu|iw|ja|ji|jv|jw|ka|kg|ki|kj|kk|kl|km|kn|ko|kr|ks|ku|kv|kw|ky|la|lb|lg|li|ln|lo|lt|lu|lv|mg|mh|mi|mk|ml|mn|mo|mr|ms|mt|my|na|nb|nd|ne|ng|nl|nn|no|nr|nv|ny|oc|oj|om|or|os|pa|pi|pl|ps|pt|qu|rm|rn|ro|ru|rw|sa|sc|sd|se|sg|sh|si|sk|sl|sm|sn|so|sq|sr|ss|st|su|sv|sw|ta|te|tg|th|ti|tk|tl|tn|to|tr|ts|tt|tw|ty|ug|uk|ur|uz|ve|vi|vo|wa|wo|xh|yi|yo|za|zh|zu)$",
            )
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err(
                "doesn't match pattern \"^(aa|ab|ae|af|ak|am|an|ar|as|av|ay|az|ba|be|bg|bh|bi|bm|bn|bo|br|bs|ca|ce|ch|co|cr|cs|cu|cv|cy|da|de|dv|dz|ee|el|en|eo|es|et|eu|fa|ff|fi|fj|fo|fr|fy|ga|gd|gl|gn|gu|gv|ha|he|hi|ho|hr|ht|hu|hy|hz|ia|id|ie|ig|ii|ik|in|io|is|it|iu|iw|ja|ji|jv|jw|ka|kg|ki|kj|kk|kl|km|kn|ko|kr|ks|ku|kv|kw|ky|la|lb|lg|li|ln|lo|lt|lu|lv|mg|mh|mi|mk|ml|mn|mo|mr|ms|mt|my|na|nb|nd|ne|ng|nl|nn|no|nr|nv|ny|oc|oj|om|or|os|pa|pi|pl|ps|pt|qu|rm|rn|ro|ru|rw|sa|sc|sd|se|sg|sh|si|sk|sl|sm|sn|so|sq|sr|ss|st|su|sv|sw|ta|te|tg|th|ti|tk|tl|tn|to|tr|ts|tt|tw|ty|ug|uk|ur|uz|ve|vi|vo|wa|wo|xh|yi|yo|za|zh|zu)$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for ManyLangStringKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ManyLangStringKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ManyLangStringKey {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for ManyLangStringKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
///ManyLearningAchievementSpecificationOrQualificationType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/LearningAchievementSpecificationOrQualificationType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/LearningAchievementSpecificationOrQualificationType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ManyLearningAchievementSpecificationOrQualificationType {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub learn_achievement_or_qualification: Option<LearningAchievementSpecificationOrQualificationType>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub learning_achievement_or_qualification_vecs: Option<Vec<LearningAchievementSpecificationOrQualificationType>>,
}

///MediaObjectType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "content",
///    "contentEncoding",
///    "contentType"
///  ],
///  "properties": {
///    "attachmentType": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "content": {
///      "$ref": "#/$defs/StringType"
///    },
///    "contentEncoding": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "contentSize": {
///      "$ref": "#/$defs/IntegerType"
///    },
///    "contentType": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "contentURL": {
///      "$ref": "#/$defs/URIType"
///    },
///    "description": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "title": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "type": {
///      "const": "MediaObject"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MediaObject {
    #[serde(rename = "attachmentType", default, skip_serializing_if = "Option::is_none")]
    pub attachment_type: Option<Concept>,
    pub content: StringType,
    #[serde(rename = "contentEncoding")]
    pub content_encoding: Concept,
    #[serde(rename = "contentSize", default, skip_serializing_if = "Option::is_none")]
    pub content_size: Option<IntegerType>,
    #[serde(rename = "contentType")]
    pub content_type: Concept,
    #[serde(rename = "contentURL", default, skip_serializing_if = "Option::is_none")]
    pub content_url: Option<UriType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<ManyLangStringType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///NoteType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "noteLiteral"
///  ],
///  "properties": {
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "noteFormat": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "noteLiteral": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "subject": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "type": {
///      "const": "Note"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Note {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "noteFormat", default, skip_serializing_if = "Option::is_none")]
    pub note_format: Option<Concept>,
    #[serde(rename = "noteLiteral")]
    pub note_literal: ManyLangStringType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<Concept>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///OrganisationType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "legalName",
///    "location"
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
///    "contactPoint": {
///      "$ref": "#/$defs/Many!ContactPointType"
///    },
///    "dateModified": {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    "dcType": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "eIDASIdentifier": {
///      "$ref": "#/$defs/LegalIdentifierType"
///    },
///    "groupMemberOf": {
///      "$ref": "#/$defs/Many!GroupType"
///    },
///    "hasMember": {
///      "$ref": "#/$defs/Many!PersonType"
///    },
///    "hasSubOrganization": {
///      "$ref": "#/$defs/Many!OrganisationType"
///    },
///    "homepage": {
///      "$ref": "#/$defs/Many!WebResourceType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "identifier": {
///      "$ref": "#/$defs/IdentifierOrLegalIdentifierType"
///    },
///    "legalName": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "location": {
///      "$ref": "#/$defs/Many!LocationType"
///    },
///    "logo": {
///      "$ref": "#/$defs/MediaObjectType"
///    },
///    "registration": {
///      "$ref": "#/$defs/LegalIdentifierType"
///    },
///    "subOrganizationOf": {
///      "$ref": "#/$defs/OrganisationType"
///    },
///    "taxIdentifier": {
///      "$ref": "#/$defs/Many!LegalIdentifierType"
///    },
///    "type": {
///      "const": "Organisation"
///    },
///    "vatIdentifier": {
///      "$ref": "#/$defs/Many!LegalIdentifierType"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Organisation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accreditation: Option<BoxObjectOrVector<Accreditation>>,
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<ManyLangStringType>,
    #[serde(rename = "contactPoint", default, skip_serializing_if = "Option::is_none")]
    pub contact_point: Option<ObjectOrVector<ContactPoint>>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTimeType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<Concept>>,
    #[serde(
        rename = "eIDASIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub e_idas_identifier: Option<LegalIdentifier>,
    #[serde(rename = "groupMemberOf", default, skip_serializing_if = "Option::is_none")]
    pub group_member_of: Option<ObjectOrVector<Group>>,
    #[serde(rename = "hasMember", default, skip_serializing_if = "Option::is_none")]
    pub has_member: Option<ObjectOrVector<Person>>,
    #[serde(rename = "hasSubOrganization", default)]
    pub has_sub_organization: Box<Option<ObjectOrVector<Organisation>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifierType>,
    #[serde(rename = "legalName")]
    pub legal_name: ManyLangStringType,
    pub location: ObjectOrVector<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo: Option<MediaObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration: Option<LegalIdentifier>,
    #[serde(
        rename = "subOrganizationOf",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sub_organization_of: Option<Box<Organisation>>,
    #[serde(rename = "taxIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub tax_identifier: Option<ObjectOrVector<LegalIdentifier>>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
    #[serde(rename = "vatIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub vat_identifier: Option<ObjectOrVector<LegalIdentifier>>,
}

///PercentageIntegerType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "maximum": 100.0,
///  "minimum": 0.0
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PercentageInteger(pub i64);
impl std::ops::Deref for PercentageInteger {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}

impl From<PercentageInteger> for i64 {
    fn from(value: PercentageInteger) -> Self {
        value.0
    }
}

impl From<&PercentageInteger> for PercentageInteger {
    fn from(value: &PercentageInteger) -> Self {
        value.clone()
    }
}

impl From<i64> for PercentageInteger {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl std::str::FromStr for PercentageInteger {
    type Err = <i64 as std::str::FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}

impl std::convert::TryFrom<&str> for PercentageInteger {
    type Error = <i64 as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for PercentageInteger {
    type Error = <i64 as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for PercentageInteger {
    type Error = <i64 as std::str::FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl ToString for PercentageInteger {
    fn to_string(&self) -> String {
        self.0.to_string()
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
///      "$ref": "#/$defs/GenericIdType"
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
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PeriodOfTime {
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateTimeType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "prefLabel", default, skip_serializing_if = "Option::is_none")]
    pub pref_label: Option<ManyLangStringType>,
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateTimeType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///PersonType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "birthName": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "citizenshipCountry": {
///      "$ref": "#/$defs/Many!ConceptType"
///    },
///    "contactPoint": {
///      "$ref": "#/$defs/Many!ContactPointType"
///    },
///    "dateModified": {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    "dateOfBirth": {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    "familyName": {
///      "$ref": "#/$defs/LangStringType"
///    },
///    "fullName": {
///      "$ref": "#/$defs/LangStringType"
///    },
///    "gender": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "givenName": {
///      "$ref": "#/$defs/LangStringType"
///    },
///    "groupMemberOf": {
///      "$ref": "#/$defs/Many!GroupType"
///    },
///    "hasClaim": {
///      "$ref": "#/$defs/Many!ClaimNodeType"
///    },
///    "hasCredential": {
///      "$ref": "#/$defs/Many!EuropeanDigitalCredentialType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "identifier": {
///      "$ref": "#/$defs/IdentifierOrLegalIdentifierType"
///    },
///    "location": {
///      "$ref": "#/$defs/LocationType"
///    },
///    "memberOf": {
///      "$ref": "#/$defs/Many!OrganisationType"
///    },
///    "nationalID": {
///      "$ref": "#/$defs/LegalIdentifierType"
///    },
///    "patronymicName": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "placeOfBirth": {
///      "$ref": "#/$defs/LocationType"
///    },
///    "type": {
///      "const": "Person"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Person {
    #[serde(rename = "birthName", default, skip_serializing_if = "Option::is_none")]
    pub birth_name: Option<ManyLangStringType>,
    #[serde(
        rename = "citizenshipCountry",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub citizenship_country: Option<ObjectOrVector<Concept>>,
    #[serde(rename = "contactPoint", default, skip_serializing_if = "Option::is_none")]
    pub contact_point: Option<ObjectOrVector<ContactPoint>>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTimeType>,
    #[serde(rename = "dateOfBirth", default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<DateTimeType>,
    #[serde(rename = "familyName", default, skip_serializing_if = "Option::is_none")]
    pub family_name: Option<LangStringType>,
    #[serde(rename = "fullName", default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<LangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<Concept>,
    #[serde(rename = "givenName", default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<LangStringType>,
    #[serde(rename = "groupMemberOf", default, skip_serializing_if = "Option::is_none")]
    pub group_member_of: Option<ObjectOrVector<Group>>,
    #[serde(rename = "hasClaim", default, skip_serializing_if = "Option::is_none")]
    pub has_claim: Option<ManyClaimNodeType>,
    #[serde(rename = "hasCredential", default, skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<ObjectOrVector<EuropeanDigitalCredential>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifierType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(rename = "memberOf", default)]
    pub member_of: Option<Box<ObjectOrVector<Organisation>>>,
    #[serde(rename = "nationalID", default, skip_serializing_if = "Option::is_none")]
    pub national_id: Option<LegalIdentifier>,
    #[serde(rename = "patronymicName", default, skip_serializing_if = "Option::is_none")]
    pub patronymic_name: Option<ManyLangStringType>,
    #[serde(rename = "placeOfBirth", default, skip_serializing_if = "Option::is_none")]
    pub place_of_birth: Option<Location>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

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
///      "$ref": "#/$defs/GenericIdType"
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
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Phone {
    #[serde(rename = "areaDialing", default, skip_serializing_if = "Option::is_none")]
    pub area_dialing: Option<StringType>,
    #[serde(rename = "countryDialing", default, skip_serializing_if = "Option::is_none")]
    pub country_dialing: Option<StringType>,
    #[serde(rename = "dialNumber", default, skip_serializing_if = "Option::is_none")]
    pub dial_number: Option<StringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "phoneNumber", default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<StringType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///PositiveIntegerType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "minimum": 0.0
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PositiveIntegerType(pub u64);
impl std::ops::Deref for PositiveIntegerType {
    type Target = u64;
    fn deref(&self) -> &u64 {
        &self.0
    }
}

impl From<PositiveIntegerType> for u64 {
    fn from(value: PositiveIntegerType) -> Self {
        value.0
    }
}

impl From<&PositiveIntegerType> for PositiveIntegerType {
    fn from(value: &PositiveIntegerType) -> Self {
        value.clone()
    }
}

impl From<u64> for PositiveIntegerType {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl std::str::FromStr for PositiveIntegerType {
    type Err = <u64 as std::str::FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}

impl std::convert::TryFrom<&str> for PositiveIntegerType {
    type Error = <u64 as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for PositiveIntegerType {
    type Error = <u64 as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for PositiveIntegerType {
    type Error = <u64 as std::str::FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl ToString for PositiveIntegerType {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
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
///      "$ref": "#/$defs/GenericIdType"
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
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PriceDetail {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<AmountType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(rename = "prefLabel", default, skip_serializing_if = "Option::is_none")]
    pub pref_label: Option<ManyLangStringType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
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
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "type": {
///      "const": "Proof"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Proof {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
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
///      "$ref": "#/$defs/GenericIdType"
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
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Qualification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accreditation: Option<BoxObjectOrVector<Accreditation>>,
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ObjectOrVector<Note>>,
    #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<ManyLangStringType>,
    #[serde(
        rename = "awardingOpportunity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub awarding_opportunity: Option<ObjectOrVector<AwardingOpportunity>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<ManyLangStringType>,
    #[serde(rename = "creditPoint", default, skip_serializing_if = "Option::is_none")]
    pub credit_point: Option<ObjectOrVector<CreditPoint>>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTimeType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<Concept>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(rename = "educationLevel", default, skip_serializing_if = "Option::is_none")]
    pub education_level: Option<ObjectOrVector<Concept>>,
    #[serde(
        rename = "educationSubject",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub education_subject: Option<ObjectOrVector<Concept>>,
    #[serde(rename = "entitlesTo", default, skip_serializing_if = "Option::is_none")]
    pub entitles_to: Option<ObjectOrVector<LearningEntitlementSpecification>>,
    #[serde(
        rename = "entryRequirement",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub entry_requirement: Option<Note>,
    #[serde(rename = "eqfLevel", default, skip_serializing_if = "Option::is_none")]
    pub eqf_level: Option<Concept>,
    #[serde(
        rename = "generalisationOf",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub generalisation_of: Option<BoxObjectOrVector<Qualification>>,
    #[serde(rename = "hasPart", default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<BoxObjectOrVector<Qualification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ObjectOrVector<WebResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifierType>,
    #[serde(rename = "influencedBy", default, skip_serializing_if = "Option::is_none")]
    pub influenced_by: Option<ObjectOrVector<LearningActivitySpecification>>,
    #[serde(rename = "isPartOf", default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<BoxObjectOrVector<Qualification>>,
    #[serde(
        rename = "isPartialQualification",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_partial_qualification: Option<BooleanType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<ObjectOrVector<Concept>>,
    #[serde(
        rename = "learningOutcome",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub learning_outcome: Option<ObjectOrVector<LearningOutcome>>,
    #[serde(
        rename = "learningOutcomeSummary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub learning_outcome_summary: Option<Note>,
    #[serde(
        rename = "learningSetting",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub learning_setting: Option<Concept>,
    #[serde(
        rename = "maximumDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_duration: Option<DurationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ObjectOrVector<Concept>>,
    #[serde(rename = "nqfLevel", default, skip_serializing_if = "Option::is_none")]
    pub nqf_level: Option<ObjectOrVector<Concept>>,
    #[serde(rename = "provenBy", default)]
    pub proven_by: Option<Box<ObjectOrVector<LearningAssessmentSpecification>>>,
    #[serde(
        rename = "qualificationCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub qualification_code: Option<ObjectOrVector<Concept>>,
    #[serde(
        rename = "specialisationOf",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub specialisation_of: Option<BoxObjectOrVector<Qualification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StringType>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ObjectOrVector<WebResource>>,
    #[serde(rename = "targetGroup", default, skip_serializing_if = "Option::is_none")]
    pub target_group: Option<ObjectOrVector<Concept>>,
    #[serde(rename = "thematicArea", default, skip_serializing_if = "Option::is_none")]
    pub thematic_area: Option<ObjectOrVector<Concept>>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
    #[serde(
        rename = "volumeOfLearning",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_of_learning: Option<DurationType>,
}

///ResultCategoryType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "count",
///    "label"
///  ],
///  "properties": {
///    "count": {
///      "$ref": "#/$defs/PositiveIntegerType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "label": {
///      "$ref": "#/$defs/StringType"
///    },
///    "maximumScore": {
///      "$ref": "#/$defs/StringType"
///    },
///    "minimumScore": {
///      "$ref": "#/$defs/StringType"
///    },
///    "score": {
///      "$ref": "#/$defs/StringType"
///    },
///    "type": {
///      "const": "ResultCategory"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResultCategory {
    pub count: PositiveIntegerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    pub label: StringType,
    #[serde(rename = "maximumScore", default, skip_serializing_if = "Option::is_none")]
    pub maximum_score: Option<StringType>,
    #[serde(rename = "minimumScore", default, skip_serializing_if = "Option::is_none")]
    pub minimum_score: Option<StringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<StringType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///ResultDistributionType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "description": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "resultCategory": {
///      "$ref": "#/$defs/Many!ResultCategoryType"
///    },
///    "type": {
///      "const": "ResultDistribution"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResultDistribution {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "resultCategory", default, skip_serializing_if = "Option::is_none")]
    pub result_category: Option<ObjectOrVector<ResultCategory>>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///ShaclValidator2017Type
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "type": {
///      "const": "ShaclValidator2017"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ShaclValidator2017 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///ShortenedGradingType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "percentageEqual",
///    "percentageHigher",
///    "percentageLower"
///  ],
///  "properties": {
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "percentageEqual": {
///      "$ref": "#/$defs/IntegerType"
///    },
///    "percentageHigher": {
///      "$ref": "#/$defs/IntegerType"
///    },
///    "percentageLower": {
///      "$ref": "#/$defs/IntegerType"
///    },
///    "type": {
///      "const": "ShortenedGrading"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ShortenedGrading {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "percentageEqual")]
    pub percentage_equal: IntegerType,
    #[serde(rename = "percentageHigher")]
    pub percentage_higher: IntegerType,
    #[serde(rename = "percentageLower")]
    pub percentage_lower: IntegerType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///StringType
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
pub struct StringType(pub String);
impl std::ops::Deref for StringType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl From<StringType> for String {
    fn from(value: StringType) -> Self {
        value.0
    }
}

impl From<&StringType> for StringType {
    fn from(value: &StringType) -> Self {
        value.clone()
    }
}

impl From<String> for StringType {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl std::str::FromStr for StringType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}

impl ToString for StringType {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

///Contains the terms under which the issued credential was issued
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Contains the terms under which the issued credential was issued",
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "id": {
///      "description": "Contains a URL that points to where more information about this instance of terms of use can be found.",
///      "type": "string",
///      "format": "uri"
///    },
///    "type": {
///      "description": "Defines the type extension",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TermsOfUse {
    ///Contains a URL that points to where more information about this instance of terms of use can be found.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Defines the type extension
    #[serde(rename = "type")]
    pub type_: String,
}

///TermsOfUseType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "type": {
///      "const": "TermsOfUse"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TermsOfUseType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}

///UriType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "format": "uri"
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct UriType(pub String);
impl std::ops::Deref for UriType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<UriType> for String {
    fn from(value: UriType) -> Self {
        value.0
    }
}
impl From<&UriType> for UriType {
    fn from(value: &UriType) -> Self {
        value.clone()
    }
}
impl From<String> for UriType {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for UriType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for UriType {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
///VerificationCheckType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "dcType",
///    "subject",
///    "verificationStatus"
///  ],
///  "properties": {
///    "dcType": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "description": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "elmSubject": {
///      "$ref": "#/$defs/EuropeanDigitalCredentialType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "type": {
///      "const": "VerificationCheck"
///    },
///    "verificationStatus": {
///      "$ref": "#/$defs/ConceptType"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerificationCheck {
    #[serde(rename = "dcType")]
    pub dc_type: Concept,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(rename = "elmSubject", default, skip_serializing_if = "Option::is_none")]
    pub elm_subject: Option<EuropeanDigitalCredential>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    pub subject: serde_json::Value,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
    #[serde(rename = "verificationStatus")]
    pub verification_status: Concept,
}

///WebResourceType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "contentURL"
///  ],
///  "properties": {
///    "contentURL": {
///      "$ref": "#/$defs/URIType"
///    },
///    "id": {
///      "$ref": "#/$defs/GenericIdType"
///    },
///    "language": {
///      "$ref": "#/$defs/ConceptType"
///    },
///    "title": {
///      "$ref": "#/$defs/Many!LangStringType"
///    },
///    "type": {
///      "const": "WebResource"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WebResource {
    #[serde(rename = "contentURL")]
    pub content_url: UriType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<Concept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<ManyLangStringType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
