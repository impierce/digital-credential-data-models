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
pub struct AccreditationType {
    #[serde(rename = "accreditingAgent")]
    pub accrediting_agent: OrganisationType,
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ManyNoteType>,
    #[serde(rename = "dateIssued", default, skip_serializing_if = "Option::is_none")]
    pub date_issued: Option<DateTimeType>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTimeType>,
    #[serde(rename = "dcType")]
    pub dc_type: ConceptType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision: Option<ConceptType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<DateTimeType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ManyWebResourceType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifierType>,
    #[serde(rename = "landingPage", default, skip_serializing_if = "Option::is_none")]
    pub landing_page: Option<ManyWebResourceType>,
    #[serde(
        rename = "limitCredentialType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_credential_type: Option<ObjectOrVector<ConceptType>>,
    #[serde(rename = "limitEQFLevel", default, skip_serializing_if = "Option::is_none")]
    pub limit_eqf_level: Option<ObjectOrVector<ConceptType>>,
    #[serde(rename = "limitField", default, skip_serializing_if = "Option::is_none")]
    pub limit_field: Option<ObjectOrVector<ConceptType>>,
    #[serde(
        rename = "limitJurisdiction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_jurisdiction: Option<ObjectOrVector<ConceptType>>,
    #[serde(
        rename = "limitQualification",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_qualification: Option<QualificationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organisation: Option<ManyOrganisationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report: Option<WebResourceType>,
    #[serde(rename = "reviewDate", default, skip_serializing_if = "Option::is_none")]
    pub review_date: Option<DateTimeType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StringType>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ManyWebResourceType>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&AccreditationType> for AccreditationType {
    fn from(value: &AccreditationType) -> Self {
        value.clone()
    }
}
impl AccreditationType {
    pub fn builder() -> builder::AccreditationType {
        Default::default()
    }
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
pub struct AddressType {
    #[serde(rename = "countryCode")]
    pub country_code: ConceptType,
    #[serde(rename = "fullAddress", default, skip_serializing_if = "Option::is_none")]
    pub full_address: Option<NoteType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifierType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&AddressType> for AddressType {
    fn from(value: &AddressType) -> Self {
        value.clone()
    }
}
impl AddressType {
    pub fn builder() -> builder::AddressType {
        Default::default()
    }
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
    pub subtype_0: Option<AgentType>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<Box<PersonType>>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_2: Option<Box<OrganisationType>>,
}
impl From<&AgentOrPersonOrOrganisationType> for AgentOrPersonOrOrganisationType {
    fn from(value: &AgentOrPersonOrOrganisationType) -> Self {
        value.clone()
    }
}
impl AgentOrPersonOrOrganisationType {
    pub fn builder() -> builder::AgentOrPersonOrOrganisationType {
        Default::default()
    }
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
    pub additional_note: Option<ManyNoteType>,
    #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<ManyLangStringType>,
    #[serde(rename = "contactPoint", default, skip_serializing_if = "Option::is_none")]
    pub contact_point: Option<ManyContactPointType>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTimeType>,
    #[serde(rename = "groupMemberOf", default, skip_serializing_if = "Option::is_none")]
    pub group_member_of: Option<ObjectOrVector<Group>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<ManyLocationType>,
    #[serde(rename = "prefLabel", default, skip_serializing_if = "Option::is_none")]
    pub pref_label: Option<ManyLangStringType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&AgentType> for AgentType {
    fn from(value: &AgentType) -> Self {
        value.clone()
    }
}
impl AgentType {
    pub fn builder() -> builder::AgentType {
        Default::default()
    }
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
    pub unit: ConceptType,
    pub value: DecimalType,
}
impl From<&AmountType> for AmountType {
    fn from(value: &AmountType) -> Self {
        value.clone()
    }
}
impl AmountType {
    pub fn builder() -> builder::AmountType {
        Default::default()
    }
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
pub struct AwardingOpportunityType {
    #[serde(rename = "awardingBody")]
    pub awarding_body: ManyAgentOrPersonOrOrganisationType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(rename = "learningAchievementSpecification", default)]
    pub learning_achievement_specification: Box<
        Option<LearningAchievementSpecificationOrQualificationType>,
    >,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporal: Option<PeriodOfTimeType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&AwardingOpportunityType> for AwardingOpportunityType {
    fn from(value: &AwardingOpportunityType) -> Self {
        value.clone()
    }
}
impl AwardingOpportunityType {
    pub fn builder() -> builder::AwardingOpportunityType {
        Default::default()
    }
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
pub struct AwardingProcessType {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ManyNoteType>,
    #[serde(rename = "awardingBody")]
    pub awarding_body: ManyAgentOrPersonOrOrganisationType,
    #[serde(rename = "awardingDate", default, skip_serializing_if = "Option::is_none")]
    pub awarding_date: Option<DateTimeType>,
    #[serde(default)]
    pub awards: Box<Option<ManyClaimNodeType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(
        rename = "educationalSystemNote",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub educational_system_note: Option<ConceptType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used: Option<ManyLearningAssessmentType>,
}
impl From<&AwardingProcessType> for AwardingProcessType {
    fn from(value: &AwardingProcessType) -> Self {
        value.clone()
    }
}
impl AwardingProcessType {
    pub fn builder() -> builder::AwardingProcessType {
        Default::default()
    }
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
impl From<BooleanType> for String {
    fn from(value: BooleanType) -> Self {
        value.0
    }
}
impl From<&BooleanType> for BooleanType {
    fn from(value: &BooleanType) -> Self {
        value.clone()
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
pub struct ClaimNodeType {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<LearningAchievementType>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<LearningActivityType>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_2: Option<LearningAssessmentType>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_3: Option<LearningEntitlementType>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_4: Option<ClaimTypeNodeType>,
}
impl From<&ClaimNodeType> for ClaimNodeType {
    fn from(value: &ClaimNodeType) -> Self {
        value.clone()
    }
}
impl ClaimNodeType {
    pub fn builder() -> builder::ClaimNodeType {
        Default::default()
    }
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
    pub additional_note: Option<ManyNoteType>,
    #[serde(rename = "awardedBy")]
    pub awarded_by: AwardingProcessType,
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
    pub supplementary_document: Option<ManyWebResourceType>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&ClaimTypeNodeType> for ClaimTypeNodeType {
    fn from(value: &ClaimTypeNodeType) -> Self {
        value.clone()
    }
}
impl ClaimTypeNodeType {
    pub fn builder() -> builder::ClaimTypeNodeType {
        Default::default()
    }
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
pub struct ConceptSchemeType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&ConceptSchemeType> for ConceptSchemeType {
    fn from(value: &ConceptSchemeType) -> Self {
        value.clone()
    }
}
impl ConceptSchemeType {
    pub fn builder() -> builder::ConceptSchemeType {
        Default::default()
    }
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
pub struct ConceptType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<ManyLangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "inScheme", default, skip_serializing_if = "Option::is_none")]
    pub in_scheme: Option<ConceptSchemeType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notation: Option<LiteralType>,
    #[serde(rename = "prefLabel", default, skip_serializing_if = "Option::is_none")]
    pub pref_label: Option<ManyLangStringType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&ConceptType> for ConceptType {
    fn from(value: &ConceptType) -> Self {
        value.clone()
    }
}
impl ConceptType {
    pub fn builder() -> builder::ConceptType {
        Default::default()
    }
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
pub struct ContactPointType {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ManyNoteType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<ManyAddressType>,
    #[serde(rename = "contactForm", default, skip_serializing_if = "Option::is_none")]
    pub contact_form: Option<ManyWebResourceType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<ManyMailboxType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<ManyPhoneType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&ContactPointType> for ContactPointType {
    fn from(value: &ContactPointType) -> Self {
        value.clone()
    }
}
impl ContactPointType {
    pub fn builder() -> builder::ContactPointType {
        Default::default()
    }
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
impl From<&CredentialSchema> for CredentialSchema {
    fn from(value: &CredentialSchema) -> Self {
        value.clone()
    }
}
impl CredentialSchema {
    pub fn builder() -> builder::CredentialSchema {
        Default::default()
    }
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
pub struct CredentialStatusType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&CredentialStatusType> for CredentialStatusType {
    fn from(value: &CredentialStatusType) -> Self {
        value.clone()
    }
}
impl CredentialStatusType {
    pub fn builder() -> builder::CredentialStatusType {
        Default::default()
    }
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
impl From<&CredentialSubject> for CredentialSubject {
    fn from(value: &CredentialSubject) -> Self {
        value.clone()
    }
}
impl CredentialSubject {
    pub fn builder() -> builder::CredentialSubject {
        Default::default()
    }
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
pub struct CreditPointType {
    pub framework: ConceptType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    pub point: StringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&CreditPointType> for CreditPointType {
    fn from(value: &CreditPointType) -> Self {
        value.clone()
    }
}
impl CreditPointType {
    pub fn builder() -> builder::CreditPointType {
        Default::default()
    }
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
pub struct DisplayDetailType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    pub image: MediaObjectType,
    pub page: PositiveIntegerType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&DisplayDetailType> for DisplayDetailType {
    fn from(value: &DisplayDetailType) -> Self {
        value.clone()
    }
}
impl DisplayDetailType {
    pub fn builder() -> builder::DisplayDetailType {
        Default::default()
    }
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
pub struct DisplayParameterType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "individualDisplay")]
    pub individual_display: ManyIndividualDisplayType,
    pub language: ObjectOrVector<ConceptType>,
    #[serde(rename = "primaryLanguage")]
    pub primary_language: ConceptType,
    #[serde(rename = "summaryDisplay", default, skip_serializing_if = "Option::is_none")]
    pub summary_display: Option<StringType>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&DisplayParameterType> for DisplayParameterType {
    fn from(value: &DisplayParameterType) -> Self {
        value.clone()
    }
}
impl DisplayParameterType {
    pub fn builder() -> builder::DisplayParameterType {
        Default::default()
    }
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
impl From<&EmailType> for EmailType {
    fn from(value: &EmailType) -> Self {
        value.clone()
    }
}
impl EmailType {
    pub fn builder() -> builder::EmailType {
        Default::default()
    }
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
    pub credential_profiles: Option<ObjectOrVector<ConceptType>>,
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
    pub display_parameter: Option<DisplayParameterType>,
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
impl From<&EuropassEdcCredential> for EuropassEdcCredential {
    fn from(value: &EuropassEdcCredential) -> Self {
        value.clone()
    }
}

impl EuropassEdcCredential {
    pub fn builder() -> builder::EuropassEdcCredential {
        Default::default()
    }
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
impl From<&EuropassEdcCredentialCredentialStatus>
for EuropassEdcCredentialCredentialStatus {
    fn from(value: &EuropassEdcCredentialCredentialStatus) -> Self {
        value.clone()
    }
}
impl EuropassEdcCredentialCredentialStatus {
    pub fn builder() -> builder::EuropassEdcCredentialCredentialStatus {
        Default::default()
    }
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
        additional_note: Option<ManyNoteType>,
        #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
        alt_label: Option<ManyLangStringType>,
        #[serde(
            rename = "contactPoint",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        contact_point: Option<ManyContactPointType>,
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
        group_member_of: Option<ManyGroupType>,
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
        citizenship_country: Option<ObjectOrVector<ConceptType>>,
        #[serde(
            rename = "contactPoint",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        contact_point: Option<ManyContactPointType>,
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
        gender: Option<ConceptType>,
        #[serde(rename = "givenName", default, skip_serializing_if = "Option::is_none")]
        given_name: Option<LangStringType>,
        #[serde(
            rename = "groupMemberOf",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        group_member_of: Option<ManyGroupType>,
        #[serde(rename = "hasClaim", default, skip_serializing_if = "Option::is_none")]
        has_claim: Option<ManyClaimNodeType>,
        #[serde(
            rename = "hasCredential",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        has_credential: Option<ManyEuropeanDigitalCredentialType>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        identifier: Option<IdentifierOrLegalIdentifierType>,
        #[serde(rename = "memberOf", default, skip_serializing_if = "Option::is_none")]
        member_of: Option<ManyOrganisationType>,
        #[serde(rename = "nationalID", default, skip_serializing_if = "Option::is_none")]
        national_id: Option<LegalIdentifierType>,
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
        place_of_birth: Option<LocationType>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        type_: Option<serde_json::Value>,
    },
    Variant2 {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        accreditation: Option<ManyAccreditationType>,
        #[serde(
            rename = "additionalNote",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        additional_note: Option<ManyNoteType>,
        #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
        alt_label: Option<ManyLangStringType>,
        #[serde(
            rename = "contactPoint",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        contact_point: Option<ManyContactPointType>,
        #[serde(
            rename = "dateModified",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        date_modified: Option<DateTimeType>,
        #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
        dc_type: Option<ObjectOrVector<ConceptType>>,
        #[serde(
            rename = "eIDASIdentifier",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        e_idas_identifier: Option<LegalIdentifierType>,
        #[serde(
            rename = "groupMemberOf",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        group_member_of: Option<ManyGroupType>,
        #[serde(rename = "hasMember", default, skip_serializing_if = "Option::is_none")]
        has_member: Option<ManyPersonType>,
        #[serde(
            rename = "hasSubOrganization",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        has_sub_organization: Option<ManyOrganisationType>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        homepage: Option<ManyWebResourceType>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        identifier: Option<IdentifierOrLegalIdentifierType>,
        #[serde(rename = "legalName")]
        legal_name: ManyLangStringType,
        location: ManyLocationType,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        logo: Option<MediaObjectType>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        registration: Option<LegalIdentifierType>,
        #[serde(
            rename = "subOrganizationOf",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        sub_organization_of: Option<Box<OrganisationType>>,
        #[serde(
            rename = "taxIdentifier",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        tax_identifier: Option<ManyLegalIdentifierType>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        type_: Option<serde_json::Value>,
        #[serde(
            rename = "vatIdentifier",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        vat_identifier: Option<ManyLegalIdentifierType>,
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
pub struct EuropeanDigitalCredentialType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment: Option<ManyMediaObjectType>,
    #[serde(rename = "credentialProfiles")]
    pub credential_profiles: ObjectOrVector<ConceptType>,
    #[serde(rename = "credentialSchema")]
    pub credential_schema: ManyShaclValidator2017Type,
    #[serde(
        rename = "credentialStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub credential_status: Option<CredentialStatusType>,
    #[serde(rename = "credentialSubject")]
    pub credential_subject: AgentOrPersonOrOrganisationType,
    #[serde(rename = "displayParameter")]
    pub display_parameter: DisplayParameterType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence: Option<ManyEvidenceType>,
    #[serde(rename = "expirationDate")]
    pub expiration_date: ManyDateTimeType,
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
    pub proof: Option<ManyProofType>,
    #[serde(rename = "termsOfUse", default, skip_serializing_if = "Option::is_none")]
    pub terms_of_use: Option<ManyTermsOfUseType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
    #[serde(rename = "validFrom")]
    pub valid_from: DateTimeType,
    #[serde(rename = "validUntil", default, skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<DateTimeType>,
}
impl From<&EuropeanDigitalCredentialType> for EuropeanDigitalCredentialType {
    fn from(value: &EuropeanDigitalCredentialType) -> Self {
        value.clone()
    }
}
impl EuropeanDigitalCredentialType {
    pub fn builder() -> builder::EuropeanDigitalCredentialType {
        Default::default()
    }
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
    pub proof: Option<ManyProofType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
    #[serde(
        rename = "verifiableCredential",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verifiable_credential: Option<ManyEuropeanDigitalCredentialType>,
    #[serde(
        rename = "verificationCheck",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verification_check: Option<ManyVerificationCheckType>,
}
impl From<&EuropeanDigitalPresentationType> for EuropeanDigitalPresentationType {
    fn from(value: &EuropeanDigitalPresentationType) -> Self {
        value.clone()
    }
}
impl EuropeanDigitalPresentationType {
    pub fn builder() -> builder::EuropeanDigitalPresentationType {
        Default::default()
    }
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
impl From<&Evidence> for Evidence {
    fn from(value: &Evidence) -> Self {
        value.clone()
    }
}
impl Evidence {
    pub fn builder() -> builder::Evidence {
        Default::default()
    }
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
    pub accreditation: Option<Box<AccreditationType>>,
    #[serde(
        rename = "embeddedEvidence",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub embedded_evidence: Option<ManyMediaObjectType>,
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
impl From<&EvidenceType> for EvidenceType {
    fn from(value: &EvidenceType) -> Self {
        value.clone()
    }
}
impl EvidenceType {
    pub fn builder() -> builder::EvidenceType {
        Default::default()
    }
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
pub struct GeometryType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latitude: Option<StringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<StringType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&GeometryType> for GeometryType {
    fn from(value: &GeometryType) -> Self {
        value.clone()
    }
}
impl GeometryType {
    pub fn builder() -> builder::GeometryType {
        Default::default()
    }
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
    pub supplementary_document: Option<ManyWebResourceType>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&GradingSchemeType> for GradingSchemeType {
    fn from(value: &GradingSchemeType) -> Self {
        value.clone()
    }
}
impl GradingSchemeType {
    pub fn builder() -> builder::GradingSchemeType {
        Default::default()
    }
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
pub struct GrantType {
    #[serde(rename = "contentURL", default, skip_serializing_if = "Option::is_none")]
    pub content_url: Option<UriType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ConceptType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ManyWebResourceType>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&GrantType> for GrantType {
    fn from(value: &GrantType) -> Self {
        value.clone()
    }
}
impl GrantType {
    pub fn builder() -> builder::GrantType {
        Default::default()
    }
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
    pub additional_note: Option<ManyNoteType>,
    #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<ManyLangStringType>,
    #[serde(rename = "contactPoint", default, skip_serializing_if = "Option::is_none")]
    pub contact_point: Option<ManyContactPointType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<ManyLocationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<ManyAgentOrPersonOrOrganisationType>,
    #[serde(rename = "prefLabel")]
    pub pref_label: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&Group> for Group {
    fn from(value: &Group) -> Self {
        value.clone()
    }
}
impl Group {
    pub fn builder() -> builder::GroupType {
        Default::default()
    }
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
    LegalIdentifierType(LegalIdentifierType),
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
impl From<LegalIdentifierType> for IdentifierOrLegalIdentifierType {
    fn from(value: LegalIdentifierType) -> Self {
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
    pub dc_type: Option<ObjectOrVector<ConceptType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    pub notation: LiteralType,
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
impl From<&IdentifierType> for IdentifierType {
    fn from(value: &IdentifierType) -> Self {
        value.clone()
    }
}
impl IdentifierType {
    pub fn builder() -> builder::IdentifierType {
        Default::default()
    }
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
pub struct IndividualDisplayType {
    #[serde(rename = "displayDetail")]
    pub display_detail: ManyDisplayDetailType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    pub language: ConceptType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&IndividualDisplayType> for IndividualDisplayType {
    fn from(value: &IndividualDisplayType) -> Self {
        value.clone()
    }
}
impl IndividualDisplayType {
    pub fn builder() -> builder::IndividualDisplayType {
        Default::default()
    }
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
    pub eidas_legal_identifier: LegalIdentifierType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&IssuerNodeType> for IssuerNodeType {
    fn from(value: &IssuerNodeType) -> Self {
        value.clone()
    }
}
impl IssuerNodeType {
    pub fn builder() -> builder::IssuerNodeType {
        Default::default()
    }
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
    pub subtype_0: Option<LearningAchievementSpecificationType>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<QualificationType>,
}
impl From<&LearningAchievementSpecificationOrQualificationType>
for LearningAchievementSpecificationOrQualificationType {
    fn from(value: &LearningAchievementSpecificationOrQualificationType) -> Self {
        value.clone()
    }
}
impl LearningAchievementSpecificationOrQualificationType {
    pub fn builder() -> builder::LearningAchievementSpecificationOrQualificationType {
        Default::default()
    }
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
    pub subtype_0: Option<LearningAchievementSpecificationType>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<QualificationType>,
}
impl From<&LearningAchievementSpecificationOrSpecificationType>
for LearningAchievementSpecificationOrSpecificationType {
    fn from(value: &LearningAchievementSpecificationOrSpecificationType) -> Self {
        value.clone()
    }
}
impl LearningAchievementSpecificationOrSpecificationType {
    pub fn builder() -> builder::LearningAchievementSpecificationOrSpecificationType {
        Default::default()
    }
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
pub struct LearningAchievementSpecificationType {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ManyNoteType>,
    #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<ManyLangStringType>,
    #[serde(
        rename = "awardingOpportunity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub awarding_opportunity: Option<ManyAwardingOpportunityType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<ManyLangStringType>,
    #[serde(rename = "creditPoint", default, skip_serializing_if = "Option::is_none")]
    pub credit_point: Option<ManyCreditPointType>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTimeType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<ConceptType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(rename = "educationLevel", default, skip_serializing_if = "Option::is_none")]
    pub education_level: Option<ObjectOrVector<ConceptType>>,
    #[serde(
        rename = "educationSubject",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub education_subject: Option<ObjectOrVector<ConceptType>>,
    #[serde(rename = "entitlesTo", default, skip_serializing_if = "Option::is_none")]
    pub entitles_to: Option<ManyLearningEntitlementSpecificationType>,
    #[serde(
        rename = "entryRequirement",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub entry_requirement: Option<NoteType>,
    #[serde(rename = "generalisationOf", default)]
    pub generalisation_of: 
        Option<Box<ManyLearningAchievementSpecificationOrQualificationType>>,
    #[serde(rename = "hasPart", default)]
    pub has_part: Option<Box<ManyLearningAchievementSpecificationOrQualificationType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ManyWebResourceType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifierType>,
    #[serde(rename = "influencedBy", default)]
    pub influenced_by: Option<Box<ManyLearningActivitySpecificationType>>,
    #[serde(rename = "isPartOf", default)]
    pub is_part_of: Option<Box<ManyLearningAchievementSpecificationOrQualificationType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<ObjectOrVector<ConceptType>>,
    #[serde(
        rename = "learningOutcome",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub learning_outcome: Option<ManyLearningOutcomeType>,
    #[serde(
        rename = "learningOutcomeSummary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub learning_outcome_summary: Option<NoteType>,
    #[serde(
        rename = "learningSetting",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub learning_setting: Option<ConceptType>,
    #[serde(
        rename = "maximumDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_duration: Option<DurationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ObjectOrVector<ConceptType>>,
    #[serde(rename = "provenBy", default)]
    pub proven_by: Box<Option<ManyLearningAssessmentSpecificationType>>,
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
    pub supplementary_document: Option<ManyWebResourceType>,
    #[serde(rename = "targetGroup", default, skip_serializing_if = "Option::is_none")]
    pub target_group: Option<ObjectOrVector<ConceptType>>,
    #[serde(rename = "thematicArea", default, skip_serializing_if = "Option::is_none")]
    pub thematic_area: Option<ObjectOrVector<ConceptType>>,
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
impl From<&LearningAchievementSpecificationType>
for LearningAchievementSpecificationType {
    fn from(value: &LearningAchievementSpecificationType) -> Self {
        value.clone()
    }
}
impl LearningAchievementSpecificationType {
    pub fn builder() -> builder::LearningAchievementSpecificationType {
        Default::default()
    }
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
pub struct LearningAchievementType {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ManyNoteType>,
    #[serde(rename = "awardedBy")]
    pub awarded_by: Box<AwardingProcessType>,
    #[serde(rename = "creditReceived", default, skip_serializing_if = "Option::is_none")]
    pub credit_received: Option<ManyCreditPointType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<ConceptType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(rename = "entitlesTo", default, skip_serializing_if = "Option::is_none")]
    pub entitles_to: Option<ManyLearningEntitlementType>,
    #[serde(rename = "hasPart", default)]
    pub has_part: Box<Option<ManyLearningAchievementType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifierType>,
    #[serde(rename = "influencedBy", default, skip_serializing_if = "Option::is_none")]
    pub influenced_by: Option<ManyLearningActivityType>,
    #[serde(rename = "isPartOf", default)]
    pub is_part_of: Box<Option<ManyLearningAchievementType>>,
    #[serde(
        rename = "learningOpportunity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub learning_opportunity: Option<LearningOpportunityType>,
    #[serde(rename = "provenBy", default)]
    pub proven_by: Box<Option<ManyLearningAssessmentType>>,
    #[serde(rename = "specifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub specified_by: Option<LearningAchievementSpecificationOrQualificationType>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ManyWebResourceType>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&LearningAchievementType> for LearningAchievementType {
    fn from(value: &LearningAchievementType) -> Self {
        value.clone()
    }
}
impl LearningAchievementType {
    pub fn builder() -> builder::LearningAchievementType {
        Default::default()
    }
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
pub struct LearningActivitySpecificationType {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ManyNoteType>,
    #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<ManyLangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<ManyLangStringType>,
    #[serde(rename = "contactHour", default, skip_serializing_if = "Option::is_none")]
    pub contact_hour: Option<ManyStringType>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTimeType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<ConceptType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(
        rename = "generalisationOf",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub generalisation_of: Option<ManyLearningAchievementSpecificationType>,
    #[serde(rename = "hasPart", default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<ManyLearningAchievementSpecificationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ManyWebResourceType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(default)]
    pub influences: Box<Option<ManyLearningAchievementSpecificationOrQualificationType>>,
    #[serde(rename = "isPartOf", default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<ManyLearningAchievementSpecificationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<ObjectOrVector<ConceptType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ObjectOrVector<ConceptType>>,
    #[serde(
        rename = "specialisationOf",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub specialisation_of: Option<ManyLearningAchievementSpecificationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StringType>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ManyWebResourceType>,
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
impl From<&LearningActivitySpecificationType> for LearningActivitySpecificationType {
    fn from(value: &LearningActivitySpecificationType) -> Self {
        value.clone()
    }
}
impl LearningActivitySpecificationType {
    pub fn builder() -> builder::LearningActivitySpecificationType {
        Default::default()
    }
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
pub struct LearningActivityType {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ManyNoteType>,
    #[serde(rename = "awardedBy")]
    pub awarded_by: Box<AwardingProcessType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<ConceptType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(rename = "directedBy", default, skip_serializing_if = "Option::is_none")]
    pub directed_by: Option<ManyAgentOrPersonOrOrganisationType>,
    #[serde(rename = "hasPart", default)]
    pub has_part: Box<Option<ManyLearningActivityType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(default)]
    pub influences: Box<Option<ManyLearningAchievementType>>,
    #[serde(rename = "isPartOf", default)]
    pub is_part_of: Box<Option<ManyLearningActivityType>>,
    #[serde(
        rename = "learningOpportunity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub learning_opportunity: Option<LearningOpportunityType>,
    #[serde(
        rename = "levelOfCompletion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub level_of_completion: Option<PercentageIntegerType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<ManyLocationType>,
    #[serde(rename = "specifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub specified_by: Option<LearningActivitySpecificationType>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ManyWebResourceType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporal: Option<ManyPeriodOfTimeType>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workload: Option<DurationType>,
}
impl From<&LearningActivityType> for LearningActivityType {
    fn from(value: &LearningActivityType) -> Self {
        value.clone()
    }
}
impl LearningActivityType {
    pub fn builder() -> builder::LearningActivityType {
        Default::default()
    }
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
pub struct LearningAssessmentSpecificationType {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ManyNoteType>,
    #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<ManyLangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<ManyLangStringType>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTimeType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ConceptType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(rename = "generalisationOf", default)]
    pub generalisation_of: Box<Option<ManyLearningAssessmentSpecificationType>>,
    #[serde(rename = "gradingScheme", default, skip_serializing_if = "Option::is_none")]
    pub grading_scheme: Option<GradingSchemeType>,
    #[serde(rename = "hasPart", default)]
    pub has_part: Box<Option<ManyLearningAssessmentSpecificationType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ManyWebResourceType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(rename = "isPartOf", default)]
    pub is_part_of: Box<Option<ManyLearningAssessmentSpecificationType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<ObjectOrVector<ConceptType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ObjectOrVector<ConceptType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proves: Option<ManyLearningAchievementSpecificationOrQualificationType>,
    #[serde(rename = "specialisationOf", default)]
    pub specialisation_of: Box<Option<ManyLearningAssessmentSpecificationType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StringType>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ManyWebResourceType>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&LearningAssessmentSpecificationType> for LearningAssessmentSpecificationType {
    fn from(value: &LearningAssessmentSpecificationType) -> Self {
        value.clone()
    }
}
impl LearningAssessmentSpecificationType {
    pub fn builder() -> builder::LearningAssessmentSpecificationType {
        Default::default()
    }
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
pub struct LearningAssessmentType {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ManyNoteType>,
    #[serde(rename = "assessedBy", default, skip_serializing_if = "Option::is_none")]
    pub assessed_by: Option<ManyAgentOrPersonOrOrganisationType>,
    #[serde(rename = "awardedBy")]
    pub awarded_by: Box<AwardingProcessType>,
    #[serde(rename = "dateIssued", default, skip_serializing_if = "Option::is_none")]
    pub date_issued: Option<DateTimeType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<ConceptType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    pub grade: NoteType,
    #[serde(rename = "gradeStatus", default, skip_serializing_if = "Option::is_none")]
    pub grade_status: Option<ConceptType>,
    #[serde(rename = "hasPart", default)]
    pub has_part: Box<Option<ManyLearningAssessmentType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "idVerification", default, skip_serializing_if = "Option::is_none")]
    pub id_verification: Option<ConceptType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(rename = "isPartOf", default)]
    pub is_part_of: Box<Option<ManyLearningAssessmentType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proves: Option<ManyLearningAchievementType>,
    #[serde(
        rename = "resultDistribution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_distribution: Option<ResultDistributionType>,
    #[serde(
        rename = "shortenedGrading",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shortened_grading: Option<ShortenedGradingType>,
    #[serde(rename = "specifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub specified_by: Option<ManyLearningAssessmentSpecificationType>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ManyWebResourceType>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&LearningAssessmentType> for LearningAssessmentType {
    fn from(value: &LearningAssessmentType) -> Self {
        value.clone()
    }
}
impl LearningAssessmentType {
    pub fn builder() -> builder::LearningAssessmentType {
        Default::default()
    }
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
pub struct LearningEntitlementSpecificationType {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ManyNoteType>,
    #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<ManyLangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<ManyLangStringType>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTimeType>,
    #[serde(rename = "dcType")]
    pub dc_type: ConceptType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(rename = "entitledBy", default)]
    pub entitled_by: Box<
        Option<ManyLearningAchievementSpecificationOrQualificationType>,
    >,
    #[serde(rename = "entitlementStatus")]
    pub entitlement_status: ConceptType,
    #[serde(rename = "generalisationOf", default)]
    pub generalisation_of: Box<Option<ManyLearningEntitlementSpecificationType>>,
    #[serde(rename = "hasPart", default)]
    pub has_part: Box<Option<ManyLearningEntitlementSpecificationType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ManyWebResourceType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(rename = "isPartOf", default)]
    pub is_part_of: Box<Option<ManyLearningEntitlementSpecificationType>>,
    #[serde(
        rename = "limitJurisdiction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_jurisdiction: Option<ObjectOrVector<ConceptType>>,
    #[serde(
        rename = "limitNationalOccupation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_national_occupation: Option<ObjectOrVector<ConceptType>>,
    #[serde(
        rename = "limitOccupation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_occupation: Option<ObjectOrVector<ConceptType>>,
    #[serde(rename = "limitOrganisation", default)]
    pub limit_organisation: Box<Option<ManyOrganisationType>>,
    #[serde(rename = "specialisationOf", default)]
    pub specialisation_of: Box<Option<ManyLearningEntitlementSpecificationType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StringType>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ManyWebResourceType>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&LearningEntitlementSpecificationType>
for LearningEntitlementSpecificationType {
    fn from(value: &LearningEntitlementSpecificationType) -> Self {
        value.clone()
    }
}
impl LearningEntitlementSpecificationType {
    pub fn builder() -> builder::LearningEntitlementSpecificationType {
        Default::default()
    }
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
pub struct LearningEntitlementType {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ManyNoteType>,
    #[serde(rename = "awardedBy")]
    pub awarded_by: Box<AwardingProcessType>,
    #[serde(rename = "dateIssued", default, skip_serializing_if = "Option::is_none")]
    pub date_issued: Option<DateTimeType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<ConceptType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(rename = "entitledBy", default)]
    pub entitled_by: Box<Option<ManyLearningAchievementType>>,
    #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<DateTimeType>,
    #[serde(rename = "hasPart", default)]
    pub has_part: Box<Option<ManyLearningEntitlementType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(rename = "isPartOf", default)]
    pub is_part_of: Box<Option<ManyLearningEntitlementType>>,
    #[serde(rename = "specifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub specified_by: Option<ManyLearningEntitlementSpecificationType>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ManyWebResourceType>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&LearningEntitlementType> for LearningEntitlementType {
    fn from(value: &LearningEntitlementType) -> Self {
        value.clone()
    }
}
impl LearningEntitlementType {
    pub fn builder() -> builder::LearningEntitlementType {
        Default::default()
    }
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
pub struct LearningOpportunityType {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ManyNoteType>,
    #[serde(
        rename = "admissionProcedure",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub admission_procedure: Option<NoteType>,
    #[serde(
        rename = "applicationDeadline",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub application_deadline: Option<ManyDateTimeType>,
    #[serde(rename = "bannerImage", default, skip_serializing_if = "Option::is_none")]
    pub banner_image: Option<MediaObjectType>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTimeType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<ConceptType>>,
    #[serde(
        rename = "defaultLanguage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_language: Option<ConceptType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(
        rename = "descriptionHtml",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description_html: Option<ManyHtmlType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<DurationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant: Option<ManyGrantType>,
    #[serde(rename = "hasPart", default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<ManyLearningOpportunityType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ManyWebResourceType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ManyIdentifierOrLegalIdentifierType>,
    #[serde(rename = "isPartOf", default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<ManyLearningOpportunityType>,
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
    pub learning_activity_specification: Option<LearningActivitySpecificationType>,
    #[serde(
        rename = "learningSchedule",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub learning_schedule: Option<ConceptType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<ManyLocationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ObjectOrVector<ConceptType>>,
    #[serde(rename = "priceDetail", default, skip_serializing_if = "Option::is_none")]
    pub price_detail: Option<ManyPriceDetailType>,
    #[serde(rename = "providedBy", default, skip_serializing_if = "Option::is_none")]
    pub provided_by: Option<Box<OrganisationType>>,
    #[serde(
        rename = "scheduleInformation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub schedule_information: Option<NoteType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StringType>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ManyWebResourceType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporal: Option<PeriodOfTimeType>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&LearningOpportunityType> for LearningOpportunityType {
    fn from(value: &LearningOpportunityType) -> Self {
        value.clone()
    }
}
impl LearningOpportunityType {
    pub fn builder() -> builder::LearningOpportunityType {
        Default::default()
    }
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
pub struct LearningOutcomeType {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ManyNoteType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ConceptType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifierType>,
    #[serde(
        rename = "relatedESCOSkill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub related_esco_skill: Option<ObjectOrVector<ConceptType>>,
    #[serde(rename = "relatedSkill", default, skip_serializing_if = "Option::is_none")]
    pub related_skill: Option<ObjectOrVector<ConceptType>>,
    #[serde(
        rename = "reusabilityLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reusability_level: Option<ConceptType>,
    pub title: ManyLangStringType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&LearningOutcomeType> for LearningOutcomeType {
    fn from(value: &LearningOutcomeType) -> Self {
        value.clone()
    }
}
impl LearningOutcomeType {
    pub fn builder() -> builder::LearningOutcomeType {
        Default::default()
    }
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
pub struct LegalIdentifierType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<IriType>,
    #[serde(rename = "dateIssued", default, skip_serializing_if = "Option::is_none")]
    pub date_issued: Option<DateTimeType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<ConceptType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    pub notation: LiteralType,
    #[serde(rename = "schemeAgency", default, skip_serializing_if = "Option::is_none")]
    pub scheme_agency: Option<LangStringType>,
    #[serde(rename = "schemeId", default, skip_serializing_if = "Option::is_none")]
    pub scheme_id: Option<UriType>,
    #[serde(rename = "schemeName", default, skip_serializing_if = "Option::is_none")]
    pub scheme_name: Option<StringType>,
    #[serde(rename = "schemeVersion", default, skip_serializing_if = "Option::is_none")]
    pub scheme_version: Option<StringType>,
    pub spatial: ConceptType,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&LegalIdentifierType> for LegalIdentifierType {
    fn from(value: &LegalIdentifierType) -> Self {
        value.clone()
    }
}
impl LegalIdentifierType {
    pub fn builder() -> builder::LegalIdentifierType {
        Default::default()
    }
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
pub struct LiteralType(pub StringType);
impl std::ops::Deref for LiteralType {
    type Target = StringType;
    fn deref(&self) -> &StringType {
        &self.0
    }
}
impl From<LiteralType> for StringType {
    fn from(value: LiteralType) -> Self {
        value.0
    }
}
impl From<&LiteralType> for LiteralType {
    fn from(value: &LiteralType) -> Self {
        value.clone()
    }
}
impl From<StringType> for LiteralType {
    fn from(value: StringType) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for LiteralType {
    type Err = <StringType as std::str::FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl std::convert::TryFrom<&str> for LiteralType {
    type Error = <StringType as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LiteralType {
    type Error = <StringType as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LiteralType {
    type Error = <StringType as std::str::FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl ToString for LiteralType {
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
pub struct LocationType {
    pub address: ManyAddressType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(rename = "geographicName", default, skip_serializing_if = "Option::is_none")]
    pub geographic_name: Option<ManyAddressType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geometry: Option<ManyGeometryType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifierType>,
    #[serde(rename = "spatialCode", default, skip_serializing_if = "Option::is_none")]
    pub spatial_code: Option<ObjectOrVector<ConceptType>>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&LocationType> for LocationType {
    fn from(value: &LocationType) -> Self {
        value.clone()
    }
}
impl LocationType {
    pub fn builder() -> builder::LocationType {
        Default::default()
    }
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
pub struct MailboxType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<EmailType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&MailboxType> for MailboxType {
    fn from(value: &MailboxType) -> Self {
        value.clone()
    }
}
impl MailboxType {
    pub fn builder() -> builder::MailboxType {
        Default::default()
    }
}
///ManyAccreditationType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/AccreditationType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/AccreditationType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyAccreditationType {
    Variant0(Box<AccreditationType>),
    Variant1(Vec<AccreditationType>),
}
impl From<&ManyAccreditationType> for ManyAccreditationType {
    fn from(value: &ManyAccreditationType) -> Self {
        value.clone()
    }
}
impl From<Box<AccreditationType>> for ManyAccreditationType {
    fn from(value: Box<AccreditationType>) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<AccreditationType>> for ManyAccreditationType {
    fn from(value: Vec<AccreditationType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyAddressType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/AddressType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/AddressType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyAddressType {
    Variant0(AddressType),
    Variant1(Vec<AddressType>),
}
impl From<&ManyAddressType> for ManyAddressType {
    fn from(value: &ManyAddressType) -> Self {
        value.clone()
    }
}
impl From<AddressType> for ManyAddressType {
    fn from(value: AddressType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<AddressType>> for ManyAddressType {
    fn from(value: Vec<AddressType>) -> Self {
        Self::Variant1(value)
    }
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
impl From<&ManyAgentOrPersonOrOrganisationType> for ManyAgentOrPersonOrOrganisationType {
    fn from(value: &ManyAgentOrPersonOrOrganisationType) -> Self {
        value.clone()
    }
}
impl ManyAgentOrPersonOrOrganisationType {
    pub fn builder() -> builder::ManyAgentOrPersonOrOrganisationType {
        Default::default()
    }
}
///ManyAgentType
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
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/AgentType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyAgentType {
    Variant0(AgentType),
    Variant1(Vec<AgentType>),
}
impl From<&ManyAgentType> for ManyAgentType {
    fn from(value: &ManyAgentType) -> Self {
        value.clone()
    }
}
impl From<AgentType> for ManyAgentType {
    fn from(value: AgentType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<AgentType>> for ManyAgentType {
    fn from(value: Vec<AgentType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyAmountType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/AmountType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/AmountType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyAmountType {
    Variant0(AmountType),
    Variant1(Vec<AmountType>),
}
impl From<&ManyAmountType> for ManyAmountType {
    fn from(value: &ManyAmountType) -> Self {
        value.clone()
    }
}
impl From<AmountType> for ManyAmountType {
    fn from(value: AmountType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<AmountType>> for ManyAmountType {
    fn from(value: Vec<AmountType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyAwardingOpportunityType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/AwardingOpportunityType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/AwardingOpportunityType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyAwardingOpportunityType {
    Variant0(AwardingOpportunityType),
    Variant1(Vec<AwardingOpportunityType>),
}
impl From<&ManyAwardingOpportunityType> for ManyAwardingOpportunityType {
    fn from(value: &ManyAwardingOpportunityType) -> Self {
        value.clone()
    }
}
impl From<AwardingOpportunityType> for ManyAwardingOpportunityType {
    fn from(value: AwardingOpportunityType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<AwardingOpportunityType>> for ManyAwardingOpportunityType {
    fn from(value: Vec<AwardingOpportunityType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyAwardingProcessType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/AwardingProcessType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/AwardingProcessType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyAwardingProcessType {
    Variant0(AwardingProcessType),
    Variant1(Vec<AwardingProcessType>),
}
impl From<&ManyAwardingProcessType> for ManyAwardingProcessType {
    fn from(value: &ManyAwardingProcessType) -> Self {
        value.clone()
    }
}
impl From<AwardingProcessType> for ManyAwardingProcessType {
    fn from(value: AwardingProcessType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<AwardingProcessType>> for ManyAwardingProcessType {
    fn from(value: Vec<AwardingProcessType>) -> Self {
        Self::Variant1(value)
    }
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
    pub subtype_0: Option<ClaimNodeType>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<Vec<ClaimNodeType>>,
}
impl From<&ManyClaimNodeType> for ManyClaimNodeType {
    fn from(value: &ManyClaimNodeType) -> Self {
        value.clone()
    }
}
impl ManyClaimNodeType {
    pub fn builder() -> builder::ManyClaimNodeType {
        Default::default()
    }
}
///ManyClaimTypeNodeType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/ClaimTypeNodeType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ClaimTypeNodeType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyClaimTypeNodeType {
    Variant0(ClaimTypeNodeType),
    Variant1(Vec<ClaimTypeNodeType>),
}
impl From<&ManyClaimTypeNodeType> for ManyClaimTypeNodeType {
    fn from(value: &ManyClaimTypeNodeType) -> Self {
        value.clone()
    }
}
impl From<ClaimTypeNodeType> for ManyClaimTypeNodeType {
    fn from(value: ClaimTypeNodeType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<ClaimTypeNodeType>> for ManyClaimTypeNodeType {
    fn from(value: Vec<ClaimTypeNodeType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyConceptType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/ConceptType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ConceptType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyConceptType {
    Object(ConceptType),
    Vector(Vec<ConceptType>),
}
impl From<&ManyConceptType> for ManyConceptType {
    fn from(value: &ManyConceptType) -> Self {
        value.clone()
    }
}
impl From<ConceptType> for ManyConceptType {
    fn from(value: ConceptType) -> Self {
        Self::Object(value)
    }
}
impl From<Vec<ConceptType>> for ManyConceptType {
    fn from(value: Vec<ConceptType>) -> Self {
        Self::Vector(value)
    }
}
///ManyContactPointType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/ContactPointType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ContactPointType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyContactPointType {
    Object(ContactPointType),
    Vector(Vec<ContactPointType>),
}
impl From<&ManyContactPointType> for ManyContactPointType {
    fn from(value: &ManyContactPointType) -> Self {
        value.clone()
    }
}
impl From<ContactPointType> for ManyContactPointType {
    fn from(value: ContactPointType) -> Self {
        Self::Object(value)
    }
}
impl From<Vec<ContactPointType>> for ManyContactPointType {
    fn from(value: Vec<ContactPointType>) -> Self {
        Self::Vector(value)
    }
}
///ManyCredentialStatusType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/CredentialStatusType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/CredentialStatusType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyCredentialStatusType {
    Object(CredentialStatusType),
    Vector(Vec<CredentialStatusType>),
}
impl From<&ManyCredentialStatusType> for ManyCredentialStatusType {
    fn from(value: &ManyCredentialStatusType) -> Self {
        value.clone()
    }
}
impl From<CredentialStatusType> for ManyCredentialStatusType {
    fn from(value: CredentialStatusType) -> Self {
        Self::Object(value)
    }
}
impl From<Vec<CredentialStatusType>> for ManyCredentialStatusType {
    fn from(value: Vec<CredentialStatusType>) -> Self {
        Self::Vector(value)
    }
}
///ManyCreditPointType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/CreditPointType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/CreditPointType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyCreditPointType {
    Object(CreditPointType),
    Vector(Vec<CreditPointType>),
}
impl From<&ManyCreditPointType> for ManyCreditPointType {
    fn from(value: &ManyCreditPointType) -> Self {
        value.clone()
    }
}
impl From<CreditPointType> for ManyCreditPointType {
    fn from(value: CreditPointType) -> Self {
        Self::Object(value)
    }
}
impl From<Vec<CreditPointType>> for ManyCreditPointType {
    fn from(value: Vec<CreditPointType>) -> Self {
        Self::Vector(value)
    }
}
///ManyDateTimeType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/DateTimeType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/DateTimeType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyDateTimeType {
    Object(DateTimeType),
    Vector(Vec<DateTimeType>),
}
impl From<&ManyDateTimeType> for ManyDateTimeType {
    fn from(value: &ManyDateTimeType) -> Self {
        value.clone()
    }
}
impl From<DateTimeType> for ManyDateTimeType {
    fn from(value: DateTimeType) -> Self {
        Self::Object(value)
    }
}
impl From<Vec<DateTimeType>> for ManyDateTimeType {
    fn from(value: Vec<DateTimeType>) -> Self {
        Self::Vector(value)
    }
}
///ManyDisplayDetailType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/DisplayDetailType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/DisplayDetailType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyDisplayDetailType {
    Object(DisplayDetailType),
    Vector(Vec<DisplayDetailType>),
}
impl From<&ManyDisplayDetailType> for ManyDisplayDetailType {
    fn from(value: &ManyDisplayDetailType) -> Self {
        value.clone()
    }
}
impl From<DisplayDetailType> for ManyDisplayDetailType {
    fn from(value: DisplayDetailType) -> Self {
        Self::Object(value)
    }
}
impl From<Vec<DisplayDetailType>> for ManyDisplayDetailType {
    fn from(value: Vec<DisplayDetailType>) -> Self {
        Self::Vector(value)
    }
}
///ManyDisplayParameterType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/DisplayParameterType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/DisplayParameterType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyDisplayParameterType {
    Object(DisplayParameterType),
    Vector(Vec<DisplayParameterType>),
}
impl From<&ManyDisplayParameterType> for ManyDisplayParameterType {
    fn from(value: &ManyDisplayParameterType) -> Self {
        value.clone()
    }
}
impl From<DisplayParameterType> for ManyDisplayParameterType {
    fn from(value: DisplayParameterType) -> Self {
        Self::Object(value)
    }
}
impl From<Vec<DisplayParameterType>> for ManyDisplayParameterType {
    fn from(value: Vec<DisplayParameterType>) -> Self {
        Self::Vector(value)
    }
}
///ManyEuropeanDigitalCredentialType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/EuropeanDigitalCredentialType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/EuropeanDigitalCredentialType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyEuropeanDigitalCredentialType {
    Object(EuropeanDigitalCredentialType),
    Vector(Vec<EuropeanDigitalCredentialType>),
}
impl From<&ManyEuropeanDigitalCredentialType> for ManyEuropeanDigitalCredentialType {
    fn from(value: &ManyEuropeanDigitalCredentialType) -> Self {
        value.clone()
    }
}
impl From<EuropeanDigitalCredentialType> for ManyEuropeanDigitalCredentialType {
    fn from(value: EuropeanDigitalCredentialType) -> Self {
        Self::Object(value)
    }
}
impl From<Vec<EuropeanDigitalCredentialType>> for ManyEuropeanDigitalCredentialType {
    fn from(value: Vec<EuropeanDigitalCredentialType>) -> Self {
        Self::Vector(value)
    }
}
///ManyEuropeanDigitalPresentationType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/EuropeanDigitalPresentationType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/EuropeanDigitalPresentationType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyEuropeanDigitalPresentationType {
    Object(EuropeanDigitalPresentationType),
    Vector(Vec<EuropeanDigitalPresentationType>),
}
impl From<&ManyEuropeanDigitalPresentationType> for ManyEuropeanDigitalPresentationType {
    fn from(value: &ManyEuropeanDigitalPresentationType) -> Self {
        value.clone()
    }
}
impl From<EuropeanDigitalPresentationType> for ManyEuropeanDigitalPresentationType {
    fn from(value: EuropeanDigitalPresentationType) -> Self {
        Self::Object(value)
    }
}
impl From<Vec<EuropeanDigitalPresentationType>> for ManyEuropeanDigitalPresentationType {
    fn from(value: Vec<EuropeanDigitalPresentationType>) -> Self {
        Self::Vector(value)
    }
}
///ManyEvidenceType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/EvidenceType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/EvidenceType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyEvidenceType {
    Object(EvidenceType),
    Vector(Vec<EvidenceType>),
}
impl From<&ManyEvidenceType> for ManyEvidenceType {
    fn from(value: &ManyEvidenceType) -> Self {
        value.clone()
    }
}
impl From<EvidenceType> for ManyEvidenceType {
    fn from(value: EvidenceType) -> Self {
        Self::Object(value)
    }
}
impl From<Vec<EvidenceType>> for ManyEvidenceType {
    fn from(value: Vec<EvidenceType>) -> Self {
        Self::Vector(value)
    }
}
///ManyGeometryType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/GeometryType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/GeometryType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyGeometryType {
    Object(GeometryType),
    Vector(Vec<GeometryType>),
}
impl From<&ManyGeometryType> for ManyGeometryType {
    fn from(value: &ManyGeometryType) -> Self {
        value.clone()
    }
}
impl From<GeometryType> for ManyGeometryType {
    fn from(value: GeometryType) -> Self {
        Self::Object(value)
    }
}
impl From<Vec<GeometryType>> for ManyGeometryType {
    fn from(value: Vec<GeometryType>) -> Self {
        Self::Vector(value)
    }
}
///ManyGradingSchemeType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/GradingSchemeType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/GradingSchemeType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyGradingSchemeType {
    Object(GradingSchemeType),
    Vector(Vec<GradingSchemeType>),
}
impl From<&ManyGradingSchemeType> for ManyGradingSchemeType {
    fn from(value: &ManyGradingSchemeType) -> Self {
        value.clone()
    }
}
impl From<GradingSchemeType> for ManyGradingSchemeType {
    fn from(value: GradingSchemeType) -> Self {
        Self::Object(value)
    }
}
impl From<Vec<GradingSchemeType>> for ManyGradingSchemeType {
    fn from(value: Vec<GradingSchemeType>) -> Self {
        Self::Vector(value)
    }
}
///ManyGrantType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/GrantType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/GrantType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyGrantType {
    Object(GrantType),
    Vector(Vec<GrantType>),
}
impl From<&ManyGrantType> for ManyGrantType {
    fn from(value: &ManyGrantType) -> Self {
        value.clone()
    }
}
impl From<GrantType> for ManyGrantType {
    fn from(value: GrantType) -> Self {
        Self::Object(value)
    }
}
impl From<Vec<GrantType>> for ManyGrantType {
    fn from(value: Vec<GrantType>) -> Self {
        Self::Vector(value)
    }
}
///ManyGroupType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/GroupType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/GroupType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyGroupType {
    Object(Group),
    Vector(Vec<Group>),
}
impl From<&ManyGroupType> for ManyGroupType {
    fn from(value: &ManyGroupType) -> Self {
        value.clone()
    }
}
impl From<Group> for ManyGroupType {
    fn from(value: Group) -> Self {
        Self::Object(value)
    }
}
impl From<Vec<Group>> for ManyGroupType {
    fn from(value: Vec<Group>) -> Self {
        Self::Vector(value)
    }
}
///ManyHtmlType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/HTMLType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/HTMLType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyHtmlType {
    Object(HtmlType),
    Vector(Vec<HtmlType>),
}
impl From<&ManyHtmlType> for ManyHtmlType {
    fn from(value: &ManyHtmlType) -> Self {
        value.clone()
    }
}
impl From<HtmlType> for ManyHtmlType {
    fn from(value: HtmlType) -> Self {
        Self::Object(value)
    }
}
impl From<Vec<HtmlType>> for ManyHtmlType {
    fn from(value: Vec<HtmlType>) -> Self {
        Self::Vector(value)
    }
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
    pub subtype_0: Option<IdentifierOrLegalIdentifierType>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<Vec<IdentifierOrLegalIdentifierType>>,
}
impl From<&ManyIdentifierOrLegalIdentifierType> for ManyIdentifierOrLegalIdentifierType {
    fn from(value: &ManyIdentifierOrLegalIdentifierType) -> Self {
        value.clone()
    }
}
impl ManyIdentifierOrLegalIdentifierType {
    pub fn builder() -> builder::ManyIdentifierOrLegalIdentifierType {
        Default::default()
    }
}
///ManyIdentifierType
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
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/IdentifierType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyIdentifierType {
    Object(IdentifierType),
    Vector(Vec<IdentifierType>),
}
impl From<&ManyIdentifierType> for ManyIdentifierType {
    fn from(value: &ManyIdentifierType) -> Self {
        value.clone()
    }
}
impl From<IdentifierType> for ManyIdentifierType {
    fn from(value: IdentifierType) -> Self {
        Self::Object(value)
    }
}
impl From<Vec<IdentifierType>> for ManyIdentifierType {
    fn from(value: Vec<IdentifierType>) -> Self {
        Self::Vector(value)
    }
}
///ManyIndividualDisplayType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/IndividualDisplayType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/IndividualDisplayType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyIndividualDisplayType {
    Variant0(IndividualDisplayType),
    Variant1(Vec<IndividualDisplayType>),
}
impl From<&ManyIndividualDisplayType> for ManyIndividualDisplayType {
    fn from(value: &ManyIndividualDisplayType) -> Self {
        value.clone()
    }
}
impl From<IndividualDisplayType> for ManyIndividualDisplayType {
    fn from(value: IndividualDisplayType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<IndividualDisplayType>> for ManyIndividualDisplayType {
    fn from(value: Vec<IndividualDisplayType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyIssuerNodeType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/IssuerNodeType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/IssuerNodeType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyIssuerNodeType {
    Variant0(IssuerNodeType),
    Variant1(Vec<IssuerNodeType>),
}
impl From<&ManyIssuerNodeType> for ManyIssuerNodeType {
    fn from(value: &ManyIssuerNodeType) -> Self {
        value.clone()
    }
}
impl From<IssuerNodeType> for ManyIssuerNodeType {
    fn from(value: IssuerNodeType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<IssuerNodeType>> for ManyIssuerNodeType {
    fn from(value: Vec<IssuerNodeType>) -> Self {
        Self::Variant1(value)
    }
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
    pub std::collections::HashMap<ManyLangStringTypeKey, serde_json::Value>,
);
impl std::ops::Deref for ManyLangStringType {
    type Target = std::collections::HashMap<ManyLangStringTypeKey, serde_json::Value>;
    fn deref(
        &self,
    ) -> &std::collections::HashMap<ManyLangStringTypeKey, serde_json::Value> {
        &self.0
    }
}
impl From<ManyLangStringType>
for std::collections::HashMap<ManyLangStringTypeKey, serde_json::Value> {
    fn from(value: ManyLangStringType) -> Self {
        value.0
    }
}
impl From<&ManyLangStringType> for ManyLangStringType {
    fn from(value: &ManyLangStringType) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<ManyLangStringTypeKey, serde_json::Value>>
for ManyLangStringType {
    fn from(
        value: std::collections::HashMap<ManyLangStringTypeKey, serde_json::Value>,
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
pub struct ManyLangStringTypeKey(String);
impl std::ops::Deref for ManyLangStringTypeKey {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ManyLangStringTypeKey> for String {
    fn from(value: ManyLangStringTypeKey) -> Self {
        value.0
    }
}
impl From<&ManyLangStringTypeKey> for ManyLangStringTypeKey {
    fn from(value: &ManyLangStringTypeKey) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for ManyLangStringTypeKey {
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
impl std::convert::TryFrom<&str> for ManyLangStringTypeKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ManyLangStringTypeKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ManyLangStringTypeKey {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for ManyLangStringTypeKey {
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
    pub subtype_0: Option<LearningAchievementSpecificationOrQualificationType>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<Vec<LearningAchievementSpecificationOrQualificationType>>,
}
impl From<&ManyLearningAchievementSpecificationOrQualificationType>
for ManyLearningAchievementSpecificationOrQualificationType {
    fn from(value: &ManyLearningAchievementSpecificationOrQualificationType) -> Self {
        value.clone()
    }
}
impl ManyLearningAchievementSpecificationOrQualificationType {
    pub fn builder() -> builder::ManyLearningAchievementSpecificationOrQualificationType {
        Default::default()
    }
}
///ManyLearningAchievementSpecificationType
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
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/LearningAchievementSpecificationType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyLearningAchievementSpecificationType {
    Variant0(LearningAchievementSpecificationType),
    Variant1(Vec<LearningAchievementSpecificationType>),
}
impl From<&ManyLearningAchievementSpecificationType>
for ManyLearningAchievementSpecificationType {
    fn from(value: &ManyLearningAchievementSpecificationType) -> Self {
        value.clone()
    }
}
impl From<LearningAchievementSpecificationType>
for ManyLearningAchievementSpecificationType {
    fn from(value: LearningAchievementSpecificationType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<LearningAchievementSpecificationType>>
for ManyLearningAchievementSpecificationType {
    fn from(value: Vec<LearningAchievementSpecificationType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyLearningAchievementType
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
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/LearningAchievementType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyLearningAchievementType {
    Variant0(LearningAchievementType),
    Variant1(Vec<LearningAchievementType>),
}
impl From<&ManyLearningAchievementType> for ManyLearningAchievementType {
    fn from(value: &ManyLearningAchievementType) -> Self {
        value.clone()
    }
}
impl From<LearningAchievementType> for ManyLearningAchievementType {
    fn from(value: LearningAchievementType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<LearningAchievementType>> for ManyLearningAchievementType {
    fn from(value: Vec<LearningAchievementType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyLearningActivitySpecificationType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/LearningActivitySpecificationType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/LearningActivitySpecificationType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyLearningActivitySpecificationType {
    Variant0(LearningActivitySpecificationType),
    Variant1(Vec<LearningActivitySpecificationType>),
}
impl From<&ManyLearningActivitySpecificationType>
for ManyLearningActivitySpecificationType {
    fn from(value: &ManyLearningActivitySpecificationType) -> Self {
        value.clone()
    }
}
impl From<LearningActivitySpecificationType> for ManyLearningActivitySpecificationType {
    fn from(value: LearningActivitySpecificationType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<LearningActivitySpecificationType>>
for ManyLearningActivitySpecificationType {
    fn from(value: Vec<LearningActivitySpecificationType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyLearningActivityType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/LearningActivityType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/LearningActivityType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyLearningActivityType {
    Variant0(LearningActivityType),
    Variant1(Vec<LearningActivityType>),
}
impl From<&ManyLearningActivityType> for ManyLearningActivityType {
    fn from(value: &ManyLearningActivityType) -> Self {
        value.clone()
    }
}
impl From<LearningActivityType> for ManyLearningActivityType {
    fn from(value: LearningActivityType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<LearningActivityType>> for ManyLearningActivityType {
    fn from(value: Vec<LearningActivityType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyLearningAssessmentSpecificationType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/LearningAssessmentSpecificationType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/LearningAssessmentSpecificationType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyLearningAssessmentSpecificationType {
    Variant0(LearningAssessmentSpecificationType),
    Variant1(Vec<LearningAssessmentSpecificationType>),
}
impl From<&ManyLearningAssessmentSpecificationType>
for ManyLearningAssessmentSpecificationType {
    fn from(value: &ManyLearningAssessmentSpecificationType) -> Self {
        value.clone()
    }
}
impl From<LearningAssessmentSpecificationType>
for ManyLearningAssessmentSpecificationType {
    fn from(value: LearningAssessmentSpecificationType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<LearningAssessmentSpecificationType>>
for ManyLearningAssessmentSpecificationType {
    fn from(value: Vec<LearningAssessmentSpecificationType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyLearningAssessmentType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/LearningAssessmentType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/LearningAssessmentType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyLearningAssessmentType {
    Variant0(LearningAssessmentType),
    Variant1(Vec<LearningAssessmentType>),
}
impl From<&ManyLearningAssessmentType> for ManyLearningAssessmentType {
    fn from(value: &ManyLearningAssessmentType) -> Self {
        value.clone()
    }
}
impl From<LearningAssessmentType> for ManyLearningAssessmentType {
    fn from(value: LearningAssessmentType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<LearningAssessmentType>> for ManyLearningAssessmentType {
    fn from(value: Vec<LearningAssessmentType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyLearningEntitlementSpecificationType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/LearningEntitlementSpecificationType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/LearningEntitlementSpecificationType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyLearningEntitlementSpecificationType {
    Variant0(LearningEntitlementSpecificationType),
    Variant1(Vec<LearningEntitlementSpecificationType>),
}
impl From<&ManyLearningEntitlementSpecificationType>
for ManyLearningEntitlementSpecificationType {
    fn from(value: &ManyLearningEntitlementSpecificationType) -> Self {
        value.clone()
    }
}
impl From<LearningEntitlementSpecificationType>
for ManyLearningEntitlementSpecificationType {
    fn from(value: LearningEntitlementSpecificationType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<LearningEntitlementSpecificationType>>
for ManyLearningEntitlementSpecificationType {
    fn from(value: Vec<LearningEntitlementSpecificationType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyLearningEntitlementType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/LearningEntitlementType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/LearningEntitlementType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyLearningEntitlementType {
    Variant0(LearningEntitlementType),
    Variant1(Vec<LearningEntitlementType>),
}
impl From<&ManyLearningEntitlementType> for ManyLearningEntitlementType {
    fn from(value: &ManyLearningEntitlementType) -> Self {
        value.clone()
    }
}
impl From<LearningEntitlementType> for ManyLearningEntitlementType {
    fn from(value: LearningEntitlementType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<LearningEntitlementType>> for ManyLearningEntitlementType {
    fn from(value: Vec<LearningEntitlementType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyLearningOpportunityType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/LearningOpportunityType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/LearningOpportunityType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyLearningOpportunityType {
    Variant0(Box<LearningOpportunityType>),
    Variant1(Vec<LearningOpportunityType>),
}
impl From<&ManyLearningOpportunityType> for ManyLearningOpportunityType {
    fn from(value: &ManyLearningOpportunityType) -> Self {
        value.clone()
    }
}
impl From<Box<LearningOpportunityType>> for ManyLearningOpportunityType {
    fn from(value: Box<LearningOpportunityType>) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<LearningOpportunityType>> for ManyLearningOpportunityType {
    fn from(value: Vec<LearningOpportunityType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyLearningOutcomeType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/LearningOutcomeType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/LearningOutcomeType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyLearningOutcomeType {
    Variant0(LearningOutcomeType),
    Variant1(Vec<LearningOutcomeType>),
}
impl From<&ManyLearningOutcomeType> for ManyLearningOutcomeType {
    fn from(value: &ManyLearningOutcomeType) -> Self {
        value.clone()
    }
}
impl From<LearningOutcomeType> for ManyLearningOutcomeType {
    fn from(value: LearningOutcomeType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<LearningOutcomeType>> for ManyLearningOutcomeType {
    fn from(value: Vec<LearningOutcomeType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyLegalIdentifierType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/LegalIdentifierType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/LegalIdentifierType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyLegalIdentifierType {
    Variant0(LegalIdentifierType),
    Variant1(Vec<LegalIdentifierType>),
}
impl From<&ManyLegalIdentifierType> for ManyLegalIdentifierType {
    fn from(value: &ManyLegalIdentifierType) -> Self {
        value.clone()
    }
}
impl From<LegalIdentifierType> for ManyLegalIdentifierType {
    fn from(value: LegalIdentifierType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<LegalIdentifierType>> for ManyLegalIdentifierType {
    fn from(value: Vec<LegalIdentifierType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyLocationType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/LocationType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/LocationType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyLocationType {
    Variant0(LocationType),
    Variant1(Vec<LocationType>),
}
impl From<&ManyLocationType> for ManyLocationType {
    fn from(value: &ManyLocationType) -> Self {
        value.clone()
    }
}
impl From<LocationType> for ManyLocationType {
    fn from(value: LocationType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<LocationType>> for ManyLocationType {
    fn from(value: Vec<LocationType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyMailboxType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/MailboxType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/MailboxType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyMailboxType {
    Variant0(MailboxType),
    Variant1(Vec<MailboxType>),
}
impl From<&ManyMailboxType> for ManyMailboxType {
    fn from(value: &ManyMailboxType) -> Self {
        value.clone()
    }
}
impl From<MailboxType> for ManyMailboxType {
    fn from(value: MailboxType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<MailboxType>> for ManyMailboxType {
    fn from(value: Vec<MailboxType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyMediaObjectType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/MediaObjectType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/MediaObjectType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyMediaObjectType {
    Variant0(MediaObjectType),
    Variant1(Vec<MediaObjectType>),
}
impl From<&ManyMediaObjectType> for ManyMediaObjectType {
    fn from(value: &ManyMediaObjectType) -> Self {
        value.clone()
    }
}
impl From<MediaObjectType> for ManyMediaObjectType {
    fn from(value: MediaObjectType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<MediaObjectType>> for ManyMediaObjectType {
    fn from(value: Vec<MediaObjectType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyNoteType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/NoteType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/NoteType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyNoteType {
    Variant0(NoteType),
    Variant1(Vec<NoteType>),
}
impl From<&ManyNoteType> for ManyNoteType {
    fn from(value: &ManyNoteType) -> Self {
        value.clone()
    }
}
impl From<NoteType> for ManyNoteType {
    fn from(value: NoteType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<NoteType>> for ManyNoteType {
    fn from(value: Vec<NoteType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyOrganisationType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/OrganisationType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/OrganisationType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyOrganisationType {
    Variant0(OrganisationType),
    Variant1(Vec<OrganisationType>),
}
impl From<&ManyOrganisationType> for ManyOrganisationType {
    fn from(value: &ManyOrganisationType) -> Self {
        value.clone()
    }
}
impl From<OrganisationType> for ManyOrganisationType {
    fn from(value: OrganisationType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<OrganisationType>> for ManyOrganisationType {
    fn from(value: Vec<OrganisationType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyPeriodOfTimeType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/PeriodOfTimeType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/PeriodOfTimeType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyPeriodOfTimeType {
    Variant0(PeriodOfTimeType),
    Variant1(Vec<PeriodOfTimeType>),
}
impl From<&ManyPeriodOfTimeType> for ManyPeriodOfTimeType {
    fn from(value: &ManyPeriodOfTimeType) -> Self {
        value.clone()
    }
}
impl From<PeriodOfTimeType> for ManyPeriodOfTimeType {
    fn from(value: PeriodOfTimeType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<PeriodOfTimeType>> for ManyPeriodOfTimeType {
    fn from(value: Vec<PeriodOfTimeType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyPersonType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/PersonType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/PersonType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyPersonType {
    Variant0(PersonType),
    Variant1(Vec<PersonType>),
}
impl From<&ManyPersonType> for ManyPersonType {
    fn from(value: &ManyPersonType) -> Self {
        value.clone()
    }
}
impl From<PersonType> for ManyPersonType {
    fn from(value: PersonType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<PersonType>> for ManyPersonType {
    fn from(value: Vec<PersonType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyPhoneType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/PhoneType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/PhoneType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyPhoneType {
    Variant0(PhoneType),
    Variant1(Vec<PhoneType>),
}
impl From<&ManyPhoneType> for ManyPhoneType {
    fn from(value: &ManyPhoneType) -> Self {
        value.clone()
    }
}
impl From<PhoneType> for ManyPhoneType {
    fn from(value: PhoneType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<PhoneType>> for ManyPhoneType {
    fn from(value: Vec<PhoneType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyPriceDetailType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/PriceDetailType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/PriceDetailType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyPriceDetailType {
    Variant0(PriceDetailType),
    Variant1(Vec<PriceDetailType>),
}
impl From<&ManyPriceDetailType> for ManyPriceDetailType {
    fn from(value: &ManyPriceDetailType) -> Self {
        value.clone()
    }
}
impl From<PriceDetailType> for ManyPriceDetailType {
    fn from(value: PriceDetailType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<PriceDetailType>> for ManyPriceDetailType {
    fn from(value: Vec<PriceDetailType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyProofType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/ProofType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ProofType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyProofType {
    Variant0(ProofType),
    Variant1(Vec<ProofType>),
}
impl From<&ManyProofType> for ManyProofType {
    fn from(value: &ManyProofType) -> Self {
        value.clone()
    }
}
impl From<ProofType> for ManyProofType {
    fn from(value: ProofType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<ProofType>> for ManyProofType {
    fn from(value: Vec<ProofType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyQualificationType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/QualificationType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/QualificationType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyQualificationType {
    Variant0(Box<QualificationType>),
    Variant1(Vec<QualificationType>),
}
impl From<&ManyQualificationType> for ManyQualificationType {
    fn from(value: &ManyQualificationType) -> Self {
        value.clone()
    }
}
impl From<Box<QualificationType>> for ManyQualificationType {
    fn from(value: Box<QualificationType>) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<QualificationType>> for ManyQualificationType {
    fn from(value: Vec<QualificationType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyResultCategoryType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/ResultCategoryType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ResultCategoryType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyResultCategoryType {
    Variant0(ResultCategoryType),
    Variant1(Vec<ResultCategoryType>),
}
impl From<&ManyResultCategoryType> for ManyResultCategoryType {
    fn from(value: &ManyResultCategoryType) -> Self {
        value.clone()
    }
}
impl From<ResultCategoryType> for ManyResultCategoryType {
    fn from(value: ResultCategoryType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<ResultCategoryType>> for ManyResultCategoryType {
    fn from(value: Vec<ResultCategoryType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyResultDistributionType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/ResultDistributionType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ResultDistributionType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyResultDistributionType {
    Variant0(ResultDistributionType),
    Variant1(Vec<ResultDistributionType>),
}
impl From<&ManyResultDistributionType> for ManyResultDistributionType {
    fn from(value: &ManyResultDistributionType) -> Self {
        value.clone()
    }
}
impl From<ResultDistributionType> for ManyResultDistributionType {
    fn from(value: ResultDistributionType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<ResultDistributionType>> for ManyResultDistributionType {
    fn from(value: Vec<ResultDistributionType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyShaclValidator2017Type
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/ShaclValidator2017Type"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ShaclValidator2017Type"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyShaclValidator2017Type {
    Variant0(ShaclValidator2017Type),
    Variant1(Vec<ShaclValidator2017Type>),
}
impl From<&ManyShaclValidator2017Type> for ManyShaclValidator2017Type {
    fn from(value: &ManyShaclValidator2017Type) -> Self {
        value.clone()
    }
}
impl From<ShaclValidator2017Type> for ManyShaclValidator2017Type {
    fn from(value: ShaclValidator2017Type) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<ShaclValidator2017Type>> for ManyShaclValidator2017Type {
    fn from(value: Vec<ShaclValidator2017Type>) -> Self {
        Self::Variant1(value)
    }
}
///ManyShortenedGradingType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/ShortenedGradingType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ShortenedGradingType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyShortenedGradingType {
    Variant0(ShortenedGradingType),
    Variant1(Vec<ShortenedGradingType>),
}
impl From<&ManyShortenedGradingType> for ManyShortenedGradingType {
    fn from(value: &ManyShortenedGradingType) -> Self {
        value.clone()
    }
}
impl From<ShortenedGradingType> for ManyShortenedGradingType {
    fn from(value: ShortenedGradingType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<ShortenedGradingType>> for ManyShortenedGradingType {
    fn from(value: Vec<ShortenedGradingType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyStringType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/StringType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/StringType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyStringType {
    Variant0(StringType),
    Variant1(Vec<StringType>),
}
impl From<&ManyStringType> for ManyStringType {
    fn from(value: &ManyStringType) -> Self {
        value.clone()
    }
}
impl From<StringType> for ManyStringType {
    fn from(value: StringType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<StringType>> for ManyStringType {
    fn from(value: Vec<StringType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyTermsOfUseType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/TermsOfUseType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/TermsOfUseType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyTermsOfUseType {
    Variant0(TermsOfUseType),
    Variant1(Vec<TermsOfUseType>),
}
impl From<&ManyTermsOfUseType> for ManyTermsOfUseType {
    fn from(value: &ManyTermsOfUseType) -> Self {
        value.clone()
    }
}
impl From<TermsOfUseType> for ManyTermsOfUseType {
    fn from(value: TermsOfUseType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<TermsOfUseType>> for ManyTermsOfUseType {
    fn from(value: Vec<TermsOfUseType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyVerificationCheckType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/VerificationCheckType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/VerificationCheckType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyVerificationCheckType {
    Variant0(VerificationCheckType),
    Variant1(Vec<VerificationCheckType>),
}
impl From<&ManyVerificationCheckType> for ManyVerificationCheckType {
    fn from(value: &ManyVerificationCheckType) -> Self {
        value.clone()
    }
}
impl From<VerificationCheckType> for ManyVerificationCheckType {
    fn from(value: VerificationCheckType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<VerificationCheckType>> for ManyVerificationCheckType {
    fn from(value: Vec<VerificationCheckType>) -> Self {
        Self::Variant1(value)
    }
}
///ManyWebResourceType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/WebResourceType"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/WebResourceType"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ManyWebResourceType {
    Variant0(WebResourceType),
    Variant1(Vec<WebResourceType>),
}
impl From<&ManyWebResourceType> for ManyWebResourceType {
    fn from(value: &ManyWebResourceType) -> Self {
        value.clone()
    }
}
impl From<WebResourceType> for ManyWebResourceType {
    fn from(value: WebResourceType) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<WebResourceType>> for ManyWebResourceType {
    fn from(value: Vec<WebResourceType>) -> Self {
        Self::Variant1(value)
    }
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
pub struct MediaObjectType {
    #[serde(rename = "attachmentType", default, skip_serializing_if = "Option::is_none")]
    pub attachment_type: Option<ConceptType>,
    pub content: StringType,
    #[serde(rename = "contentEncoding")]
    pub content_encoding: ConceptType,
    #[serde(rename = "contentSize", default, skip_serializing_if = "Option::is_none")]
    pub content_size: Option<IntegerType>,
    #[serde(rename = "contentType")]
    pub content_type: ConceptType,
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
impl From<&MediaObjectType> for MediaObjectType {
    fn from(value: &MediaObjectType) -> Self {
        value.clone()
    }
}
impl MediaObjectType {
    pub fn builder() -> builder::MediaObjectType {
        Default::default()
    }
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
pub struct NoteType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "noteFormat", default, skip_serializing_if = "Option::is_none")]
    pub note_format: Option<ConceptType>,
    #[serde(rename = "noteLiteral")]
    pub note_literal: ManyLangStringType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<ConceptType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&NoteType> for NoteType {
    fn from(value: &NoteType) -> Self {
        value.clone()
    }
}
impl NoteType {
    pub fn builder() -> builder::NoteType {
        Default::default()
    }
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
pub struct OrganisationType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accreditation: Option<ManyAccreditationType>,
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ManyNoteType>,
    #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<ManyLangStringType>,
    #[serde(rename = "contactPoint", default, skip_serializing_if = "Option::is_none")]
    pub contact_point: Option<ManyContactPointType>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTimeType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<ConceptType>>,
    #[serde(
        rename = "eIDASIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub e_idas_identifier: Option<LegalIdentifierType>,
    #[serde(rename = "groupMemberOf", default, skip_serializing_if = "Option::is_none")]
    pub group_member_of: Option<ManyGroupType>,
    #[serde(rename = "hasMember", default, skip_serializing_if = "Option::is_none")]
    pub has_member: Option<ManyPersonType>,
    #[serde(rename = "hasSubOrganization", default)]
    pub has_sub_organization: Box<Option<ManyOrganisationType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ManyWebResourceType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifierType>,
    #[serde(rename = "legalName")]
    pub legal_name: ManyLangStringType,
    pub location: ManyLocationType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo: Option<MediaObjectType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration: Option<LegalIdentifierType>,
    #[serde(
        rename = "subOrganizationOf",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sub_organization_of: Option<Box<OrganisationType>>,
    #[serde(rename = "taxIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub tax_identifier: Option<ManyLegalIdentifierType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
    #[serde(rename = "vatIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub vat_identifier: Option<ManyLegalIdentifierType>,
}
impl From<&OrganisationType> for OrganisationType {
    fn from(value: &OrganisationType) -> Self {
        value.clone()
    }
}
impl OrganisationType {
    pub fn builder() -> builder::OrganisationType {
        Default::default()
    }
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
pub struct PercentageIntegerType(pub i64);
impl std::ops::Deref for PercentageIntegerType {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl From<PercentageIntegerType> for i64 {
    fn from(value: PercentageIntegerType) -> Self {
        value.0
    }
}
impl From<&PercentageIntegerType> for PercentageIntegerType {
    fn from(value: &PercentageIntegerType) -> Self {
        value.clone()
    }
}
impl From<i64> for PercentageIntegerType {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for PercentageIntegerType {
    type Err = <i64 as std::str::FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl std::convert::TryFrom<&str> for PercentageIntegerType {
    type Error = <i64 as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PercentageIntegerType {
    type Error = <i64 as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PercentageIntegerType {
    type Error = <i64 as std::str::FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl ToString for PercentageIntegerType {
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
pub struct PeriodOfTimeType {
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
impl From<&PeriodOfTimeType> for PeriodOfTimeType {
    fn from(value: &PeriodOfTimeType) -> Self {
        value.clone()
    }
}
impl PeriodOfTimeType {
    pub fn builder() -> builder::PeriodOfTimeType {
        Default::default()
    }
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
pub struct PersonType {
    #[serde(rename = "birthName", default, skip_serializing_if = "Option::is_none")]
    pub birth_name: Option<ManyLangStringType>,
    #[serde(
        rename = "citizenshipCountry",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub citizenship_country: Option<ObjectOrVector<ConceptType>>,
    #[serde(rename = "contactPoint", default, skip_serializing_if = "Option::is_none")]
    pub contact_point: Option<ManyContactPointType>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTimeType>,
    #[serde(rename = "dateOfBirth", default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<DateTimeType>,
    #[serde(rename = "familyName", default, skip_serializing_if = "Option::is_none")]
    pub family_name: Option<LangStringType>,
    #[serde(rename = "fullName", default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<LangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<ConceptType>,
    #[serde(rename = "givenName", default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<LangStringType>,
    #[serde(rename = "groupMemberOf", default, skip_serializing_if = "Option::is_none")]
    pub group_member_of: Option<ManyGroupType>,
    #[serde(rename = "hasClaim", default, skip_serializing_if = "Option::is_none")]
    pub has_claim: Option<ManyClaimNodeType>,
    #[serde(rename = "hasCredential", default, skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<ManyEuropeanDigitalCredentialType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifierType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationType>,
    #[serde(rename = "memberOf", default)]
    pub member_of: Box<Option<ManyOrganisationType>>,
    #[serde(rename = "nationalID", default, skip_serializing_if = "Option::is_none")]
    pub national_id: Option<LegalIdentifierType>,
    #[serde(rename = "patronymicName", default, skip_serializing_if = "Option::is_none")]
    pub patronymic_name: Option<ManyLangStringType>,
    #[serde(rename = "placeOfBirth", default, skip_serializing_if = "Option::is_none")]
    pub place_of_birth: Option<LocationType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&PersonType> for PersonType {
    fn from(value: &PersonType) -> Self {
        value.clone()
    }
}
impl PersonType {
    pub fn builder() -> builder::PersonType {
        Default::default()
    }
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
pub struct PhoneType {
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
impl From<&PhoneType> for PhoneType {
    fn from(value: &PhoneType) -> Self {
        value.clone()
    }
}
impl PhoneType {
    pub fn builder() -> builder::PhoneType {
        Default::default()
    }
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
pub struct PriceDetailType {
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ManyNoteType>,
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
impl From<&PriceDetailType> for PriceDetailType {
    fn from(value: &PriceDetailType) -> Self {
        value.clone()
    }
}
impl PriceDetailType {
    pub fn builder() -> builder::PriceDetailType {
        Default::default()
    }
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
pub struct ProofType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&ProofType> for ProofType {
    fn from(value: &ProofType) -> Self {
        value.clone()
    }
}
impl ProofType {
    pub fn builder() -> builder::ProofType {
        Default::default()
    }
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
pub struct QualificationType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accreditation: Option<ManyAccreditationType>,
    #[serde(rename = "additionalNote", default, skip_serializing_if = "Option::is_none")]
    pub additional_note: Option<ManyNoteType>,
    #[serde(rename = "altLabel", default, skip_serializing_if = "Option::is_none")]
    pub alt_label: Option<ManyLangStringType>,
    #[serde(
        rename = "awardingOpportunity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub awarding_opportunity: Option<ManyAwardingOpportunityType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<ManyLangStringType>,
    #[serde(rename = "creditPoint", default, skip_serializing_if = "Option::is_none")]
    pub credit_point: Option<ManyCreditPointType>,
    #[serde(rename = "dateModified", default, skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<DateTimeType>,
    #[serde(rename = "dcType", default, skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<ObjectOrVector<ConceptType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(rename = "educationLevel", default, skip_serializing_if = "Option::is_none")]
    pub education_level: Option<ObjectOrVector<ConceptType>>,
    #[serde(
        rename = "educationSubject",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub education_subject: Option<ObjectOrVector<ConceptType>>,
    #[serde(rename = "entitlesTo", default, skip_serializing_if = "Option::is_none")]
    pub entitles_to: Option<ManyLearningEntitlementSpecificationType>,
    #[serde(
        rename = "entryRequirement",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub entry_requirement: Option<NoteType>,
    #[serde(rename = "eqfLevel", default, skip_serializing_if = "Option::is_none")]
    pub eqf_level: Option<ConceptType>,
    #[serde(
        rename = "generalisationOf",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub generalisation_of: Option<ManyQualificationType>,
    #[serde(rename = "hasPart", default, skip_serializing_if = "Option::is_none")]
    pub has_part: Option<ManyQualificationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ManyWebResourceType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<IdentifierOrLegalIdentifierType>,
    #[serde(rename = "influencedBy", default, skip_serializing_if = "Option::is_none")]
    pub influenced_by: Option<ManyLearningActivitySpecificationType>,
    #[serde(rename = "isPartOf", default, skip_serializing_if = "Option::is_none")]
    pub is_part_of: Option<ManyQualificationType>,
    #[serde(
        rename = "isPartialQualification",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_partial_qualification: Option<BooleanType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<ObjectOrVector<ConceptType>>,
    #[serde(
        rename = "learningOutcome",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub learning_outcome: Option<ManyLearningOutcomeType>,
    #[serde(
        rename = "learningOutcomeSummary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub learning_outcome_summary: Option<NoteType>,
    #[serde(
        rename = "learningSetting",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub learning_setting: Option<ConceptType>,
    #[serde(
        rename = "maximumDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_duration: Option<DurationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ObjectOrVector<ConceptType>>,
    #[serde(rename = "nqfLevel", default, skip_serializing_if = "Option::is_none")]
    pub nqf_level: Option<ObjectOrVector<ConceptType>>,
    #[serde(rename = "provenBy", default)]
    pub proven_by: Box<Option<ManyLearningAssessmentSpecificationType>>,
    #[serde(
        rename = "qualificationCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub qualification_code: Option<ObjectOrVector<ConceptType>>,
    #[serde(
        rename = "specialisationOf",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub specialisation_of: Option<ManyQualificationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StringType>,
    #[serde(
        rename = "supplementaryDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_document: Option<ManyWebResourceType>,
    #[serde(rename = "targetGroup", default, skip_serializing_if = "Option::is_none")]
    pub target_group: Option<ObjectOrVector<ConceptType>>,
    #[serde(rename = "thematicArea", default, skip_serializing_if = "Option::is_none")]
    pub thematic_area: Option<ObjectOrVector<ConceptType>>,
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
impl From<&QualificationType> for QualificationType {
    fn from(value: &QualificationType) -> Self {
        value.clone()
    }
}
impl QualificationType {
    pub fn builder() -> builder::QualificationType {
        Default::default()
    }
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
pub struct ResultCategoryType {
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
impl From<&ResultCategoryType> for ResultCategoryType {
    fn from(value: &ResultCategoryType) -> Self {
        value.clone()
    }
}
impl ResultCategoryType {
    pub fn builder() -> builder::ResultCategoryType {
        Default::default()
    }
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
pub struct ResultDistributionType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "resultCategory", default, skip_serializing_if = "Option::is_none")]
    pub result_category: Option<ManyResultCategoryType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&ResultDistributionType> for ResultDistributionType {
    fn from(value: &ResultDistributionType) -> Self {
        value.clone()
    }
}
impl ResultDistributionType {
    pub fn builder() -> builder::ResultDistributionType {
        Default::default()
    }
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
pub struct ShaclValidator2017Type {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&ShaclValidator2017Type> for ShaclValidator2017Type {
    fn from(value: &ShaclValidator2017Type) -> Self {
        value.clone()
    }
}
impl ShaclValidator2017Type {
    pub fn builder() -> builder::ShaclValidator2017Type {
        Default::default()
    }
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
pub struct ShortenedGradingType {
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
impl From<&ShortenedGradingType> for ShortenedGradingType {
    fn from(value: &ShortenedGradingType) -> Self {
        value.clone()
    }
}
impl ShortenedGradingType {
    pub fn builder() -> builder::ShortenedGradingType {
        Default::default()
    }
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
impl From<&TermsOfUse> for TermsOfUse {
    fn from(value: &TermsOfUse) -> Self {
        value.clone()
    }
}
impl TermsOfUse {
    pub fn builder() -> builder::TermsOfUse {
        Default::default()
    }
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
impl From<&TermsOfUseType> for TermsOfUseType {
    fn from(value: &TermsOfUseType) -> Self {
        value.clone()
    }
}
impl TermsOfUseType {
    pub fn builder() -> builder::TermsOfUseType {
        Default::default()
    }
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
pub struct VerificationCheckType {
    #[serde(rename = "dcType")]
    pub dc_type: ConceptType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ManyLangStringType>,
    #[serde(rename = "elmSubject", default, skip_serializing_if = "Option::is_none")]
    pub elm_subject: Option<EuropeanDigitalCredentialType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    pub subject: serde_json::Value,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
    #[serde(rename = "verificationStatus")]
    pub verification_status: ConceptType,
}
impl From<&VerificationCheckType> for VerificationCheckType {
    fn from(value: &VerificationCheckType) -> Self {
        value.clone()
    }
}
impl VerificationCheckType {
    pub fn builder() -> builder::VerificationCheckType {
        Default::default()
    }
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
pub struct WebResourceType {
    #[serde(rename = "contentURL")]
    pub content_url: UriType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<ConceptType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<ManyLangStringType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl From<&WebResourceType> for WebResourceType {
    fn from(value: &WebResourceType) -> Self {
        value.clone()
    }
}
impl WebResourceType {
    pub fn builder() -> builder::WebResourceType {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    use types_common::ObjectOrVector;

    #[derive(Clone, Debug)]
    pub struct AccreditationType {
        accrediting_agent: Result<super::OrganisationType, String>,
        additional_note: Result<Option<super::ManyNoteType>, String>,
        date_issued: Result<Option<super::DateTimeType>, String>,
        date_modified: Result<Option<super::DateTimeType>, String>,
        dc_type: Result<super::ConceptType, String>,
        decision: Result<Option<super::ConceptType>, String>,
        description: Result<Option<super::ManyLangStringType>, String>,
        expiry_date: Result<Option<super::DateTimeType>, String>,
        homepage: Result<Option<super::ManyWebResourceType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        identifier: Result<Option<super::IdentifierOrLegalIdentifierType>, String>,
        landing_page: Result<Option<super::ManyWebResourceType>, String>,
        limit_credential_type: Result<Option<ObjectOrVector<ConceptType>>, String>,
        limit_eqf_level: Result<Option<ObjectOrVector<ConceptType>>, String>,
        limit_field: Result<Option<ObjectOrVector<ConceptType>>, String>,
        limit_jurisdiction: Result<Option<ObjectOrVector<ConceptType>>, String>,
        limit_qualification: Result<Option<super::QualificationType>, String>,
        organisation: Result<Option<super::ManyOrganisationType>, String>,
        report: Result<Option<super::WebResourceType>, String>,
        review_date: Result<Option<super::DateTimeType>, String>,
        status: Result<Option<super::StringType>, String>,
        supplementary_document: Result<Option<super::ManyWebResourceType>, String>,
        title: Result<super::ManyLangStringType, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for AccreditationType {
        fn default() -> Self {
            Self {
                accrediting_agent: Err(
                    "no value supplied for accrediting_agent".to_string(),
                ),
                additional_note: Ok(Default::default()),
                date_issued: Ok(Default::default()),
                date_modified: Ok(Default::default()),
                dc_type: Err("no value supplied for dc_type".to_string()),
                decision: Ok(Default::default()),
                description: Ok(Default::default()),
                expiry_date: Ok(Default::default()),
                homepage: Ok(Default::default()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                landing_page: Ok(Default::default()),
                limit_credential_type: Ok(Default::default()),
                limit_eqf_level: Ok(Default::default()),
                limit_field: Ok(Default::default()),
                limit_jurisdiction: Ok(Default::default()),
                limit_qualification: Ok(Default::default()),
                organisation: Ok(Default::default()),
                report: Ok(Default::default()),
                review_date: Ok(Default::default()),
                status: Ok(Default::default()),
                supplementary_document: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl AccreditationType {
        pub fn accrediting_agent<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::OrganisationType>,
            T::Error: std::fmt::Display,
        {
            self.accrediting_agent = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for accrediting_agent: {}", e
                    )
                });
            self
        }
        pub fn additional_note<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyNoteType>>,
            T::Error: std::fmt::Display,
        {
            self.additional_note = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for additional_note: {}", e)
                });
            self
        }
        pub fn date_issued<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.date_issued = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for date_issued: {}", e)
                });
            self
        }
        pub fn date_modified<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.date_modified = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for date_modified: {}", e)
                });
            self
        }
        pub fn dc_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ConceptType>,
            T::Error: std::fmt::Display,
        {
            self.dc_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dc_type: {}", e)
                });
            self
        }
        pub fn decision<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ConceptType>>,
            T::Error: std::fmt::Display,
        {
            self.decision = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for decision: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn expiry_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.expiry_date = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for expiry_date: {}", e)
                });
            self
        }
        pub fn homepage<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.homepage = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for homepage: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn landing_page<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.landing_page = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for landing_page: {}", e)
                });
            self
        }
        pub fn limit_credential_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.limit_credential_type = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for limit_credential_type: {}",
                        e
                    )
                });
            self
        }
        pub fn limit_eqf_level<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.limit_eqf_level = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for limit_eqf_level: {}", e)
                });
            self
        }
        pub fn limit_field<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.limit_field = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for limit_field: {}", e)
                });
            self
        }
        pub fn limit_jurisdiction<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.limit_jurisdiction = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for limit_jurisdiction: {}", e
                    )
                });
            self
        }
        pub fn limit_qualification<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::QualificationType>>,
            T::Error: std::fmt::Display,
        {
            self.limit_qualification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for limit_qualification: {}", e
                    )
                });
            self
        }
        pub fn organisation<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyOrganisationType>>,
            T::Error: std::fmt::Display,
        {
            self.organisation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for organisation: {}", e)
                });
            self
        }
        pub fn report<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::WebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.report = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for report: {}", e)
                });
            self
        }
        pub fn review_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.review_date = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for review_date: {}", e)
                });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringType>>,
            T::Error: std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status: {}", e)
                });
            self
        }
        pub fn supplementary_document<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.supplementary_document = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for supplementary_document: {}",
                        e
                    )
                });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyLangStringType>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for title: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<AccreditationType> for super::AccreditationType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AccreditationType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                accrediting_agent: value.accrediting_agent?,
                additional_note: value.additional_note?,
                date_issued: value.date_issued?,
                date_modified: value.date_modified?,
                dc_type: value.dc_type?,
                decision: value.decision?,
                description: value.description?,
                expiry_date: value.expiry_date?,
                homepage: value.homepage?,
                id: value.id?,
                identifier: value.identifier?,
                landing_page: value.landing_page?,
                limit_credential_type: value.limit_credential_type?,
                limit_eqf_level: value.limit_eqf_level?,
                limit_field: value.limit_field?,
                limit_jurisdiction: value.limit_jurisdiction?,
                limit_qualification: value.limit_qualification?,
                organisation: value.organisation?,
                report: value.report?,
                review_date: value.review_date?,
                status: value.status?,
                supplementary_document: value.supplementary_document?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::AccreditationType> for AccreditationType {
        fn from(value: super::AccreditationType) -> Self {
            Self {
                accrediting_agent: Ok(value.accrediting_agent),
                additional_note: Ok(value.additional_note),
                date_issued: Ok(value.date_issued),
                date_modified: Ok(value.date_modified),
                dc_type: Ok(value.dc_type),
                decision: Ok(value.decision),
                description: Ok(value.description),
                expiry_date: Ok(value.expiry_date),
                homepage: Ok(value.homepage),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                landing_page: Ok(value.landing_page),
                limit_credential_type: Ok(value.limit_credential_type),
                limit_eqf_level: Ok(value.limit_eqf_level),
                limit_field: Ok(value.limit_field),
                limit_jurisdiction: Ok(value.limit_jurisdiction),
                limit_qualification: Ok(value.limit_qualification),
                organisation: Ok(value.organisation),
                report: Ok(value.report),
                review_date: Ok(value.review_date),
                status: Ok(value.status),
                supplementary_document: Ok(value.supplementary_document),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AddressType {
        country_code: Result<super::ConceptType, String>,
        full_address: Result<Option<super::NoteType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        identifier: Result<Option<super::IdentifierOrLegalIdentifierType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for AddressType {
        fn default() -> Self {
            Self {
                country_code: Err("no value supplied for country_code".to_string()),
                full_address: Ok(Default::default()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl AddressType {
        pub fn country_code<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ConceptType>,
            T::Error: std::fmt::Display,
        {
            self.country_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for country_code: {}", e)
                });
            self
        }
        pub fn full_address<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::NoteType>>,
            T::Error: std::fmt::Display,
        {
            self.full_address = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for full_address: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<AddressType> for super::AddressType {
        type Error = super::error::ConversionError;
        fn try_from(value: AddressType) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                country_code: value.country_code?,
                full_address: value.full_address?,
                id: value.id?,
                identifier: value.identifier?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::AddressType> for AddressType {
        fn from(value: super::AddressType) -> Self {
            Self {
                country_code: Ok(value.country_code),
                full_address: Ok(value.full_address),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentOrPersonOrOrganisationType {
        subtype_0: Result<Option<super::AgentType>, String>,
        subtype_1: Result<Option<Box<super::PersonType>>, String>,
        subtype_2: Result<Option<Box<super::OrganisationType>>, String>,
    }
    impl Default for AgentOrPersonOrOrganisationType {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
            }
        }
    }
    impl AgentOrPersonOrOrganisationType {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AgentType>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_0: {}", e)
                });
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Box<super::PersonType>>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_1: {}", e)
                });
            self
        }
        pub fn subtype_2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Box<super::OrganisationType>>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_2 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_2: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<AgentOrPersonOrOrganisationType>
    for super::AgentOrPersonOrOrganisationType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentOrPersonOrOrganisationType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
            })
        }
    }
    impl From<super::AgentOrPersonOrOrganisationType>
    for AgentOrPersonOrOrganisationType {
        fn from(value: super::AgentOrPersonOrOrganisationType) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
                subtype_2: Ok(value.subtype_2),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentType {
        additional_note: Result<Option<super::ManyNoteType>, String>,
        alt_label: Result<Option<super::ManyLangStringType>, String>,
        contact_point: Result<Option<super::ManyContactPointType>, String>,
        date_modified: Result<Option<super::DateTimeType>, String>,
        group_member_of: Result<Option<super::ManyGroupType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        identifier: Result<Option<super::ManyIdentifierOrLegalIdentifierType>, String>,
        location: Result<Option<super::ManyLocationType>, String>,
        pref_label: Result<Option<super::ManyLangStringType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for AgentType {
        fn default() -> Self {
            Self {
                additional_note: Ok(Default::default()),
                alt_label: Ok(Default::default()),
                contact_point: Ok(Default::default()),
                date_modified: Ok(Default::default()),
                group_member_of: Ok(Default::default()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                location: Ok(Default::default()),
                pref_label: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl AgentType {
        pub fn additional_note<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyNoteType>>,
            T::Error: std::fmt::Display,
        {
            self.additional_note = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for additional_note: {}", e)
                });
            self
        }
        pub fn alt_label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.alt_label = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for alt_label: {}", e)
                });
            self
        }
        pub fn contact_point<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyContactPointType>>,
            T::Error: std::fmt::Display,
        {
            self.contact_point = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for contact_point: {}", e)
                });
            self
        }
        pub fn date_modified<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.date_modified = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for date_modified: {}", e)
                });
            self
        }
        pub fn group_member_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyGroupType>>,
            T::Error: std::fmt::Display,
        {
            self.group_member_of = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for group_member_of: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyIdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLocationType>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for location: {}", e)
                });
            self
        }
        pub fn pref_label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.pref_label = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for pref_label: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<AgentType> for super::AgentType {
        type Error = super::error::ConversionError;
        fn try_from(value: AgentType) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_note: value.additional_note?,
                alt_label: value.alt_label?,
                contact_point: value.contact_point?,
                date_modified: value.date_modified?,
                group_member_of: value.group_member_of?,
                id: value.id?,
                identifier: value.identifier?,
                location: value.location?,
                pref_label: value.pref_label?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::AgentType> for AgentType {
        fn from(value: super::AgentType) -> Self {
            Self {
                additional_note: Ok(value.additional_note),
                alt_label: Ok(value.alt_label),
                contact_point: Ok(value.contact_point),
                date_modified: Ok(value.date_modified),
                group_member_of: Ok(value.group_member_of),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                location: Ok(value.location),
                pref_label: Ok(value.pref_label),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AmountType {
        id: Result<Option<super::GenericIdType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
        unit: Result<super::ConceptType, String>,
        value: Result<super::DecimalType, String>,
    }
    impl Default for AmountType {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                type_: Ok(Default::default()),
                unit: Err("no value supplied for unit".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl AmountType {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn unit<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ConceptType>,
            T::Error: std::fmt::Display,
        {
            self.unit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for unit: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DecimalType>,
            T::Error: std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<AmountType> for super::AmountType {
        type Error = super::error::ConversionError;
        fn try_from(value: AmountType) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                type_: value.type_?,
                unit: value.unit?,
                value: value.value?,
            })
        }
    }
    impl From<super::AmountType> for AmountType {
        fn from(value: super::AmountType) -> Self {
            Self {
                id: Ok(value.id),
                type_: Ok(value.type_),
                unit: Ok(value.unit),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AwardingOpportunityType {
        awarding_body: Result<super::ManyAgentOrPersonOrOrganisationType, String>,
        id: Result<Option<super::GenericIdType>, String>,
        identifier: Result<Option<super::ManyIdentifierOrLegalIdentifierType>, String>,
        learning_achievement_specification: Result<
            Box<Option<super::LearningAchievementSpecificationOrQualificationType>>,
            String,
        >,
        location: Result<Option<super::LocationType>, String>,
        temporal: Result<Option<super::PeriodOfTimeType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for AwardingOpportunityType {
        fn default() -> Self {
            Self {
                awarding_body: Err("no value supplied for awarding_body".to_string()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                learning_achievement_specification: Ok(Default::default()),
                location: Ok(Default::default()),
                temporal: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl AwardingOpportunityType {
        pub fn awarding_body<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyAgentOrPersonOrOrganisationType>,
            T::Error: std::fmt::Display,
        {
            self.awarding_body = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for awarding_body: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyIdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn learning_achievement_specification<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Box<Option<super::LearningAchievementSpecificationOrQualificationType>>,
            >,
            T::Error: std::fmt::Display,
        {
            self.learning_achievement_specification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for learning_achievement_specification: {}",
                        e
                    )
                });
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LocationType>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for location: {}", e)
                });
            self
        }
        pub fn temporal<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::PeriodOfTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.temporal = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for temporal: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<AwardingOpportunityType>
    for super::AwardingOpportunityType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AwardingOpportunityType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                awarding_body: value.awarding_body?,
                id: value.id?,
                identifier: value.identifier?,
                learning_achievement_specification: value
                    .learning_achievement_specification?,
                location: value.location?,
                temporal: value.temporal?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::AwardingOpportunityType> for AwardingOpportunityType {
        fn from(value: super::AwardingOpportunityType) -> Self {
            Self {
                awarding_body: Ok(value.awarding_body),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                learning_achievement_specification: Ok(
                    value.learning_achievement_specification,
                ),
                location: Ok(value.location),
                temporal: Ok(value.temporal),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AwardingProcessType {
        additional_note: Result<Option<super::ManyNoteType>, String>,
        awarding_body: Result<super::ManyAgentOrPersonOrOrganisationType, String>,
        awarding_date: Result<Option<super::DateTimeType>, String>,
        awards: Result<Box<Option<super::ManyClaimNodeType>>, String>,
        description: Result<Option<super::ManyLangStringType>, String>,
        educational_system_note: Result<Option<super::ConceptType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        identifier: Result<Option<super::ManyIdentifierOrLegalIdentifierType>, String>,
        location: Result<Option<super::LocationType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
        used: Result<Option<super::ManyLearningAssessmentType>, String>,
    }
    impl Default for AwardingProcessType {
        fn default() -> Self {
            Self {
                additional_note: Ok(Default::default()),
                awarding_body: Err("no value supplied for awarding_body".to_string()),
                awarding_date: Ok(Default::default()),
                awards: Ok(Default::default()),
                description: Ok(Default::default()),
                educational_system_note: Ok(Default::default()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                location: Ok(Default::default()),
                type_: Ok(Default::default()),
                used: Ok(Default::default()),
            }
        }
    }
    impl AwardingProcessType {
        pub fn additional_note<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyNoteType>>,
            T::Error: std::fmt::Display,
        {
            self.additional_note = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for additional_note: {}", e)
                });
            self
        }
        pub fn awarding_body<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyAgentOrPersonOrOrganisationType>,
            T::Error: std::fmt::Display,
        {
            self.awarding_body = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for awarding_body: {}", e)
                });
            self
        }
        pub fn awarding_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.awarding_date = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for awarding_date: {}", e)
                });
            self
        }
        pub fn awards<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<Option<super::ManyClaimNodeType>>>,
            T::Error: std::fmt::Display,
        {
            self.awards = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for awards: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn educational_system_note<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ConceptType>>,
            T::Error: std::fmt::Display,
        {
            self.educational_system_note = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for educational_system_note: {}",
                        e
                    )
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyIdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LocationType>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for location: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn used<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLearningAssessmentType>>,
            T::Error: std::fmt::Display,
        {
            self.used = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for used: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AwardingProcessType> for super::AwardingProcessType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AwardingProcessType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_note: value.additional_note?,
                awarding_body: value.awarding_body?,
                awarding_date: value.awarding_date?,
                awards: value.awards?,
                description: value.description?,
                educational_system_note: value.educational_system_note?,
                id: value.id?,
                identifier: value.identifier?,
                location: value.location?,
                type_: value.type_?,
                used: value.used?,
            })
        }
    }
    impl From<super::AwardingProcessType> for AwardingProcessType {
        fn from(value: super::AwardingProcessType) -> Self {
            Self {
                additional_note: Ok(value.additional_note),
                awarding_body: Ok(value.awarding_body),
                awarding_date: Ok(value.awarding_date),
                awards: Ok(value.awards),
                description: Ok(value.description),
                educational_system_note: Ok(value.educational_system_note),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                location: Ok(value.location),
                type_: Ok(value.type_),
                used: Ok(value.used),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClaimNodeType {
        subtype_0: Result<Option<super::LearningAchievementType>, String>,
        subtype_1: Result<Option<super::LearningActivityType>, String>,
        subtype_2: Result<Option<super::LearningAssessmentType>, String>,
        subtype_3: Result<Option<super::LearningEntitlementType>, String>,
        subtype_4: Result<Option<super::ClaimTypeNodeType>, String>,
    }
    impl Default for ClaimNodeType {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
                subtype_3: Ok(Default::default()),
                subtype_4: Ok(Default::default()),
            }
        }
    }
    impl ClaimNodeType {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LearningAchievementType>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_0: {}", e)
                });
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LearningActivityType>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_1: {}", e)
                });
            self
        }
        pub fn subtype_2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LearningAssessmentType>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_2 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_2: {}", e)
                });
            self
        }
        pub fn subtype_3<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LearningEntitlementType>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_3 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_3: {}", e)
                });
            self
        }
        pub fn subtype_4<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ClaimTypeNodeType>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_4 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_4: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<ClaimNodeType> for super::ClaimNodeType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClaimNodeType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
                subtype_3: value.subtype_3?,
                subtype_4: value.subtype_4?,
            })
        }
    }
    impl From<super::ClaimNodeType> for ClaimNodeType {
        fn from(value: super::ClaimNodeType) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
                subtype_2: Ok(value.subtype_2),
                subtype_3: Ok(value.subtype_3),
                subtype_4: Ok(value.subtype_4),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClaimTypeNodeType {
        additional_note: Result<Option<super::ManyNoteType>, String>,
        awarded_by: Result<super::AwardingProcessType, String>,
        description: Result<Option<super::ManyLangStringType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        identifier: Result<Option<super::ManyIdentifierOrLegalIdentifierType>, String>,
        supplementary_document: Result<Option<super::ManyWebResourceType>, String>,
        title: Result<super::ManyLangStringType, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for ClaimTypeNodeType {
        fn default() -> Self {
            Self {
                additional_note: Ok(Default::default()),
                awarded_by: Err("no value supplied for awarded_by".to_string()),
                description: Ok(Default::default()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                supplementary_document: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl ClaimTypeNodeType {
        pub fn additional_note<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyNoteType>>,
            T::Error: std::fmt::Display,
        {
            self.additional_note = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for additional_note: {}", e)
                });
            self
        }
        pub fn awarded_by<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AwardingProcessType>,
            T::Error: std::fmt::Display,
        {
            self.awarded_by = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for awarded_by: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyIdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn supplementary_document<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.supplementary_document = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for supplementary_document: {}",
                        e
                    )
                });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyLangStringType>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for title: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<ClaimTypeNodeType> for super::ClaimTypeNodeType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClaimTypeNodeType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_note: value.additional_note?,
                awarded_by: value.awarded_by?,
                description: value.description?,
                id: value.id?,
                identifier: value.identifier?,
                supplementary_document: value.supplementary_document?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::ClaimTypeNodeType> for ClaimTypeNodeType {
        fn from(value: super::ClaimTypeNodeType) -> Self {
            Self {
                additional_note: Ok(value.additional_note),
                awarded_by: Ok(value.awarded_by),
                description: Ok(value.description),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                supplementary_document: Ok(value.supplementary_document),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConceptSchemeType {
        id: Result<Option<super::GenericIdType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for ConceptSchemeType {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl ConceptSchemeType {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<ConceptSchemeType> for super::ConceptSchemeType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConceptSchemeType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::ConceptSchemeType> for ConceptSchemeType {
        fn from(value: super::ConceptSchemeType) -> Self {
            Self {
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConceptType {
        definition: Result<Option<super::ManyLangStringType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        in_scheme: Result<Option<super::ConceptSchemeType>, String>,
        notation: Result<Option<super::LiteralType>, String>,
        pref_label: Result<Option<super::ManyLangStringType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for ConceptType {
        fn default() -> Self {
            Self {
                definition: Ok(Default::default()),
                id: Ok(Default::default()),
                in_scheme: Ok(Default::default()),
                notation: Ok(Default::default()),
                pref_label: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl ConceptType {
        pub fn definition<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.definition = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for definition: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn in_scheme<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ConceptSchemeType>>,
            T::Error: std::fmt::Display,
        {
            self.in_scheme = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for in_scheme: {}", e)
                });
            self
        }
        pub fn notation<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LiteralType>>,
            T::Error: std::fmt::Display,
        {
            self.notation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for notation: {}", e)
                });
            self
        }
        pub fn pref_label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.pref_label = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for pref_label: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<ConceptType> for super::ConceptType {
        type Error = super::error::ConversionError;
        fn try_from(value: ConceptType) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                definition: value.definition?,
                id: value.id?,
                in_scheme: value.in_scheme?,
                notation: value.notation?,
                pref_label: value.pref_label?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::ConceptType> for ConceptType {
        fn from(value: super::ConceptType) -> Self {
            Self {
                definition: Ok(value.definition),
                id: Ok(value.id),
                in_scheme: Ok(value.in_scheme),
                notation: Ok(value.notation),
                pref_label: Ok(value.pref_label),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ContactPointType {
        additional_note: Result<Option<super::ManyNoteType>, String>,
        address: Result<Option<super::ManyAddressType>, String>,
        contact_form: Result<Option<super::ManyWebResourceType>, String>,
        description: Result<Option<super::ManyLangStringType>, String>,
        email_address: Result<Option<super::ManyMailboxType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        phone: Result<Option<super::ManyPhoneType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for ContactPointType {
        fn default() -> Self {
            Self {
                additional_note: Ok(Default::default()),
                address: Ok(Default::default()),
                contact_form: Ok(Default::default()),
                description: Ok(Default::default()),
                email_address: Ok(Default::default()),
                id: Ok(Default::default()),
                phone: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl ContactPointType {
        pub fn additional_note<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyNoteType>>,
            T::Error: std::fmt::Display,
        {
            self.additional_note = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for additional_note: {}", e)
                });
            self
        }
        pub fn address<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyAddressType>>,
            T::Error: std::fmt::Display,
        {
            self.address = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for address: {}", e)
                });
            self
        }
        pub fn contact_form<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.contact_form = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for contact_form: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn email_address<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyMailboxType>>,
            T::Error: std::fmt::Display,
        {
            self.email_address = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for email_address: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn phone<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyPhoneType>>,
            T::Error: std::fmt::Display,
        {
            self.phone = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for phone: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<ContactPointType> for super::ContactPointType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ContactPointType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_note: value.additional_note?,
                address: value.address?,
                contact_form: value.contact_form?,
                description: value.description?,
                email_address: value.email_address?,
                id: value.id?,
                phone: value.phone?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::ContactPointType> for ContactPointType {
        fn from(value: super::ContactPointType) -> Self {
            Self {
                additional_note: Ok(value.additional_note),
                address: Ok(value.address),
                contact_form: Ok(value.contact_form),
                description: Ok(value.description),
                email_address: Ok(value.email_address),
                id: Ok(value.id),
                phone: Ok(value.phone),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CredentialSchema {
        id: Result<String, String>,
        type_: Result<super::CredentialSchemaType, String>,
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
            T: std::convert::TryInto<super::CredentialSchemaType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<CredentialSchema> for super::CredentialSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CredentialSchema,
        ) -> Result<Self, super::error::ConversionError> {
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
    pub struct CredentialStatusType {
        id: Result<Option<super::GenericIdType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for CredentialStatusType {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl CredentialStatusType {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<CredentialStatusType> for super::CredentialStatusType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CredentialStatusType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::CredentialStatusType> for CredentialStatusType {
        fn from(value: super::CredentialStatusType) -> Self {
            Self {
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CredentialSubject {
        id: Result<Option<String>, String>,
    }
    impl Default for CredentialSubject {
        fn default() -> Self {
            Self { id: Ok(Default::default()) }
        }
    }
    impl CredentialSubject {
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
    }
    impl std::convert::TryFrom<CredentialSubject> for super::CredentialSubject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CredentialSubject,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl From<super::CredentialSubject> for CredentialSubject {
        fn from(value: super::CredentialSubject) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreditPointType {
        framework: Result<super::ConceptType, String>,
        id: Result<Option<super::GenericIdType>, String>,
        point: Result<super::StringType, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for CreditPointType {
        fn default() -> Self {
            Self {
                framework: Err("no value supplied for framework".to_string()),
                id: Ok(Default::default()),
                point: Err("no value supplied for point".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl CreditPointType {
        pub fn framework<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ConceptType>,
            T::Error: std::fmt::Display,
        {
            self.framework = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for framework: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn point<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::StringType>,
            T::Error: std::fmt::Display,
        {
            self.point = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for point: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<CreditPointType> for super::CreditPointType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreditPointType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                framework: value.framework?,
                id: value.id?,
                point: value.point?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::CreditPointType> for CreditPointType {
        fn from(value: super::CreditPointType) -> Self {
            Self {
                framework: Ok(value.framework),
                id: Ok(value.id),
                point: Ok(value.point),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DisplayDetailType {
        id: Result<Option<super::GenericIdType>, String>,
        image: Result<super::MediaObjectType, String>,
        page: Result<super::PositiveIntegerType, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for DisplayDetailType {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                image: Err("no value supplied for image".to_string()),
                page: Err("no value supplied for page".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl DisplayDetailType {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn image<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::MediaObjectType>,
            T::Error: std::fmt::Display,
        {
            self.image = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for image: {}", e)
                });
            self
        }
        pub fn page<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::PositiveIntegerType>,
            T::Error: std::fmt::Display,
        {
            self.page = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<DisplayDetailType> for super::DisplayDetailType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DisplayDetailType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                image: value.image?,
                page: value.page?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::DisplayDetailType> for DisplayDetailType {
        fn from(value: super::DisplayDetailType) -> Self {
            Self {
                id: Ok(value.id),
                image: Ok(value.image),
                page: Ok(value.page),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DisplayParameterType {
        description: Result<Option<super::ManyLangStringType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        individual_display: Result<super::ManyIndividualDisplayType, String>,
        language: Result<ObjectOrVector<ConceptType>, String>,
        primary_language: Result<super::ConceptType, String>,
        summary_display: Result<Option<super::StringType>, String>,
        title: Result<super::ManyLangStringType, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for DisplayParameterType {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                id: Ok(Default::default()),
                individual_display: Err(
                    "no value supplied for individual_display".to_string(),
                ),
                language: Err("no value supplied for language".to_string()),
                primary_language: Err(
                    "no value supplied for primary_language".to_string(),
                ),
                summary_display: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl DisplayParameterType {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn individual_display<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyIndividualDisplayType>,
            T::Error: std::fmt::Display,
        {
            self.individual_display = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for individual_display: {}", e
                    )
                });
            self
        }
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<ObjectOrVector<ConceptType>>,
            T::Error: std::fmt::Display,
        {
            self.language = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for language: {}", e)
                });
            self
        }
        pub fn primary_language<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ConceptType>,
            T::Error: std::fmt::Display,
        {
            self.primary_language = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for primary_language: {}", e
                    )
                });
            self
        }
        pub fn summary_display<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringType>>,
            T::Error: std::fmt::Display,
        {
            self.summary_display = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for summary_display: {}", e)
                });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyLangStringType>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for title: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<DisplayParameterType> for super::DisplayParameterType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DisplayParameterType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                id: value.id?,
                individual_display: value.individual_display?,
                language: value.language?,
                primary_language: value.primary_language?,
                summary_display: value.summary_display?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::DisplayParameterType> for DisplayParameterType {
        fn from(value: super::DisplayParameterType) -> Self {
            Self {
                description: Ok(value.description),
                id: Ok(value.id),
                individual_display: Ok(value.individual_display),
                language: Ok(value.language),
                primary_language: Ok(value.primary_language),
                summary_display: Ok(value.summary_display),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EmailType {
        subtype_0: Result<Option<String>, String>,
        subtype_1: Result<Option<String>, String>,
    }
    impl Default for EmailType {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl EmailType {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_0: {}", e)
                });
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_1: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<EmailType> for super::EmailType {
        type Error = super::error::ConversionError;
        fn try_from(value: EmailType) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl From<super::EmailType> for EmailType {
        fn from(value: super::EmailType) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EuropassEdcCredential {
        context: Result<super::EuropassEdcCredentialContext, String>,
        credential_profiles: Result<Option<ObjectOrVector<ConceptType>>, String>,
        credential_schema: Result<ObjectOrVector<CredentialSchema>, String>,
        credential_status: Result<
            Option<super::EuropassEdcCredentialCredentialStatus>,
            String,
        >,
        credential_subject: Result<
            super::EuropassEdcCredentialCredentialSubject,
            String,
        >,
        display_parameter: Result<Option<super::DisplayParameterType>, String>,
        evidence: Result<Option<ObjectOrVector<Evidence>>, String>,
        id: Result<String, String>,
        issuer: Result<super::EuropassEdcCredentialIssuer, String>,
        terms_of_use: Result<Option<ObjectOrVector<TermsOfUse>>, String>,
        type_: Result<Vec<String>, String>,
        valid_from: Result<chrono::DateTime<chrono::offset::Utc>, String>,
        valid_until: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
    }
    impl Default for EuropassEdcCredential {
        fn default() -> Self {
            Self {
                context: Err("no value supplied for context".to_string()),
                credential_profiles: Ok(Default::default()),
                credential_schema: Err(
                    "no value supplied for credential_schema".to_string(),
                ),
                credential_status: Ok(Default::default()),
                credential_subject: Err(
                    "no value supplied for credential_subject".to_string(),
                ),
                display_parameter: Ok(Default::default()),
                evidence: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                issuer: Err("no value supplied for issuer".to_string()),
                terms_of_use: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                valid_from: Err("no value supplied for valid_from".to_string()),
                valid_until: Ok(Default::default()),
            }
        }
    }
    impl EuropassEdcCredential {
        pub fn context<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::EuropassEdcCredentialContext>,
            T::Error: std::fmt::Display,
        {
            self.context = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for context: {}", e)
                });
            self
        }
        pub fn credential_profiles<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.credential_profiles = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for credential_profiles: {}", e
                    )
                });
            self
        }
        pub fn credential_schema<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<ObjectOrVector<CredentialSchema>>,
            T::Error: std::fmt::Display,
        {
            self.credential_schema = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for credential_schema: {}", e
                    )
                });
            self
        }
        pub fn credential_status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::EuropassEdcCredentialCredentialStatus>,
            >,
            T::Error: std::fmt::Display,
        {
            self.credential_status = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for credential_status: {}", e
                    )
                });
            self
        }
        pub fn credential_subject<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::EuropassEdcCredentialCredentialSubject>,
            T::Error: std::fmt::Display,
        {
            self.credential_subject = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for credential_subject: {}", e
                    )
                });
            self
        }
        pub fn display_parameter<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DisplayParameterType>>,
            T::Error: std::fmt::Display,
        {
            self.display_parameter = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for display_parameter: {}", e
                    )
                });
            self
        }
        pub fn evidence<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<Evidence>>>,
            T::Error: std::fmt::Display,
        {
            self.evidence = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for evidence: {}", e)
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
        pub fn issuer<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::EuropassEdcCredentialIssuer>,
            T::Error: std::fmt::Display,
        {
            self.issuer = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for issuer: {}", e)
                });
            self
        }
        pub fn terms_of_use<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<TermsOfUse>>>,
            T::Error: std::fmt::Display,
        {
            self.terms_of_use = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for terms_of_use: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn valid_from<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
            T::Error: std::fmt::Display,
        {
            self.valid_from = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_from: {}", e)
                });
            self
        }
        pub fn valid_until<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self.valid_until = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_until: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<EuropassEdcCredential> for super::EuropassEdcCredential {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EuropassEdcCredential,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                context: value.context?,
                credential_profiles: value.credential_profiles?,
                credential_schema: value.credential_schema?,
                credential_status: value.credential_status?,
                credential_subject: value.credential_subject?,
                display_parameter: value.display_parameter?,
                evidence: value.evidence?,
                id: value.id?,
                issuer: value.issuer?,
                terms_of_use: value.terms_of_use?,
                type_: value.type_?,
                valid_from: value.valid_from?,
                valid_until: value.valid_until?,
            })
        }
    }
    impl From<super::EuropassEdcCredential> for EuropassEdcCredential {
        fn from(value: super::EuropassEdcCredential) -> Self {
            Self {
                context: Ok(value.context),
                credential_profiles: Ok(value.credential_profiles),
                credential_schema: Ok(value.credential_schema),
                credential_status: Ok(value.credential_status),
                credential_subject: Ok(value.credential_subject),
                display_parameter: Ok(value.display_parameter),
                evidence: Ok(value.evidence),
                id: Ok(value.id),
                issuer: Ok(value.issuer),
                terms_of_use: Ok(value.terms_of_use),
                type_: Ok(value.type_),
                valid_from: Ok(value.valid_from),
                valid_until: Ok(value.valid_until),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EuropassEdcCredentialCredentialStatus {
        id: Result<String, String>,
        type_: Result<String, String>,
    }
    impl Default for EuropassEdcCredentialCredentialStatus {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl EuropassEdcCredentialCredentialStatus {
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
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<EuropassEdcCredentialCredentialStatus>
    for super::EuropassEdcCredentialCredentialStatus {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EuropassEdcCredentialCredentialStatus,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::EuropassEdcCredentialCredentialStatus>
    for EuropassEdcCredentialCredentialStatus {
        fn from(value: super::EuropassEdcCredentialCredentialStatus) -> Self {
            Self {
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EuropeanDigitalCredentialType {
        attachment: Result<Option<super::ManyMediaObjectType>, String>,
        credential_profiles: Result<ObjectOrVector<ConceptType>, String>,
        credential_schema: Result<super::ManyShaclValidator2017Type, String>,
        credential_status: Result<Option<super::CredentialStatusType>, String>,
        credential_subject: Result<super::AgentOrPersonOrOrganisationType, String>,
        display_parameter: Result<super::DisplayParameterType, String>,
        evidence: Result<Option<super::ManyEvidenceType>, String>,
        expiration_date: Result<super::ManyDateTimeType, String>,
        holder: Result<Option<super::ManyAgentOrPersonOrOrganisationType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        identifier: Result<Option<super::ManyIdentifierOrLegalIdentifierType>, String>,
        issuance_date: Result<Option<super::DateTimeType>, String>,
        issued: Result<super::DateTimeType, String>,
        issuer: Result<super::AgentOrPersonOrOrganisationType, String>,
        proof: Result<Option<super::ManyProofType>, String>,
        terms_of_use: Result<Option<super::ManyTermsOfUseType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
        valid_from: Result<super::DateTimeType, String>,
        valid_until: Result<Option<super::DateTimeType>, String>,
    }
    impl Default for EuropeanDigitalCredentialType {
        fn default() -> Self {
            Self {
                attachment: Ok(Default::default()),
                credential_profiles: Err(
                    "no value supplied for credential_profiles".to_string(),
                ),
                credential_schema: Err(
                    "no value supplied for credential_schema".to_string(),
                ),
                credential_status: Ok(Default::default()),
                credential_subject: Err(
                    "no value supplied for credential_subject".to_string(),
                ),
                display_parameter: Err(
                    "no value supplied for display_parameter".to_string(),
                ),
                evidence: Ok(Default::default()),
                expiration_date: Err(
                    "no value supplied for expiration_date".to_string(),
                ),
                holder: Ok(Default::default()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                issuance_date: Ok(Default::default()),
                issued: Err("no value supplied for issued".to_string()),
                issuer: Err("no value supplied for issuer".to_string()),
                proof: Ok(Default::default()),
                terms_of_use: Ok(Default::default()),
                type_: Ok(Default::default()),
                valid_from: Err("no value supplied for valid_from".to_string()),
                valid_until: Ok(Default::default()),
            }
        }
    }
    impl EuropeanDigitalCredentialType {
        pub fn attachment<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyMediaObjectType>>,
            T::Error: std::fmt::Display,
        {
            self.attachment = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for attachment: {}", e)
                });
            self
        }
        pub fn credential_profiles<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<ObjectOrVector<ConceptType>>,
            T::Error: std::fmt::Display,
        {
            self.credential_profiles = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for credential_profiles: {}", e
                    )
                });
            self
        }
        pub fn credential_schema<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyShaclValidator2017Type>,
            T::Error: std::fmt::Display,
        {
            self.credential_schema = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for credential_schema: {}", e
                    )
                });
            self
        }
        pub fn credential_status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CredentialStatusType>>,
            T::Error: std::fmt::Display,
        {
            self.credential_status = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for credential_status: {}", e
                    )
                });
            self
        }
        pub fn credential_subject<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AgentOrPersonOrOrganisationType>,
            T::Error: std::fmt::Display,
        {
            self.credential_subject = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for credential_subject: {}", e
                    )
                });
            self
        }
        pub fn display_parameter<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DisplayParameterType>,
            T::Error: std::fmt::Display,
        {
            self.display_parameter = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for display_parameter: {}", e
                    )
                });
            self
        }
        pub fn evidence<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyEvidenceType>>,
            T::Error: std::fmt::Display,
        {
            self.evidence = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for evidence: {}", e)
                });
            self
        }
        pub fn expiration_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyDateTimeType>,
            T::Error: std::fmt::Display,
        {
            self.expiration_date = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for expiration_date: {}", e)
                });
            self
        }
        pub fn holder<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyAgentOrPersonOrOrganisationType>>,
            T::Error: std::fmt::Display,
        {
            self.holder = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for holder: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyIdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn issuance_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.issuance_date = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for issuance_date: {}", e)
                });
            self
        }
        pub fn issued<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DateTimeType>,
            T::Error: std::fmt::Display,
        {
            self.issued = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for issued: {}", e)
                });
            self
        }
        pub fn issuer<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AgentOrPersonOrOrganisationType>,
            T::Error: std::fmt::Display,
        {
            self.issuer = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for issuer: {}", e)
                });
            self
        }
        pub fn proof<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyProofType>>,
            T::Error: std::fmt::Display,
        {
            self.proof = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for proof: {}", e)
                });
            self
        }
        pub fn terms_of_use<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyTermsOfUseType>>,
            T::Error: std::fmt::Display,
        {
            self.terms_of_use = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for terms_of_use: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn valid_from<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DateTimeType>,
            T::Error: std::fmt::Display,
        {
            self.valid_from = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_from: {}", e)
                });
            self
        }
        pub fn valid_until<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.valid_until = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_until: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<EuropeanDigitalCredentialType>
    for super::EuropeanDigitalCredentialType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EuropeanDigitalCredentialType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                attachment: value.attachment?,
                credential_profiles: value.credential_profiles?,
                credential_schema: value.credential_schema?,
                credential_status: value.credential_status?,
                credential_subject: value.credential_subject?,
                display_parameter: value.display_parameter?,
                evidence: value.evidence?,
                expiration_date: value.expiration_date?,
                holder: value.holder?,
                id: value.id?,
                identifier: value.identifier?,
                issuance_date: value.issuance_date?,
                issued: value.issued?,
                issuer: value.issuer?,
                proof: value.proof?,
                terms_of_use: value.terms_of_use?,
                type_: value.type_?,
                valid_from: value.valid_from?,
                valid_until: value.valid_until?,
            })
        }
    }
    impl From<super::EuropeanDigitalCredentialType> for EuropeanDigitalCredentialType {
        fn from(value: super::EuropeanDigitalCredentialType) -> Self {
            Self {
                attachment: Ok(value.attachment),
                credential_profiles: Ok(value.credential_profiles),
                credential_schema: Ok(value.credential_schema),
                credential_status: Ok(value.credential_status),
                credential_subject: Ok(value.credential_subject),
                display_parameter: Ok(value.display_parameter),
                evidence: Ok(value.evidence),
                expiration_date: Ok(value.expiration_date),
                holder: Ok(value.holder),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                issuance_date: Ok(value.issuance_date),
                issued: Ok(value.issued),
                issuer: Ok(value.issuer),
                proof: Ok(value.proof),
                terms_of_use: Ok(value.terms_of_use),
                type_: Ok(value.type_),
                valid_from: Ok(value.valid_from),
                valid_until: Ok(value.valid_until),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EuropeanDigitalPresentationType {
        holder: Result<Option<super::ManyAgentOrPersonOrOrganisationType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        proof: Result<Option<super::ManyProofType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
        verifiable_credential: Result<
            Option<super::ManyEuropeanDigitalCredentialType>,
            String,
        >,
        verification_check: Result<Option<super::ManyVerificationCheckType>, String>,
    }
    impl Default for EuropeanDigitalPresentationType {
        fn default() -> Self {
            Self {
                holder: Ok(Default::default()),
                id: Ok(Default::default()),
                proof: Ok(Default::default()),
                type_: Ok(Default::default()),
                verifiable_credential: Ok(Default::default()),
                verification_check: Ok(Default::default()),
            }
        }
    }
    impl EuropeanDigitalPresentationType {
        pub fn holder<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyAgentOrPersonOrOrganisationType>>,
            T::Error: std::fmt::Display,
        {
            self.holder = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for holder: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn proof<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyProofType>>,
            T::Error: std::fmt::Display,
        {
            self.proof = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for proof: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn verifiable_credential<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyEuropeanDigitalCredentialType>>,
            T::Error: std::fmt::Display,
        {
            self.verifiable_credential = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for verifiable_credential: {}",
                        e
                    )
                });
            self
        }
        pub fn verification_check<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyVerificationCheckType>>,
            T::Error: std::fmt::Display,
        {
            self.verification_check = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for verification_check: {}", e
                    )
                });
            self
        }
    }
    impl std::convert::TryFrom<EuropeanDigitalPresentationType>
    for super::EuropeanDigitalPresentationType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EuropeanDigitalPresentationType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                holder: value.holder?,
                id: value.id?,
                proof: value.proof?,
                type_: value.type_?,
                verifiable_credential: value.verifiable_credential?,
                verification_check: value.verification_check?,
            })
        }
    }
    impl From<super::EuropeanDigitalPresentationType>
    for EuropeanDigitalPresentationType {
        fn from(value: super::EuropeanDigitalPresentationType) -> Self {
            Self {
                holder: Ok(value.holder),
                id: Ok(value.id),
                proof: Ok(value.proof),
                type_: Ok(value.type_),
                verifiable_credential: Ok(value.verifiable_credential),
                verification_check: Ok(value.verification_check),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Evidence {
        id: Result<Option<String>, String>,
        type_: Result<super::EvidenceType, String>,
    }
    impl Default for Evidence {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Evidence {
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
            T: std::convert::TryInto<super::EvidenceType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Evidence> for super::Evidence {
        type Error = super::error::ConversionError;
        fn try_from(value: Evidence) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                ev_type: value.type_?,
            })
        }
    }
    impl From<super::Evidence> for Evidence {
        fn from(value: super::Evidence) -> Self {
            Self {
                id: Ok(value.id),
                type_: Ok(value.ev_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvidenceType {
        accreditation: Result<Option<Box<super::AccreditationType>>, String>,
        embedded_evidence: Result<Option<super::ManyMediaObjectType>, String>,
        evidence_statement: Result<Option<super::StringType>, String>,
        evidence_target: Result<
            Option<Box<super::AgentOrPersonOrOrganisationType>>,
            String,
        >,
        id: Result<Option<super::GenericIdType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for EvidenceType {
        fn default() -> Self {
            Self {
                accreditation: Ok(Default::default()),
                embedded_evidence: Ok(Default::default()),
                evidence_statement: Ok(Default::default()),
                evidence_target: Ok(Default::default()),
                id: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl EvidenceType {
        pub fn accreditation<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Box<super::AccreditationType>>>,
            T::Error: std::fmt::Display,
        {
            self.accreditation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for accreditation: {}", e)
                });
            self
        }
        pub fn embedded_evidence<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyMediaObjectType>>,
            T::Error: std::fmt::Display,
        {
            self.embedded_evidence = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for embedded_evidence: {}", e
                    )
                });
            self
        }
        pub fn evidence_statement<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringType>>,
            T::Error: std::fmt::Display,
        {
            self.evidence_statement = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for evidence_statement: {}", e
                    )
                });
            self
        }
        pub fn evidence_target<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<Box<super::AgentOrPersonOrOrganisationType>>,
            >,
            T::Error: std::fmt::Display,
        {
            self.evidence_target = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for evidence_target: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<EvidenceType> for super::EvidenceType {
        type Error = super::error::ConversionError;
        fn try_from(value: EvidenceType) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                accreditation: value.accreditation?,
                embedded_evidence: value.embedded_evidence?,
                evidence_statement: value.evidence_statement?,
                evidence_target: value.evidence_target?,
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::EvidenceType> for EvidenceType {
        fn from(value: super::EvidenceType) -> Self {
            Self {
                accreditation: Ok(value.accreditation),
                embedded_evidence: Ok(value.embedded_evidence),
                evidence_statement: Ok(value.evidence_statement),
                evidence_target: Ok(value.evidence_target),
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GeometryType {
        id: Result<Option<super::GenericIdType>, String>,
        latitude: Result<Option<super::StringType>, String>,
        longitude: Result<Option<super::StringType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for GeometryType {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                latitude: Ok(Default::default()),
                longitude: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl GeometryType {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn latitude<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringType>>,
            T::Error: std::fmt::Display,
        {
            self.latitude = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for latitude: {}", e)
                });
            self
        }
        pub fn longitude<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringType>>,
            T::Error: std::fmt::Display,
        {
            self.longitude = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for longitude: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<GeometryType> for super::GeometryType {
        type Error = super::error::ConversionError;
        fn try_from(value: GeometryType) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                latitude: value.latitude?,
                longitude: value.longitude?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::GeometryType> for GeometryType {
        fn from(value: super::GeometryType) -> Self {
            Self {
                id: Ok(value.id),
                latitude: Ok(value.latitude),
                longitude: Ok(value.longitude),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GradingSchemeType {
        description: Result<Option<super::ManyLangStringType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        identifier: Result<Option<super::ManyIdentifierOrLegalIdentifierType>, String>,
        supplementary_document: Result<Option<super::ManyWebResourceType>, String>,
        title: Result<super::ManyLangStringType, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for GradingSchemeType {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                supplementary_document: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl GradingSchemeType {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyIdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn supplementary_document<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.supplementary_document = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for supplementary_document: {}",
                        e
                    )
                });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyLangStringType>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for title: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<GradingSchemeType> for super::GradingSchemeType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GradingSchemeType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                id: value.id?,
                identifier: value.identifier?,
                supplementary_document: value.supplementary_document?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::GradingSchemeType> for GradingSchemeType {
        fn from(value: super::GradingSchemeType) -> Self {
            Self {
                description: Ok(value.description),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                supplementary_document: Ok(value.supplementary_document),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GrantType {
        content_url: Result<Option<super::UriType>, String>,
        dc_type: Result<Option<super::ConceptType>, String>,
        description: Result<Option<super::ManyLangStringType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        supplementary_document: Result<Option<super::ManyWebResourceType>, String>,
        title: Result<super::ManyLangStringType, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for GrantType {
        fn default() -> Self {
            Self {
                content_url: Ok(Default::default()),
                dc_type: Ok(Default::default()),
                description: Ok(Default::default()),
                id: Ok(Default::default()),
                supplementary_document: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl GrantType {
        pub fn content_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::UriType>>,
            T::Error: std::fmt::Display,
        {
            self.content_url = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for content_url: {}", e)
                });
            self
        }
        pub fn dc_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ConceptType>>,
            T::Error: std::fmt::Display,
        {
            self.dc_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dc_type: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn supplementary_document<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.supplementary_document = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for supplementary_document: {}",
                        e
                    )
                });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyLangStringType>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for title: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<GrantType> for super::GrantType {
        type Error = super::error::ConversionError;
        fn try_from(value: GrantType) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                content_url: value.content_url?,
                dc_type: value.dc_type?,
                description: value.description?,
                id: value.id?,
                supplementary_document: value.supplementary_document?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::GrantType> for GrantType {
        fn from(value: super::GrantType) -> Self {
            Self {
                content_url: Ok(value.content_url),
                dc_type: Ok(value.dc_type),
                description: Ok(value.description),
                id: Ok(value.id),
                supplementary_document: Ok(value.supplementary_document),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GroupType {
        additional_note: Result<Option<super::ManyNoteType>, String>,
        alt_label: Result<Option<super::ManyLangStringType>, String>,
        contact_point: Result<Option<super::ManyContactPointType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        location: Result<Option<super::ManyLocationType>, String>,
        member: Result<Option<super::ManyAgentOrPersonOrOrganisationType>, String>,
        pref_label: Result<super::ManyLangStringType, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for GroupType {
        fn default() -> Self {
            Self {
                additional_note: Ok(Default::default()),
                alt_label: Ok(Default::default()),
                contact_point: Ok(Default::default()),
                id: Ok(Default::default()),
                location: Ok(Default::default()),
                member: Ok(Default::default()),
                pref_label: Err("no value supplied for pref_label".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl GroupType {
        pub fn additional_note<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyNoteType>>,
            T::Error: std::fmt::Display,
        {
            self.additional_note = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for additional_note: {}", e)
                });
            self
        }
        pub fn alt_label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.alt_label = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for alt_label: {}", e)
                });
            self
        }
        pub fn contact_point<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyContactPointType>>,
            T::Error: std::fmt::Display,
        {
            self.contact_point = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for contact_point: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLocationType>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for location: {}", e)
                });
            self
        }
        pub fn member<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyAgentOrPersonOrOrganisationType>>,
            T::Error: std::fmt::Display,
        {
            self.member = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for member: {}", e)
                });
            self
        }
        pub fn pref_label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyLangStringType>,
            T::Error: std::fmt::Display,
        {
            self.pref_label = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for pref_label: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<GroupType> for super::Group {
        type Error = super::error::ConversionError;
        fn try_from(value: GroupType) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_note: value.additional_note?,
                alt_label: value.alt_label?,
                contact_point: value.contact_point?,
                id: value.id?,
                location: value.location?,
                member: value.member?,
                pref_label: value.pref_label?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Group> for GroupType {
        fn from(value: super::Group) -> Self {
            Self {
                additional_note: Ok(value.additional_note),
                alt_label: Ok(value.alt_label),
                contact_point: Ok(value.contact_point),
                id: Ok(value.id),
                location: Ok(value.location),
                member: Ok(value.member),
                pref_label: Ok(value.pref_label),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IdentifierType {
        creator: Result<Option<super::IriType>, String>,
        date_issued: Result<Option<super::DateTimeType>, String>,
        dc_type: Result<Option<ObjectOrVector<ConceptType>>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        notation: Result<super::LiteralType, String>,
        scheme_agency: Result<Option<super::LangStringType>, String>,
        scheme_id: Result<Option<super::UriType>, String>,
        scheme_name: Result<Option<super::StringType>, String>,
        scheme_version: Result<Option<super::StringType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for IdentifierType {
        fn default() -> Self {
            Self {
                creator: Ok(Default::default()),
                date_issued: Ok(Default::default()),
                dc_type: Ok(Default::default()),
                id: Ok(Default::default()),
                notation: Err("no value supplied for notation".to_string()),
                scheme_agency: Ok(Default::default()),
                scheme_id: Ok(Default::default()),
                scheme_name: Ok(Default::default()),
                scheme_version: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl IdentifierType {
        pub fn creator<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IriType>>,
            T::Error: std::fmt::Display,
        {
            self.creator = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for creator: {}", e)
                });
            self
        }
        pub fn date_issued<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.date_issued = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for date_issued: {}", e)
                });
            self
        }
        pub fn dc_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.dc_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dc_type: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn notation<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::LiteralType>,
            T::Error: std::fmt::Display,
        {
            self.notation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for notation: {}", e)
                });
            self
        }
        pub fn scheme_agency<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.scheme_agency = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for scheme_agency: {}", e)
                });
            self
        }
        pub fn scheme_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::UriType>>,
            T::Error: std::fmt::Display,
        {
            self.scheme_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for scheme_id: {}", e)
                });
            self
        }
        pub fn scheme_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringType>>,
            T::Error: std::fmt::Display,
        {
            self.scheme_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for scheme_name: {}", e)
                });
            self
        }
        pub fn scheme_version<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringType>>,
            T::Error: std::fmt::Display,
        {
            self.scheme_version = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for scheme_version: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<IdentifierType> for super::IdentifierType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: IdentifierType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                creator: value.creator?,
                date_issued: value.date_issued?,
                dc_type: value.dc_type?,
                id: value.id?,
                notation: value.notation?,
                scheme_agency: value.scheme_agency?,
                scheme_id: value.scheme_id?,
                scheme_name: value.scheme_name?,
                scheme_version: value.scheme_version?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::IdentifierType> for IdentifierType {
        fn from(value: super::IdentifierType) -> Self {
            Self {
                creator: Ok(value.creator),
                date_issued: Ok(value.date_issued),
                dc_type: Ok(value.dc_type),
                id: Ok(value.id),
                notation: Ok(value.notation),
                scheme_agency: Ok(value.scheme_agency),
                scheme_id: Ok(value.scheme_id),
                scheme_name: Ok(value.scheme_name),
                scheme_version: Ok(value.scheme_version),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IndividualDisplayType {
        display_detail: Result<super::ManyDisplayDetailType, String>,
        id: Result<Option<super::GenericIdType>, String>,
        language: Result<super::ConceptType, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for IndividualDisplayType {
        fn default() -> Self {
            Self {
                display_detail: Err("no value supplied for display_detail".to_string()),
                id: Ok(Default::default()),
                language: Err("no value supplied for language".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl IndividualDisplayType {
        pub fn display_detail<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyDisplayDetailType>,
            T::Error: std::fmt::Display,
        {
            self.display_detail = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for display_detail: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ConceptType>,
            T::Error: std::fmt::Display,
        {
            self.language = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for language: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<IndividualDisplayType> for super::IndividualDisplayType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: IndividualDisplayType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                display_detail: value.display_detail?,
                id: value.id?,
                language: value.language?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::IndividualDisplayType> for IndividualDisplayType {
        fn from(value: super::IndividualDisplayType) -> Self {
            Self {
                display_detail: Ok(value.display_detail),
                id: Ok(value.id),
                language: Ok(value.language),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IssuerNodeType {
        eidas_legal_identifier: Result<super::LegalIdentifierType, String>,
        id: Result<Option<super::GenericIdType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for IssuerNodeType {
        fn default() -> Self {
            Self {
                eidas_legal_identifier: Err(
                    "no value supplied for eidas_legal_identifier".to_string(),
                ),
                id: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl IssuerNodeType {
        pub fn eidas_legal_identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::LegalIdentifierType>,
            T::Error: std::fmt::Display,
        {
            self.eidas_legal_identifier = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for eidas_legal_identifier: {}",
                        e
                    )
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<IssuerNodeType> for super::IssuerNodeType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: IssuerNodeType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                eidas_legal_identifier: value.eidas_legal_identifier?,
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::IssuerNodeType> for IssuerNodeType {
        fn from(value: super::IssuerNodeType) -> Self {
            Self {
                eidas_legal_identifier: Ok(value.eidas_legal_identifier),
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LearningAchievementSpecificationOrQualificationType {
        subtype_0: Result<Option<super::LearningAchievementSpecificationType>, String>,
        subtype_1: Result<Option<super::QualificationType>, String>,
    }
    impl Default for LearningAchievementSpecificationOrQualificationType {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl LearningAchievementSpecificationOrQualificationType {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::LearningAchievementSpecificationType>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_0: {}", e)
                });
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::QualificationType>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_1: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<LearningAchievementSpecificationOrQualificationType>
    for super::LearningAchievementSpecificationOrQualificationType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LearningAchievementSpecificationOrQualificationType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl From<super::LearningAchievementSpecificationOrQualificationType>
    for LearningAchievementSpecificationOrQualificationType {
        fn from(
            value: super::LearningAchievementSpecificationOrQualificationType,
        ) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LearningAchievementSpecificationOrSpecificationType {
        subtype_0: Result<Option<super::LearningAchievementSpecificationType>, String>,
        subtype_1: Result<Option<super::QualificationType>, String>,
    }
    impl Default for LearningAchievementSpecificationOrSpecificationType {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl LearningAchievementSpecificationOrSpecificationType {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::LearningAchievementSpecificationType>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_0: {}", e)
                });
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::QualificationType>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_1: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<LearningAchievementSpecificationOrSpecificationType>
    for super::LearningAchievementSpecificationOrSpecificationType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LearningAchievementSpecificationOrSpecificationType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl From<super::LearningAchievementSpecificationOrSpecificationType>
    for LearningAchievementSpecificationOrSpecificationType {
        fn from(
            value: super::LearningAchievementSpecificationOrSpecificationType,
        ) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LearningAchievementSpecificationType {
        additional_note: Result<Option<super::ManyNoteType>, String>,
        alt_label: Result<Option<super::ManyLangStringType>, String>,
        awarding_opportunity: Result<Option<super::ManyAwardingOpportunityType>, String>,
        category: Result<Option<super::ManyLangStringType>, String>,
        credit_point: Result<Option<super::ManyCreditPointType>, String>,
        date_modified: Result<Option<super::DateTimeType>, String>,
        dc_type: Result<Option<ObjectOrVector<ConceptType>>, String>,
        description: Result<Option<super::ManyLangStringType>, String>,
        education_level: Result<Option<ObjectOrVector<ConceptType>>, String>,
        education_subject: Result<Option<ObjectOrVector<ConceptType>>, String>,
        entitles_to: Result<
            Option<super::ManyLearningEntitlementSpecificationType>,
            String,
        >,
        entry_requirement: Result<Option<super::NoteType>, String>,
        generalisation_of: Result<
            Box<Option<super::ManyLearningAchievementSpecificationOrQualificationType>>,
            String,
        >,
        has_part: Result<
            Box<Option<super::ManyLearningAchievementSpecificationOrQualificationType>>,
            String,
        >,
        homepage: Result<Option<super::ManyWebResourceType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        identifier: Result<Option<super::IdentifierOrLegalIdentifierType>, String>,
        influenced_by: Result<
            Box<Option<super::ManyLearningActivitySpecificationType>>,
            String,
        >,
        is_part_of: Result<
            Box<Option<super::ManyLearningAchievementSpecificationOrQualificationType>>,
            String,
        >,
        language: Result<Option<ObjectOrVector<ConceptType>>, String>,
        learning_outcome: Result<Option<super::ManyLearningOutcomeType>, String>,
        learning_outcome_summary: Result<Option<super::NoteType>, String>,
        learning_setting: Result<Option<super::ConceptType>, String>,
        maximum_duration: Result<Option<super::DurationType>, String>,
        mode: Result<Option<ObjectOrVector<ConceptType>>, String>,
        proven_by: Result<
            Box<Option<super::ManyLearningAssessmentSpecificationType>>,
            String,
        >,
        specialisation_of: Result<
            Box<Option<super::ManyLearningAchievementSpecificationOrQualificationType>>,
            String,
        >,
        status: Result<Option<super::StringType>, String>,
        supplementary_document: Result<Option<super::ManyWebResourceType>, String>,
        target_group: Result<Option<ObjectOrVector<ConceptType>>, String>,
        thematic_area: Result<Option<ObjectOrVector<ConceptType>>, String>,
        title: Result<super::ManyLangStringType, String>,
        type_: Result<Option<serde_json::Value>, String>,
        volume_of_learning: Result<Option<super::DurationType>, String>,
    }
    impl Default for LearningAchievementSpecificationType {
        fn default() -> Self {
            Self {
                additional_note: Ok(Default::default()),
                alt_label: Ok(Default::default()),
                awarding_opportunity: Ok(Default::default()),
                category: Ok(Default::default()),
                credit_point: Ok(Default::default()),
                date_modified: Ok(Default::default()),
                dc_type: Ok(Default::default()),
                description: Ok(Default::default()),
                education_level: Ok(Default::default()),
                education_subject: Ok(Default::default()),
                entitles_to: Ok(Default::default()),
                entry_requirement: Ok(Default::default()),
                generalisation_of: Ok(Default::default()),
                has_part: Ok(Default::default()),
                homepage: Ok(Default::default()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                influenced_by: Ok(Default::default()),
                is_part_of: Ok(Default::default()),
                language: Ok(Default::default()),
                learning_outcome: Ok(Default::default()),
                learning_outcome_summary: Ok(Default::default()),
                learning_setting: Ok(Default::default()),
                maximum_duration: Ok(Default::default()),
                mode: Ok(Default::default()),
                proven_by: Ok(Default::default()),
                specialisation_of: Ok(Default::default()),
                status: Ok(Default::default()),
                supplementary_document: Ok(Default::default()),
                target_group: Ok(Default::default()),
                thematic_area: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                type_: Ok(Default::default()),
                volume_of_learning: Ok(Default::default()),
            }
        }
    }
    impl LearningAchievementSpecificationType {
        pub fn additional_note<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyNoteType>>,
            T::Error: std::fmt::Display,
        {
            self.additional_note = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for additional_note: {}", e)
                });
            self
        }
        pub fn alt_label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.alt_label = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for alt_label: {}", e)
                });
            self
        }
        pub fn awarding_opportunity<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyAwardingOpportunityType>>,
            T::Error: std::fmt::Display,
        {
            self.awarding_opportunity = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for awarding_opportunity: {}", e
                    )
                });
            self
        }
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for category: {}", e)
                });
            self
        }
        pub fn credit_point<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyCreditPointType>>,
            T::Error: std::fmt::Display,
        {
            self.credit_point = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for credit_point: {}", e)
                });
            self
        }
        pub fn date_modified<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.date_modified = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for date_modified: {}", e)
                });
            self
        }
        pub fn dc_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.dc_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dc_type: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn education_level<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.education_level = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for education_level: {}", e)
                });
            self
        }
        pub fn education_subject<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.education_subject = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for education_subject: {}", e
                    )
                });
            self
        }
        pub fn entitles_to<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::ManyLearningEntitlementSpecificationType>,
            >,
            T::Error: std::fmt::Display,
        {
            self.entitles_to = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for entitles_to: {}", e)
                });
            self
        }
        pub fn entry_requirement<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::NoteType>>,
            T::Error: std::fmt::Display,
        {
            self.entry_requirement = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for entry_requirement: {}", e
                    )
                });
            self
        }
        pub fn generalisation_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Box<
                    Option<
                        super::ManyLearningAchievementSpecificationOrQualificationType,
                    >,
                >,
            >,
            T::Error: std::fmt::Display,
        {
            self.generalisation_of = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for generalisation_of: {}", e
                    )
                });
            self
        }
        pub fn has_part<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Box<
                    Option<
                        super::ManyLearningAchievementSpecificationOrQualificationType,
                    >,
                >,
            >,
            T::Error: std::fmt::Display,
        {
            self.has_part = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for has_part: {}", e)
                });
            self
        }
        pub fn homepage<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.homepage = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for homepage: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn influenced_by<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Box<Option<super::ManyLearningActivitySpecificationType>>,
            >,
            T::Error: std::fmt::Display,
        {
            self.influenced_by = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for influenced_by: {}", e)
                });
            self
        }
        pub fn is_part_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Box<
                    Option<
                        super::ManyLearningAchievementSpecificationOrQualificationType,
                    >,
                >,
            >,
            T::Error: std::fmt::Display,
        {
            self.is_part_of = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_part_of: {}", e)
                });
            self
        }
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.language = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for language: {}", e)
                });
            self
        }
        pub fn learning_outcome<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLearningOutcomeType>>,
            T::Error: std::fmt::Display,
        {
            self.learning_outcome = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for learning_outcome: {}", e
                    )
                });
            self
        }
        pub fn learning_outcome_summary<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::NoteType>>,
            T::Error: std::fmt::Display,
        {
            self.learning_outcome_summary = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for learning_outcome_summary: {}",
                        e
                    )
                });
            self
        }
        pub fn learning_setting<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ConceptType>>,
            T::Error: std::fmt::Display,
        {
            self.learning_setting = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for learning_setting: {}", e
                    )
                });
            self
        }
        pub fn maximum_duration<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DurationType>>,
            T::Error: std::fmt::Display,
        {
            self.maximum_duration = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for maximum_duration: {}", e
                    )
                });
            self
        }
        pub fn mode<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mode: {}", e));
            self
        }
        pub fn proven_by<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Box<Option<super::ManyLearningAssessmentSpecificationType>>,
            >,
            T::Error: std::fmt::Display,
        {
            self.proven_by = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for proven_by: {}", e)
                });
            self
        }
        pub fn specialisation_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Box<
                    Option<
                        super::ManyLearningAchievementSpecificationOrQualificationType,
                    >,
                >,
            >,
            T::Error: std::fmt::Display,
        {
            self.specialisation_of = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for specialisation_of: {}", e
                    )
                });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringType>>,
            T::Error: std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status: {}", e)
                });
            self
        }
        pub fn supplementary_document<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.supplementary_document = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for supplementary_document: {}",
                        e
                    )
                });
            self
        }
        pub fn target_group<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.target_group = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for target_group: {}", e)
                });
            self
        }
        pub fn thematic_area<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.thematic_area = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for thematic_area: {}", e)
                });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyLangStringType>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for title: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn volume_of_learning<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DurationType>>,
            T::Error: std::fmt::Display,
        {
            self.volume_of_learning = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for volume_of_learning: {}", e
                    )
                });
            self
        }
    }
    impl std::convert::TryFrom<LearningAchievementSpecificationType>
    for super::LearningAchievementSpecificationType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LearningAchievementSpecificationType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_note: value.additional_note?,
                alt_label: value.alt_label?,
                awarding_opportunity: value.awarding_opportunity?,
                category: value.category?,
                credit_point: value.credit_point?,
                date_modified: value.date_modified?,
                dc_type: value.dc_type?,
                description: value.description?,
                education_level: value.education_level?,
                education_subject: value.education_subject?,
                entitles_to: value.entitles_to?,
                entry_requirement: value.entry_requirement?,
                generalisation_of: value.generalisation_of?,
                has_part: value.has_part?,
                homepage: value.homepage?,
                id: value.id?,
                identifier: value.identifier?,
                influenced_by: value.influenced_by?,
                is_part_of: value.is_part_of?,
                language: value.language?,
                learning_outcome: value.learning_outcome?,
                learning_outcome_summary: value.learning_outcome_summary?,
                learning_setting: value.learning_setting?,
                maximum_duration: value.maximum_duration?,
                mode: value.mode?,
                proven_by: value.proven_by?,
                specialisation_of: value.specialisation_of?,
                status: value.status?,
                supplementary_document: value.supplementary_document?,
                target_group: value.target_group?,
                thematic_area: value.thematic_area?,
                title: value.title?,
                type_: value.type_?,
                volume_of_learning: value.volume_of_learning?,
            })
        }
    }
    impl From<super::LearningAchievementSpecificationType>
    for LearningAchievementSpecificationType {
        fn from(value: super::LearningAchievementSpecificationType) -> Self {
            Self {
                additional_note: Ok(value.additional_note),
                alt_label: Ok(value.alt_label),
                awarding_opportunity: Ok(value.awarding_opportunity),
                category: Ok(value.category),
                credit_point: Ok(value.credit_point),
                date_modified: Ok(value.date_modified),
                dc_type: Ok(value.dc_type),
                description: Ok(value.description),
                education_level: Ok(value.education_level),
                education_subject: Ok(value.education_subject),
                entitles_to: Ok(value.entitles_to),
                entry_requirement: Ok(value.entry_requirement),
                generalisation_of: Ok(value.generalisation_of),
                has_part: Ok(value.has_part),
                homepage: Ok(value.homepage),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                influenced_by: Ok(value.influenced_by),
                is_part_of: Ok(value.is_part_of),
                language: Ok(value.language),
                learning_outcome: Ok(value.learning_outcome),
                learning_outcome_summary: Ok(value.learning_outcome_summary),
                learning_setting: Ok(value.learning_setting),
                maximum_duration: Ok(value.maximum_duration),
                mode: Ok(value.mode),
                proven_by: Ok(value.proven_by),
                specialisation_of: Ok(value.specialisation_of),
                status: Ok(value.status),
                supplementary_document: Ok(value.supplementary_document),
                target_group: Ok(value.target_group),
                thematic_area: Ok(value.thematic_area),
                title: Ok(value.title),
                type_: Ok(value.type_),
                volume_of_learning: Ok(value.volume_of_learning),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LearningAchievementType {
        additional_note: Result<Option<super::ManyNoteType>, String>,
        awarded_by: Result<Box<super::AwardingProcessType>, String>,
        credit_received: Result<Option<super::ManyCreditPointType>, String>,
        dc_type: Result<Option<ObjectOrVector<ConceptType>>, String>,
        description: Result<Option<super::ManyLangStringType>, String>,
        entitles_to: Result<Option<super::ManyLearningEntitlementType>, String>,
        has_part: Result<Box<Option<super::ManyLearningAchievementType>>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        identifier: Result<Option<super::IdentifierOrLegalIdentifierType>, String>,
        influenced_by: Result<Option<super::ManyLearningActivityType>, String>,
        is_part_of: Result<Box<Option<super::ManyLearningAchievementType>>, String>,
        learning_opportunity: Result<Option<super::LearningOpportunityType>, String>,
        proven_by: Result<Box<Option<super::ManyLearningAssessmentType>>, String>,
        specified_by: Result<
            Option<super::LearningAchievementSpecificationOrQualificationType>,
            String,
        >,
        supplementary_document: Result<Option<super::ManyWebResourceType>, String>,
        title: Result<super::ManyLangStringType, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for LearningAchievementType {
        fn default() -> Self {
            Self {
                additional_note: Ok(Default::default()),
                awarded_by: Err("no value supplied for awarded_by".to_string()),
                credit_received: Ok(Default::default()),
                dc_type: Ok(Default::default()),
                description: Ok(Default::default()),
                entitles_to: Ok(Default::default()),
                has_part: Ok(Default::default()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                influenced_by: Ok(Default::default()),
                is_part_of: Ok(Default::default()),
                learning_opportunity: Ok(Default::default()),
                proven_by: Ok(Default::default()),
                specified_by: Ok(Default::default()),
                supplementary_document: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl LearningAchievementType {
        pub fn additional_note<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyNoteType>>,
            T::Error: std::fmt::Display,
        {
            self.additional_note = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for additional_note: {}", e)
                });
            self
        }
        pub fn awarded_by<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<super::AwardingProcessType>>,
            T::Error: std::fmt::Display,
        {
            self.awarded_by = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for awarded_by: {}", e)
                });
            self
        }
        pub fn credit_received<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyCreditPointType>>,
            T::Error: std::fmt::Display,
        {
            self.credit_received = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for credit_received: {}", e)
                });
            self
        }
        pub fn dc_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.dc_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dc_type: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn entitles_to<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLearningEntitlementType>>,
            T::Error: std::fmt::Display,
        {
            self.entitles_to = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for entitles_to: {}", e)
                });
            self
        }
        pub fn has_part<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<Option<super::ManyLearningAchievementType>>>,
            T::Error: std::fmt::Display,
        {
            self.has_part = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for has_part: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn influenced_by<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLearningActivityType>>,
            T::Error: std::fmt::Display,
        {
            self.influenced_by = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for influenced_by: {}", e)
                });
            self
        }
        pub fn is_part_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<Option<super::ManyLearningAchievementType>>>,
            T::Error: std::fmt::Display,
        {
            self.is_part_of = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_part_of: {}", e)
                });
            self
        }
        pub fn learning_opportunity<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LearningOpportunityType>>,
            T::Error: std::fmt::Display,
        {
            self.learning_opportunity = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for learning_opportunity: {}", e
                    )
                });
            self
        }
        pub fn proven_by<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<Option<super::ManyLearningAssessmentType>>>,
            T::Error: std::fmt::Display,
        {
            self.proven_by = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for proven_by: {}", e)
                });
            self
        }
        pub fn specified_by<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::LearningAchievementSpecificationOrQualificationType>,
            >,
            T::Error: std::fmt::Display,
        {
            self.specified_by = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for specified_by: {}", e)
                });
            self
        }
        pub fn supplementary_document<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.supplementary_document = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for supplementary_document: {}",
                        e
                    )
                });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyLangStringType>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for title: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<LearningAchievementType>
    for super::LearningAchievementType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LearningAchievementType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_note: value.additional_note?,
                awarded_by: value.awarded_by?,
                credit_received: value.credit_received?,
                dc_type: value.dc_type?,
                description: value.description?,
                entitles_to: value.entitles_to?,
                has_part: value.has_part?,
                id: value.id?,
                identifier: value.identifier?,
                influenced_by: value.influenced_by?,
                is_part_of: value.is_part_of?,
                learning_opportunity: value.learning_opportunity?,
                proven_by: value.proven_by?,
                specified_by: value.specified_by?,
                supplementary_document: value.supplementary_document?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::LearningAchievementType> for LearningAchievementType {
        fn from(value: super::LearningAchievementType) -> Self {
            Self {
                additional_note: Ok(value.additional_note),
                awarded_by: Ok(value.awarded_by),
                credit_received: Ok(value.credit_received),
                dc_type: Ok(value.dc_type),
                description: Ok(value.description),
                entitles_to: Ok(value.entitles_to),
                has_part: Ok(value.has_part),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                influenced_by: Ok(value.influenced_by),
                is_part_of: Ok(value.is_part_of),
                learning_opportunity: Ok(value.learning_opportunity),
                proven_by: Ok(value.proven_by),
                specified_by: Ok(value.specified_by),
                supplementary_document: Ok(value.supplementary_document),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LearningActivitySpecificationType {
        additional_note: Result<Option<super::ManyNoteType>, String>,
        alt_label: Result<Option<super::ManyLangStringType>, String>,
        category: Result<Option<super::ManyLangStringType>, String>,
        contact_hour: Result<Option<super::ManyStringType>, String>,
        date_modified: Result<Option<super::DateTimeType>, String>,
        dc_type: Result<Option<ObjectOrVector<ConceptType>>, String>,
        description: Result<Option<super::ManyLangStringType>, String>,
        generalisation_of: Result<
            Option<super::ManyLearningAchievementSpecificationType>,
            String,
        >,
        has_part: Result<
            Option<super::ManyLearningAchievementSpecificationType>,
            String,
        >,
        homepage: Result<Option<super::ManyWebResourceType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        identifier: Result<Option<super::ManyIdentifierOrLegalIdentifierType>, String>,
        influences: Result<
            Box<Option<super::ManyLearningAchievementSpecificationOrQualificationType>>,
            String,
        >,
        is_part_of: Result<
            Option<super::ManyLearningAchievementSpecificationType>,
            String,
        >,
        language: Result<Option<ObjectOrVector<ConceptType>>, String>,
        mode: Result<Option<ObjectOrVector<ConceptType>>, String>,
        specialisation_of: Result<
            Option<super::ManyLearningAchievementSpecificationType>,
            String,
        >,
        status: Result<Option<super::StringType>, String>,
        supplementary_document: Result<Option<super::ManyWebResourceType>, String>,
        title: Result<super::ManyLangStringType, String>,
        type_: Result<Option<serde_json::Value>, String>,
        volume_of_learning: Result<Option<super::DurationType>, String>,
    }
    impl Default for LearningActivitySpecificationType {
        fn default() -> Self {
            Self {
                additional_note: Ok(Default::default()),
                alt_label: Ok(Default::default()),
                category: Ok(Default::default()),
                contact_hour: Ok(Default::default()),
                date_modified: Ok(Default::default()),
                dc_type: Ok(Default::default()),
                description: Ok(Default::default()),
                generalisation_of: Ok(Default::default()),
                has_part: Ok(Default::default()),
                homepage: Ok(Default::default()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                influences: Ok(Default::default()),
                is_part_of: Ok(Default::default()),
                language: Ok(Default::default()),
                mode: Ok(Default::default()),
                specialisation_of: Ok(Default::default()),
                status: Ok(Default::default()),
                supplementary_document: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                type_: Ok(Default::default()),
                volume_of_learning: Ok(Default::default()),
            }
        }
    }
    impl LearningActivitySpecificationType {
        pub fn additional_note<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyNoteType>>,
            T::Error: std::fmt::Display,
        {
            self.additional_note = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for additional_note: {}", e)
                });
            self
        }
        pub fn alt_label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.alt_label = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for alt_label: {}", e)
                });
            self
        }
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for category: {}", e)
                });
            self
        }
        pub fn contact_hour<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyStringType>>,
            T::Error: std::fmt::Display,
        {
            self.contact_hour = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for contact_hour: {}", e)
                });
            self
        }
        pub fn date_modified<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.date_modified = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for date_modified: {}", e)
                });
            self
        }
        pub fn dc_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.dc_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dc_type: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn generalisation_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::ManyLearningAchievementSpecificationType>,
            >,
            T::Error: std::fmt::Display,
        {
            self.generalisation_of = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for generalisation_of: {}", e
                    )
                });
            self
        }
        pub fn has_part<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::ManyLearningAchievementSpecificationType>,
            >,
            T::Error: std::fmt::Display,
        {
            self.has_part = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for has_part: {}", e)
                });
            self
        }
        pub fn homepage<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.homepage = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for homepage: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyIdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn influences<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Box<
                    Option<
                        super::ManyLearningAchievementSpecificationOrQualificationType,
                    >,
                >,
            >,
            T::Error: std::fmt::Display,
        {
            self.influences = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for influences: {}", e)
                });
            self
        }
        pub fn is_part_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::ManyLearningAchievementSpecificationType>,
            >,
            T::Error: std::fmt::Display,
        {
            self.is_part_of = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_part_of: {}", e)
                });
            self
        }
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.language = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for language: {}", e)
                });
            self
        }
        pub fn mode<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mode: {}", e));
            self
        }
        pub fn specialisation_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::ManyLearningAchievementSpecificationType>,
            >,
            T::Error: std::fmt::Display,
        {
            self.specialisation_of = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for specialisation_of: {}", e
                    )
                });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringType>>,
            T::Error: std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status: {}", e)
                });
            self
        }
        pub fn supplementary_document<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.supplementary_document = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for supplementary_document: {}",
                        e
                    )
                });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyLangStringType>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for title: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn volume_of_learning<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DurationType>>,
            T::Error: std::fmt::Display,
        {
            self.volume_of_learning = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for volume_of_learning: {}", e
                    )
                });
            self
        }
    }
    impl std::convert::TryFrom<LearningActivitySpecificationType>
    for super::LearningActivitySpecificationType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LearningActivitySpecificationType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_note: value.additional_note?,
                alt_label: value.alt_label?,
                category: value.category?,
                contact_hour: value.contact_hour?,
                date_modified: value.date_modified?,
                dc_type: value.dc_type?,
                description: value.description?,
                generalisation_of: value.generalisation_of?,
                has_part: value.has_part?,
                homepage: value.homepage?,
                id: value.id?,
                identifier: value.identifier?,
                influences: value.influences?,
                is_part_of: value.is_part_of?,
                language: value.language?,
                mode: value.mode?,
                specialisation_of: value.specialisation_of?,
                status: value.status?,
                supplementary_document: value.supplementary_document?,
                title: value.title?,
                type_: value.type_?,
                volume_of_learning: value.volume_of_learning?,
            })
        }
    }
    impl From<super::LearningActivitySpecificationType>
    for LearningActivitySpecificationType {
        fn from(value: super::LearningActivitySpecificationType) -> Self {
            Self {
                additional_note: Ok(value.additional_note),
                alt_label: Ok(value.alt_label),
                category: Ok(value.category),
                contact_hour: Ok(value.contact_hour),
                date_modified: Ok(value.date_modified),
                dc_type: Ok(value.dc_type),
                description: Ok(value.description),
                generalisation_of: Ok(value.generalisation_of),
                has_part: Ok(value.has_part),
                homepage: Ok(value.homepage),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                influences: Ok(value.influences),
                is_part_of: Ok(value.is_part_of),
                language: Ok(value.language),
                mode: Ok(value.mode),
                specialisation_of: Ok(value.specialisation_of),
                status: Ok(value.status),
                supplementary_document: Ok(value.supplementary_document),
                title: Ok(value.title),
                type_: Ok(value.type_),
                volume_of_learning: Ok(value.volume_of_learning),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LearningActivityType {
        additional_note: Result<Option<super::ManyNoteType>, String>,
        awarded_by: Result<Box<super::AwardingProcessType>, String>,
        dc_type: Result<Option<ObjectOrVector<ConceptType>>, String>,
        description: Result<Option<super::ManyLangStringType>, String>,
        directed_by: Result<Option<super::ManyAgentOrPersonOrOrganisationType>, String>,
        has_part: Result<Box<Option<super::ManyLearningActivityType>>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        identifier: Result<Option<super::ManyIdentifierOrLegalIdentifierType>, String>,
        influences: Result<Box<Option<super::ManyLearningAchievementType>>, String>,
        is_part_of: Result<Box<Option<super::ManyLearningActivityType>>, String>,
        learning_opportunity: Result<Option<super::LearningOpportunityType>, String>,
        level_of_completion: Result<Option<super::PercentageIntegerType>, String>,
        location: Result<Option<super::ManyLocationType>, String>,
        specified_by: Result<Option<super::LearningActivitySpecificationType>, String>,
        supplementary_document: Result<Option<super::ManyWebResourceType>, String>,
        temporal: Result<Option<super::ManyPeriodOfTimeType>, String>,
        title: Result<super::ManyLangStringType, String>,
        type_: Result<Option<serde_json::Value>, String>,
        workload: Result<Option<super::DurationType>, String>,
    }
    impl Default for LearningActivityType {
        fn default() -> Self {
            Self {
                additional_note: Ok(Default::default()),
                awarded_by: Err("no value supplied for awarded_by".to_string()),
                dc_type: Ok(Default::default()),
                description: Ok(Default::default()),
                directed_by: Ok(Default::default()),
                has_part: Ok(Default::default()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                influences: Ok(Default::default()),
                is_part_of: Ok(Default::default()),
                learning_opportunity: Ok(Default::default()),
                level_of_completion: Ok(Default::default()),
                location: Ok(Default::default()),
                specified_by: Ok(Default::default()),
                supplementary_document: Ok(Default::default()),
                temporal: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                type_: Ok(Default::default()),
                workload: Ok(Default::default()),
            }
        }
    }
    impl LearningActivityType {
        pub fn additional_note<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyNoteType>>,
            T::Error: std::fmt::Display,
        {
            self.additional_note = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for additional_note: {}", e)
                });
            self
        }
        pub fn awarded_by<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<super::AwardingProcessType>>,
            T::Error: std::fmt::Display,
        {
            self.awarded_by = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for awarded_by: {}", e)
                });
            self
        }
        pub fn dc_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.dc_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dc_type: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn directed_by<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyAgentOrPersonOrOrganisationType>>,
            T::Error: std::fmt::Display,
        {
            self.directed_by = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for directed_by: {}", e)
                });
            self
        }
        pub fn has_part<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<Option<super::ManyLearningActivityType>>>,
            T::Error: std::fmt::Display,
        {
            self.has_part = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for has_part: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyIdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn influences<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<Option<super::ManyLearningAchievementType>>>,
            T::Error: std::fmt::Display,
        {
            self.influences = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for influences: {}", e)
                });
            self
        }
        pub fn is_part_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<Option<super::ManyLearningActivityType>>>,
            T::Error: std::fmt::Display,
        {
            self.is_part_of = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_part_of: {}", e)
                });
            self
        }
        pub fn learning_opportunity<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LearningOpportunityType>>,
            T::Error: std::fmt::Display,
        {
            self.learning_opportunity = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for learning_opportunity: {}", e
                    )
                });
            self
        }
        pub fn level_of_completion<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::PercentageIntegerType>>,
            T::Error: std::fmt::Display,
        {
            self.level_of_completion = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for level_of_completion: {}", e
                    )
                });
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLocationType>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for location: {}", e)
                });
            self
        }
        pub fn specified_by<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LearningActivitySpecificationType>>,
            T::Error: std::fmt::Display,
        {
            self.specified_by = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for specified_by: {}", e)
                });
            self
        }
        pub fn supplementary_document<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.supplementary_document = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for supplementary_document: {}",
                        e
                    )
                });
            self
        }
        pub fn temporal<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyPeriodOfTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.temporal = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for temporal: {}", e)
                });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyLangStringType>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for title: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn workload<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DurationType>>,
            T::Error: std::fmt::Display,
        {
            self.workload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for workload: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<LearningActivityType> for super::LearningActivityType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LearningActivityType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_note: value.additional_note?,
                awarded_by: value.awarded_by?,
                dc_type: value.dc_type?,
                description: value.description?,
                directed_by: value.directed_by?,
                has_part: value.has_part?,
                id: value.id?,
                identifier: value.identifier?,
                influences: value.influences?,
                is_part_of: value.is_part_of?,
                learning_opportunity: value.learning_opportunity?,
                level_of_completion: value.level_of_completion?,
                location: value.location?,
                specified_by: value.specified_by?,
                supplementary_document: value.supplementary_document?,
                temporal: value.temporal?,
                title: value.title?,
                type_: value.type_?,
                workload: value.workload?,
            })
        }
    }
    impl From<super::LearningActivityType> for LearningActivityType {
        fn from(value: super::LearningActivityType) -> Self {
            Self {
                additional_note: Ok(value.additional_note),
                awarded_by: Ok(value.awarded_by),
                dc_type: Ok(value.dc_type),
                description: Ok(value.description),
                directed_by: Ok(value.directed_by),
                has_part: Ok(value.has_part),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                influences: Ok(value.influences),
                is_part_of: Ok(value.is_part_of),
                learning_opportunity: Ok(value.learning_opportunity),
                level_of_completion: Ok(value.level_of_completion),
                location: Ok(value.location),
                specified_by: Ok(value.specified_by),
                supplementary_document: Ok(value.supplementary_document),
                temporal: Ok(value.temporal),
                title: Ok(value.title),
                type_: Ok(value.type_),
                workload: Ok(value.workload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LearningAssessmentSpecificationType {
        additional_note: Result<Option<super::ManyNoteType>, String>,
        alt_label: Result<Option<super::ManyLangStringType>, String>,
        category: Result<Option<super::ManyLangStringType>, String>,
        date_modified: Result<Option<super::DateTimeType>, String>,
        dc_type: Result<Option<super::ConceptType>, String>,
        description: Result<Option<super::ManyLangStringType>, String>,
        generalisation_of: Result<
            Box<Option<super::ManyLearningAssessmentSpecificationType>>,
            String,
        >,
        grading_scheme: Result<Option<super::GradingSchemeType>, String>,
        has_part: Result<
            Box<Option<super::ManyLearningAssessmentSpecificationType>>,
            String,
        >,
        homepage: Result<Option<super::ManyWebResourceType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        identifier: Result<Option<super::ManyIdentifierOrLegalIdentifierType>, String>,
        is_part_of: Result<
            Box<Option<super::ManyLearningAssessmentSpecificationType>>,
            String,
        >,
        language: Result<Option<ObjectOrVector<ConceptType>>, String>,
        mode: Result<Option<ObjectOrVector<ConceptType>>, String>,
        proves: Result<
            Option<super::ManyLearningAchievementSpecificationOrQualificationType>,
            String,
        >,
        specialisation_of: Result<
            Box<Option<super::ManyLearningAssessmentSpecificationType>>,
            String,
        >,
        status: Result<Option<super::StringType>, String>,
        supplementary_document: Result<Option<super::ManyWebResourceType>, String>,
        title: Result<super::ManyLangStringType, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for LearningAssessmentSpecificationType {
        fn default() -> Self {
            Self {
                additional_note: Ok(Default::default()),
                alt_label: Ok(Default::default()),
                category: Ok(Default::default()),
                date_modified: Ok(Default::default()),
                dc_type: Ok(Default::default()),
                description: Ok(Default::default()),
                generalisation_of: Ok(Default::default()),
                grading_scheme: Ok(Default::default()),
                has_part: Ok(Default::default()),
                homepage: Ok(Default::default()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                is_part_of: Ok(Default::default()),
                language: Ok(Default::default()),
                mode: Ok(Default::default()),
                proves: Ok(Default::default()),
                specialisation_of: Ok(Default::default()),
                status: Ok(Default::default()),
                supplementary_document: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl LearningAssessmentSpecificationType {
        pub fn additional_note<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyNoteType>>,
            T::Error: std::fmt::Display,
        {
            self.additional_note = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for additional_note: {}", e)
                });
            self
        }
        pub fn alt_label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.alt_label = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for alt_label: {}", e)
                });
            self
        }
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for category: {}", e)
                });
            self
        }
        pub fn date_modified<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.date_modified = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for date_modified: {}", e)
                });
            self
        }
        pub fn dc_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ConceptType>>,
            T::Error: std::fmt::Display,
        {
            self.dc_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dc_type: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn generalisation_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Box<Option<super::ManyLearningAssessmentSpecificationType>>,
            >,
            T::Error: std::fmt::Display,
        {
            self.generalisation_of = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for generalisation_of: {}", e
                    )
                });
            self
        }
        pub fn grading_scheme<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GradingSchemeType>>,
            T::Error: std::fmt::Display,
        {
            self.grading_scheme = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for grading_scheme: {}", e)
                });
            self
        }
        pub fn has_part<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Box<Option<super::ManyLearningAssessmentSpecificationType>>,
            >,
            T::Error: std::fmt::Display,
        {
            self.has_part = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for has_part: {}", e)
                });
            self
        }
        pub fn homepage<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.homepage = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for homepage: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyIdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn is_part_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Box<Option<super::ManyLearningAssessmentSpecificationType>>,
            >,
            T::Error: std::fmt::Display,
        {
            self.is_part_of = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_part_of: {}", e)
                });
            self
        }
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.language = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for language: {}", e)
                });
            self
        }
        pub fn mode<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mode: {}", e));
            self
        }
        pub fn proves<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::ManyLearningAchievementSpecificationOrQualificationType>,
            >,
            T::Error: std::fmt::Display,
        {
            self.proves = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for proves: {}", e)
                });
            self
        }
        pub fn specialisation_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Box<Option<super::ManyLearningAssessmentSpecificationType>>,
            >,
            T::Error: std::fmt::Display,
        {
            self.specialisation_of = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for specialisation_of: {}", e
                    )
                });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringType>>,
            T::Error: std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status: {}", e)
                });
            self
        }
        pub fn supplementary_document<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.supplementary_document = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for supplementary_document: {}",
                        e
                    )
                });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyLangStringType>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for title: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<LearningAssessmentSpecificationType>
    for super::LearningAssessmentSpecificationType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LearningAssessmentSpecificationType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_note: value.additional_note?,
                alt_label: value.alt_label?,
                category: value.category?,
                date_modified: value.date_modified?,
                dc_type: value.dc_type?,
                description: value.description?,
                generalisation_of: value.generalisation_of?,
                grading_scheme: value.grading_scheme?,
                has_part: value.has_part?,
                homepage: value.homepage?,
                id: value.id?,
                identifier: value.identifier?,
                is_part_of: value.is_part_of?,
                language: value.language?,
                mode: value.mode?,
                proves: value.proves?,
                specialisation_of: value.specialisation_of?,
                status: value.status?,
                supplementary_document: value.supplementary_document?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::LearningAssessmentSpecificationType>
    for LearningAssessmentSpecificationType {
        fn from(value: super::LearningAssessmentSpecificationType) -> Self {
            Self {
                additional_note: Ok(value.additional_note),
                alt_label: Ok(value.alt_label),
                category: Ok(value.category),
                date_modified: Ok(value.date_modified),
                dc_type: Ok(value.dc_type),
                description: Ok(value.description),
                generalisation_of: Ok(value.generalisation_of),
                grading_scheme: Ok(value.grading_scheme),
                has_part: Ok(value.has_part),
                homepage: Ok(value.homepage),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                is_part_of: Ok(value.is_part_of),
                language: Ok(value.language),
                mode: Ok(value.mode),
                proves: Ok(value.proves),
                specialisation_of: Ok(value.specialisation_of),
                status: Ok(value.status),
                supplementary_document: Ok(value.supplementary_document),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LearningAssessmentType {
        additional_note: Result<Option<super::ManyNoteType>, String>,
        assessed_by: Result<Option<super::ManyAgentOrPersonOrOrganisationType>, String>,
        awarded_by: Result<Box<super::AwardingProcessType>, String>,
        date_issued: Result<Option<super::DateTimeType>, String>,
        dc_type: Result<Option<ObjectOrVector<ConceptType>>, String>,
        description: Result<Option<super::ManyLangStringType>, String>,
        grade: Result<super::NoteType, String>,
        grade_status: Result<Option<super::ConceptType>, String>,
        has_part: Result<Box<Option<super::ManyLearningAssessmentType>>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        id_verification: Result<Option<super::ConceptType>, String>,
        identifier: Result<Option<super::ManyIdentifierOrLegalIdentifierType>, String>,
        is_part_of: Result<Box<Option<super::ManyLearningAssessmentType>>, String>,
        location: Result<Option<super::LocationType>, String>,
        proves: Result<Option<super::ManyLearningAchievementType>, String>,
        result_distribution: Result<Option<super::ResultDistributionType>, String>,
        shortened_grading: Result<Option<super::ShortenedGradingType>, String>,
        specified_by: Result<
            Option<super::ManyLearningAssessmentSpecificationType>,
            String,
        >,
        supplementary_document: Result<Option<super::ManyWebResourceType>, String>,
        title: Result<super::ManyLangStringType, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for LearningAssessmentType {
        fn default() -> Self {
            Self {
                additional_note: Ok(Default::default()),
                assessed_by: Ok(Default::default()),
                awarded_by: Err("no value supplied for awarded_by".to_string()),
                date_issued: Ok(Default::default()),
                dc_type: Ok(Default::default()),
                description: Ok(Default::default()),
                grade: Err("no value supplied for grade".to_string()),
                grade_status: Ok(Default::default()),
                has_part: Ok(Default::default()),
                id: Ok(Default::default()),
                id_verification: Ok(Default::default()),
                identifier: Ok(Default::default()),
                is_part_of: Ok(Default::default()),
                location: Ok(Default::default()),
                proves: Ok(Default::default()),
                result_distribution: Ok(Default::default()),
                shortened_grading: Ok(Default::default()),
                specified_by: Ok(Default::default()),
                supplementary_document: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl LearningAssessmentType {
        pub fn additional_note<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyNoteType>>,
            T::Error: std::fmt::Display,
        {
            self.additional_note = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for additional_note: {}", e)
                });
            self
        }
        pub fn assessed_by<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyAgentOrPersonOrOrganisationType>>,
            T::Error: std::fmt::Display,
        {
            self.assessed_by = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for assessed_by: {}", e)
                });
            self
        }
        pub fn awarded_by<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<super::AwardingProcessType>>,
            T::Error: std::fmt::Display,
        {
            self.awarded_by = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for awarded_by: {}", e)
                });
            self
        }
        pub fn date_issued<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.date_issued = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for date_issued: {}", e)
                });
            self
        }
        pub fn dc_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.dc_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dc_type: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn grade<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::NoteType>,
            T::Error: std::fmt::Display,
        {
            self.grade = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for grade: {}", e)
                });
            self
        }
        pub fn grade_status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ConceptType>>,
            T::Error: std::fmt::Display,
        {
            self.grade_status = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for grade_status: {}", e)
                });
            self
        }
        pub fn has_part<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<Option<super::ManyLearningAssessmentType>>>,
            T::Error: std::fmt::Display,
        {
            self.has_part = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for has_part: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn id_verification<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ConceptType>>,
            T::Error: std::fmt::Display,
        {
            self.id_verification = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for id_verification: {}", e)
                });
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyIdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn is_part_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<Option<super::ManyLearningAssessmentType>>>,
            T::Error: std::fmt::Display,
        {
            self.is_part_of = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_part_of: {}", e)
                });
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LocationType>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for location: {}", e)
                });
            self
        }
        pub fn proves<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLearningAchievementType>>,
            T::Error: std::fmt::Display,
        {
            self.proves = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for proves: {}", e)
                });
            self
        }
        pub fn result_distribution<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ResultDistributionType>>,
            T::Error: std::fmt::Display,
        {
            self.result_distribution = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for result_distribution: {}", e
                    )
                });
            self
        }
        pub fn shortened_grading<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ShortenedGradingType>>,
            T::Error: std::fmt::Display,
        {
            self.shortened_grading = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for shortened_grading: {}", e
                    )
                });
            self
        }
        pub fn specified_by<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::ManyLearningAssessmentSpecificationType>,
            >,
            T::Error: std::fmt::Display,
        {
            self.specified_by = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for specified_by: {}", e)
                });
            self
        }
        pub fn supplementary_document<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.supplementary_document = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for supplementary_document: {}",
                        e
                    )
                });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyLangStringType>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for title: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<LearningAssessmentType>
    for super::LearningAssessmentType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LearningAssessmentType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_note: value.additional_note?,
                assessed_by: value.assessed_by?,
                awarded_by: value.awarded_by?,
                date_issued: value.date_issued?,
                dc_type: value.dc_type?,
                description: value.description?,
                grade: value.grade?,
                grade_status: value.grade_status?,
                has_part: value.has_part?,
                id: value.id?,
                id_verification: value.id_verification?,
                identifier: value.identifier?,
                is_part_of: value.is_part_of?,
                location: value.location?,
                proves: value.proves?,
                result_distribution: value.result_distribution?,
                shortened_grading: value.shortened_grading?,
                specified_by: value.specified_by?,
                supplementary_document: value.supplementary_document?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::LearningAssessmentType> for LearningAssessmentType {
        fn from(value: super::LearningAssessmentType) -> Self {
            Self {
                additional_note: Ok(value.additional_note),
                assessed_by: Ok(value.assessed_by),
                awarded_by: Ok(value.awarded_by),
                date_issued: Ok(value.date_issued),
                dc_type: Ok(value.dc_type),
                description: Ok(value.description),
                grade: Ok(value.grade),
                grade_status: Ok(value.grade_status),
                has_part: Ok(value.has_part),
                id: Ok(value.id),
                id_verification: Ok(value.id_verification),
                identifier: Ok(value.identifier),
                is_part_of: Ok(value.is_part_of),
                location: Ok(value.location),
                proves: Ok(value.proves),
                result_distribution: Ok(value.result_distribution),
                shortened_grading: Ok(value.shortened_grading),
                specified_by: Ok(value.specified_by),
                supplementary_document: Ok(value.supplementary_document),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LearningEntitlementSpecificationType {
        additional_note: Result<Option<super::ManyNoteType>, String>,
        alt_label: Result<Option<super::ManyLangStringType>, String>,
        category: Result<Option<super::ManyLangStringType>, String>,
        date_modified: Result<Option<super::DateTimeType>, String>,
        dc_type: Result<super::ConceptType, String>,
        description: Result<Option<super::ManyLangStringType>, String>,
        entitled_by: Result<
            Box<Option<super::ManyLearningAchievementSpecificationOrQualificationType>>,
            String,
        >,
        entitlement_status: Result<super::ConceptType, String>,
        generalisation_of: Result<
            Box<Option<super::ManyLearningEntitlementSpecificationType>>,
            String,
        >,
        has_part: Result<
            Box<Option<super::ManyLearningEntitlementSpecificationType>>,
            String,
        >,
        homepage: Result<Option<super::ManyWebResourceType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        identifier: Result<Option<super::ManyIdentifierOrLegalIdentifierType>, String>,
        is_part_of: Result<
            Box<Option<super::ManyLearningEntitlementSpecificationType>>,
            String,
        >,
        limit_jurisdiction: Result<Option<ObjectOrVector<ConceptType>>, String>,
        limit_national_occupation: Result<Option<ObjectOrVector<ConceptType>>, String>,
        limit_occupation: Result<Option<ObjectOrVector<ConceptType>>, String>,
        limit_organisation: Result<Box<Option<super::ManyOrganisationType>>, String>,
        specialisation_of: Result<
            Box<Option<super::ManyLearningEntitlementSpecificationType>>,
            String,
        >,
        status: Result<Option<super::StringType>, String>,
        supplementary_document: Result<Option<super::ManyWebResourceType>, String>,
        title: Result<super::ManyLangStringType, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for LearningEntitlementSpecificationType {
        fn default() -> Self {
            Self {
                additional_note: Ok(Default::default()),
                alt_label: Ok(Default::default()),
                category: Ok(Default::default()),
                date_modified: Ok(Default::default()),
                dc_type: Err("no value supplied for dc_type".to_string()),
                description: Ok(Default::default()),
                entitled_by: Ok(Default::default()),
                entitlement_status: Err(
                    "no value supplied for entitlement_status".to_string(),
                ),
                generalisation_of: Ok(Default::default()),
                has_part: Ok(Default::default()),
                homepage: Ok(Default::default()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                is_part_of: Ok(Default::default()),
                limit_jurisdiction: Ok(Default::default()),
                limit_national_occupation: Ok(Default::default()),
                limit_occupation: Ok(Default::default()),
                limit_organisation: Ok(Default::default()),
                specialisation_of: Ok(Default::default()),
                status: Ok(Default::default()),
                supplementary_document: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl LearningEntitlementSpecificationType {
        pub fn additional_note<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyNoteType>>,
            T::Error: std::fmt::Display,
        {
            self.additional_note = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for additional_note: {}", e)
                });
            self
        }
        pub fn alt_label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.alt_label = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for alt_label: {}", e)
                });
            self
        }
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for category: {}", e)
                });
            self
        }
        pub fn date_modified<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.date_modified = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for date_modified: {}", e)
                });
            self
        }
        pub fn dc_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ConceptType>,
            T::Error: std::fmt::Display,
        {
            self.dc_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dc_type: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn entitled_by<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Box<
                    Option<
                        super::ManyLearningAchievementSpecificationOrQualificationType,
                    >,
                >,
            >,
            T::Error: std::fmt::Display,
        {
            self.entitled_by = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for entitled_by: {}", e)
                });
            self
        }
        pub fn entitlement_status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ConceptType>,
            T::Error: std::fmt::Display,
        {
            self.entitlement_status = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for entitlement_status: {}", e
                    )
                });
            self
        }
        pub fn generalisation_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Box<Option<super::ManyLearningEntitlementSpecificationType>>,
            >,
            T::Error: std::fmt::Display,
        {
            self.generalisation_of = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for generalisation_of: {}", e
                    )
                });
            self
        }
        pub fn has_part<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Box<Option<super::ManyLearningEntitlementSpecificationType>>,
            >,
            T::Error: std::fmt::Display,
        {
            self.has_part = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for has_part: {}", e)
                });
            self
        }
        pub fn homepage<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.homepage = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for homepage: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyIdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn is_part_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Box<Option<super::ManyLearningEntitlementSpecificationType>>,
            >,
            T::Error: std::fmt::Display,
        {
            self.is_part_of = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_part_of: {}", e)
                });
            self
        }
        pub fn limit_jurisdiction<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.limit_jurisdiction = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for limit_jurisdiction: {}", e
                    )
                });
            self
        }
        pub fn limit_national_occupation<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.limit_national_occupation = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for limit_national_occupation: {}",
                        e
                    )
                });
            self
        }
        pub fn limit_occupation<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.limit_occupation = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for limit_occupation: {}", e
                    )
                });
            self
        }
        pub fn limit_organisation<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<Option<super::ManyOrganisationType>>>,
            T::Error: std::fmt::Display,
        {
            self.limit_organisation = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for limit_organisation: {}", e
                    )
                });
            self
        }
        pub fn specialisation_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Box<Option<super::ManyLearningEntitlementSpecificationType>>,
            >,
            T::Error: std::fmt::Display,
        {
            self.specialisation_of = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for specialisation_of: {}", e
                    )
                });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringType>>,
            T::Error: std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status: {}", e)
                });
            self
        }
        pub fn supplementary_document<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.supplementary_document = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for supplementary_document: {}",
                        e
                    )
                });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyLangStringType>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for title: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<LearningEntitlementSpecificationType>
    for super::LearningEntitlementSpecificationType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LearningEntitlementSpecificationType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_note: value.additional_note?,
                alt_label: value.alt_label?,
                category: value.category?,
                date_modified: value.date_modified?,
                dc_type: value.dc_type?,
                description: value.description?,
                entitled_by: value.entitled_by?,
                entitlement_status: value.entitlement_status?,
                generalisation_of: value.generalisation_of?,
                has_part: value.has_part?,
                homepage: value.homepage?,
                id: value.id?,
                identifier: value.identifier?,
                is_part_of: value.is_part_of?,
                limit_jurisdiction: value.limit_jurisdiction?,
                limit_national_occupation: value.limit_national_occupation?,
                limit_occupation: value.limit_occupation?,
                limit_organisation: value.limit_organisation?,
                specialisation_of: value.specialisation_of?,
                status: value.status?,
                supplementary_document: value.supplementary_document?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::LearningEntitlementSpecificationType>
    for LearningEntitlementSpecificationType {
        fn from(value: super::LearningEntitlementSpecificationType) -> Self {
            Self {
                additional_note: Ok(value.additional_note),
                alt_label: Ok(value.alt_label),
                category: Ok(value.category),
                date_modified: Ok(value.date_modified),
                dc_type: Ok(value.dc_type),
                description: Ok(value.description),
                entitled_by: Ok(value.entitled_by),
                entitlement_status: Ok(value.entitlement_status),
                generalisation_of: Ok(value.generalisation_of),
                has_part: Ok(value.has_part),
                homepage: Ok(value.homepage),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                is_part_of: Ok(value.is_part_of),
                limit_jurisdiction: Ok(value.limit_jurisdiction),
                limit_national_occupation: Ok(value.limit_national_occupation),
                limit_occupation: Ok(value.limit_occupation),
                limit_organisation: Ok(value.limit_organisation),
                specialisation_of: Ok(value.specialisation_of),
                status: Ok(value.status),
                supplementary_document: Ok(value.supplementary_document),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LearningEntitlementType {
        additional_note: Result<Option<super::ManyNoteType>, String>,
        awarded_by: Result<Box<super::AwardingProcessType>, String>,
        date_issued: Result<Option<super::DateTimeType>, String>,
        dc_type: Result<Option<ObjectOrVector<ConceptType>>, String>,
        description: Result<Option<super::ManyLangStringType>, String>,
        entitled_by: Result<Box<Option<super::ManyLearningAchievementType>>, String>,
        expiry_date: Result<Option<super::DateTimeType>, String>,
        has_part: Result<Box<Option<super::ManyLearningEntitlementType>>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        identifier: Result<Option<super::ManyIdentifierOrLegalIdentifierType>, String>,
        is_part_of: Result<Box<Option<super::ManyLearningEntitlementType>>, String>,
        specified_by: Result<
            Option<super::ManyLearningEntitlementSpecificationType>,
            String,
        >,
        supplementary_document: Result<Option<super::ManyWebResourceType>, String>,
        title: Result<super::ManyLangStringType, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for LearningEntitlementType {
        fn default() -> Self {
            Self {
                additional_note: Ok(Default::default()),
                awarded_by: Err("no value supplied for awarded_by".to_string()),
                date_issued: Ok(Default::default()),
                dc_type: Ok(Default::default()),
                description: Ok(Default::default()),
                entitled_by: Ok(Default::default()),
                expiry_date: Ok(Default::default()),
                has_part: Ok(Default::default()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                is_part_of: Ok(Default::default()),
                specified_by: Ok(Default::default()),
                supplementary_document: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl LearningEntitlementType {
        pub fn additional_note<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyNoteType>>,
            T::Error: std::fmt::Display,
        {
            self.additional_note = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for additional_note: {}", e)
                });
            self
        }
        pub fn awarded_by<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<super::AwardingProcessType>>,
            T::Error: std::fmt::Display,
        {
            self.awarded_by = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for awarded_by: {}", e)
                });
            self
        }
        pub fn date_issued<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.date_issued = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for date_issued: {}", e)
                });
            self
        }
        pub fn dc_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.dc_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dc_type: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn entitled_by<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<Option<super::ManyLearningAchievementType>>>,
            T::Error: std::fmt::Display,
        {
            self.entitled_by = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for entitled_by: {}", e)
                });
            self
        }
        pub fn expiry_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.expiry_date = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for expiry_date: {}", e)
                });
            self
        }
        pub fn has_part<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<Option<super::ManyLearningEntitlementType>>>,
            T::Error: std::fmt::Display,
        {
            self.has_part = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for has_part: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyIdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn is_part_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<Option<super::ManyLearningEntitlementType>>>,
            T::Error: std::fmt::Display,
        {
            self.is_part_of = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_part_of: {}", e)
                });
            self
        }
        pub fn specified_by<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::ManyLearningEntitlementSpecificationType>,
            >,
            T::Error: std::fmt::Display,
        {
            self.specified_by = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for specified_by: {}", e)
                });
            self
        }
        pub fn supplementary_document<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.supplementary_document = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for supplementary_document: {}",
                        e
                    )
                });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyLangStringType>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for title: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<LearningEntitlementType>
    for super::LearningEntitlementType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LearningEntitlementType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_note: value.additional_note?,
                awarded_by: value.awarded_by?,
                date_issued: value.date_issued?,
                dc_type: value.dc_type?,
                description: value.description?,
                entitled_by: value.entitled_by?,
                expiry_date: value.expiry_date?,
                has_part: value.has_part?,
                id: value.id?,
                identifier: value.identifier?,
                is_part_of: value.is_part_of?,
                specified_by: value.specified_by?,
                supplementary_document: value.supplementary_document?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::LearningEntitlementType> for LearningEntitlementType {
        fn from(value: super::LearningEntitlementType) -> Self {
            Self {
                additional_note: Ok(value.additional_note),
                awarded_by: Ok(value.awarded_by),
                date_issued: Ok(value.date_issued),
                dc_type: Ok(value.dc_type),
                description: Ok(value.description),
                entitled_by: Ok(value.entitled_by),
                expiry_date: Ok(value.expiry_date),
                has_part: Ok(value.has_part),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                is_part_of: Ok(value.is_part_of),
                specified_by: Ok(value.specified_by),
                supplementary_document: Ok(value.supplementary_document),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LearningOpportunityType {
        additional_note: Result<Option<super::ManyNoteType>, String>,
        admission_procedure: Result<Option<super::NoteType>, String>,
        application_deadline: Result<Option<super::ManyDateTimeType>, String>,
        banner_image: Result<Option<super::MediaObjectType>, String>,
        date_modified: Result<Option<super::DateTimeType>, String>,
        dc_type: Result<Option<ObjectOrVector<ConceptType>>, String>,
        default_language: Result<Option<super::ConceptType>, String>,
        description: Result<Option<super::ManyLangStringType>, String>,
        description_html: Result<Option<super::ManyHtmlType>, String>,
        duration: Result<Option<super::DurationType>, String>,
        grant: Result<Option<super::ManyGrantType>, String>,
        has_part: Result<Option<super::ManyLearningOpportunityType>, String>,
        homepage: Result<Option<super::ManyWebResourceType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        identifier: Result<Option<super::ManyIdentifierOrLegalIdentifierType>, String>,
        is_part_of: Result<Option<super::ManyLearningOpportunityType>, String>,
        learning_achievement_specification: Result<
            Option<super::LearningAchievementSpecificationOrQualificationType>,
            String,
        >,
        learning_activity_specification: Result<
            Option<super::LearningActivitySpecificationType>,
            String,
        >,
        learning_schedule: Result<Option<super::ConceptType>, String>,
        location: Result<Option<super::ManyLocationType>, String>,
        mode: Result<Option<ObjectOrVector<ConceptType>>, String>,
        price_detail: Result<Option<super::ManyPriceDetailType>, String>,
        provided_by: Result<Option<Box<super::OrganisationType>>, String>,
        schedule_information: Result<Option<super::NoteType>, String>,
        status: Result<Option<super::StringType>, String>,
        supplementary_document: Result<Option<super::ManyWebResourceType>, String>,
        temporal: Result<Option<super::PeriodOfTimeType>, String>,
        title: Result<super::ManyLangStringType, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for LearningOpportunityType {
        fn default() -> Self {
            Self {
                additional_note: Ok(Default::default()),
                admission_procedure: Ok(Default::default()),
                application_deadline: Ok(Default::default()),
                banner_image: Ok(Default::default()),
                date_modified: Ok(Default::default()),
                dc_type: Ok(Default::default()),
                default_language: Ok(Default::default()),
                description: Ok(Default::default()),
                description_html: Ok(Default::default()),
                duration: Ok(Default::default()),
                grant: Ok(Default::default()),
                has_part: Ok(Default::default()),
                homepage: Ok(Default::default()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                is_part_of: Ok(Default::default()),
                learning_achievement_specification: Ok(Default::default()),
                learning_activity_specification: Ok(Default::default()),
                learning_schedule: Ok(Default::default()),
                location: Ok(Default::default()),
                mode: Ok(Default::default()),
                price_detail: Ok(Default::default()),
                provided_by: Ok(Default::default()),
                schedule_information: Ok(Default::default()),
                status: Ok(Default::default()),
                supplementary_document: Ok(Default::default()),
                temporal: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl LearningOpportunityType {
        pub fn additional_note<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyNoteType>>,
            T::Error: std::fmt::Display,
        {
            self.additional_note = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for additional_note: {}", e)
                });
            self
        }
        pub fn admission_procedure<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::NoteType>>,
            T::Error: std::fmt::Display,
        {
            self.admission_procedure = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for admission_procedure: {}", e
                    )
                });
            self
        }
        pub fn application_deadline<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyDateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.application_deadline = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for application_deadline: {}", e
                    )
                });
            self
        }
        pub fn banner_image<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::MediaObjectType>>,
            T::Error: std::fmt::Display,
        {
            self.banner_image = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for banner_image: {}", e)
                });
            self
        }
        pub fn date_modified<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.date_modified = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for date_modified: {}", e)
                });
            self
        }
        pub fn dc_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.dc_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dc_type: {}", e)
                });
            self
        }
        pub fn default_language<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ConceptType>>,
            T::Error: std::fmt::Display,
        {
            self.default_language = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for default_language: {}", e
                    )
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn description_html<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyHtmlType>>,
            T::Error: std::fmt::Display,
        {
            self.description_html = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for description_html: {}", e
                    )
                });
            self
        }
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DurationType>>,
            T::Error: std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for duration: {}", e)
                });
            self
        }
        pub fn grant<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyGrantType>>,
            T::Error: std::fmt::Display,
        {
            self.grant = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for grant: {}", e)
                });
            self
        }
        pub fn has_part<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLearningOpportunityType>>,
            T::Error: std::fmt::Display,
        {
            self.has_part = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for has_part: {}", e)
                });
            self
        }
        pub fn homepage<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.homepage = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for homepage: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyIdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn is_part_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLearningOpportunityType>>,
            T::Error: std::fmt::Display,
        {
            self.is_part_of = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_part_of: {}", e)
                });
            self
        }
        pub fn learning_achievement_specification<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::LearningAchievementSpecificationOrQualificationType>,
            >,
            T::Error: std::fmt::Display,
        {
            self.learning_achievement_specification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for learning_achievement_specification: {}",
                        e
                    )
                });
            self
        }
        pub fn learning_activity_specification<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LearningActivitySpecificationType>>,
            T::Error: std::fmt::Display,
        {
            self.learning_activity_specification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for learning_activity_specification: {}",
                        e
                    )
                });
            self
        }
        pub fn learning_schedule<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ConceptType>>,
            T::Error: std::fmt::Display,
        {
            self.learning_schedule = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for learning_schedule: {}", e
                    )
                });
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLocationType>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for location: {}", e)
                });
            self
        }
        pub fn mode<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mode: {}", e));
            self
        }
        pub fn price_detail<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyPriceDetailType>>,
            T::Error: std::fmt::Display,
        {
            self.price_detail = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for price_detail: {}", e)
                });
            self
        }
        pub fn provided_by<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Box<super::OrganisationType>>>,
            T::Error: std::fmt::Display,
        {
            self.provided_by = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for provided_by: {}", e)
                });
            self
        }
        pub fn schedule_information<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::NoteType>>,
            T::Error: std::fmt::Display,
        {
            self.schedule_information = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for schedule_information: {}", e
                    )
                });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringType>>,
            T::Error: std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status: {}", e)
                });
            self
        }
        pub fn supplementary_document<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.supplementary_document = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for supplementary_document: {}",
                        e
                    )
                });
            self
        }
        pub fn temporal<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::PeriodOfTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.temporal = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for temporal: {}", e)
                });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyLangStringType>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for title: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<LearningOpportunityType>
    for super::LearningOpportunityType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LearningOpportunityType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_note: value.additional_note?,
                admission_procedure: value.admission_procedure?,
                application_deadline: value.application_deadline?,
                banner_image: value.banner_image?,
                date_modified: value.date_modified?,
                dc_type: value.dc_type?,
                default_language: value.default_language?,
                description: value.description?,
                description_html: value.description_html?,
                duration: value.duration?,
                grant: value.grant?,
                has_part: value.has_part?,
                homepage: value.homepage?,
                id: value.id?,
                identifier: value.identifier?,
                is_part_of: value.is_part_of?,
                learning_achievement_specification: value
                    .learning_achievement_specification?,
                learning_activity_specification: value.learning_activity_specification?,
                learning_schedule: value.learning_schedule?,
                location: value.location?,
                mode: value.mode?,
                price_detail: value.price_detail?,
                provided_by: value.provided_by?,
                schedule_information: value.schedule_information?,
                status: value.status?,
                supplementary_document: value.supplementary_document?,
                temporal: value.temporal?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::LearningOpportunityType> for LearningOpportunityType {
        fn from(value: super::LearningOpportunityType) -> Self {
            Self {
                additional_note: Ok(value.additional_note),
                admission_procedure: Ok(value.admission_procedure),
                application_deadline: Ok(value.application_deadline),
                banner_image: Ok(value.banner_image),
                date_modified: Ok(value.date_modified),
                dc_type: Ok(value.dc_type),
                default_language: Ok(value.default_language),
                description: Ok(value.description),
                description_html: Ok(value.description_html),
                duration: Ok(value.duration),
                grant: Ok(value.grant),
                has_part: Ok(value.has_part),
                homepage: Ok(value.homepage),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                is_part_of: Ok(value.is_part_of),
                learning_achievement_specification: Ok(
                    value.learning_achievement_specification,
                ),
                learning_activity_specification: Ok(
                    value.learning_activity_specification,
                ),
                learning_schedule: Ok(value.learning_schedule),
                location: Ok(value.location),
                mode: Ok(value.mode),
                price_detail: Ok(value.price_detail),
                provided_by: Ok(value.provided_by),
                schedule_information: Ok(value.schedule_information),
                status: Ok(value.status),
                supplementary_document: Ok(value.supplementary_document),
                temporal: Ok(value.temporal),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LearningOutcomeType {
        additional_note: Result<Option<super::ManyNoteType>, String>,
        dc_type: Result<Option<super::ConceptType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        identifier: Result<Option<super::IdentifierOrLegalIdentifierType>, String>,
        related_esco_skill: Result<Option<ObjectOrVector<ConceptType>>, String>,
        related_skill: Result<Option<ObjectOrVector<ConceptType>>, String>,
        reusability_level: Result<Option<super::ConceptType>, String>,
        title: Result<super::ManyLangStringType, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for LearningOutcomeType {
        fn default() -> Self {
            Self {
                additional_note: Ok(Default::default()),
                dc_type: Ok(Default::default()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                related_esco_skill: Ok(Default::default()),
                related_skill: Ok(Default::default()),
                reusability_level: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl LearningOutcomeType {
        pub fn additional_note<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyNoteType>>,
            T::Error: std::fmt::Display,
        {
            self.additional_note = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for additional_note: {}", e)
                });
            self
        }
        pub fn dc_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ConceptType>>,
            T::Error: std::fmt::Display,
        {
            self.dc_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dc_type: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn related_esco_skill<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.related_esco_skill = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for related_esco_skill: {}", e
                    )
                });
            self
        }
        pub fn related_skill<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.related_skill = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for related_skill: {}", e)
                });
            self
        }
        pub fn reusability_level<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ConceptType>>,
            T::Error: std::fmt::Display,
        {
            self.reusability_level = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for reusability_level: {}", e
                    )
                });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyLangStringType>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for title: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<LearningOutcomeType> for super::LearningOutcomeType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LearningOutcomeType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_note: value.additional_note?,
                dc_type: value.dc_type?,
                id: value.id?,
                identifier: value.identifier?,
                related_esco_skill: value.related_esco_skill?,
                related_skill: value.related_skill?,
                reusability_level: value.reusability_level?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::LearningOutcomeType> for LearningOutcomeType {
        fn from(value: super::LearningOutcomeType) -> Self {
            Self {
                additional_note: Ok(value.additional_note),
                dc_type: Ok(value.dc_type),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                related_esco_skill: Ok(value.related_esco_skill),
                related_skill: Ok(value.related_skill),
                reusability_level: Ok(value.reusability_level),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LegalIdentifierType {
        creator: Result<Option<super::IriType>, String>,
        date_issued: Result<Option<super::DateTimeType>, String>,
        dc_type: Result<Option<ObjectOrVector<ConceptType>>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        notation: Result<super::LiteralType, String>,
        scheme_agency: Result<Option<super::LangStringType>, String>,
        scheme_id: Result<Option<super::UriType>, String>,
        scheme_name: Result<Option<super::StringType>, String>,
        scheme_version: Result<Option<super::StringType>, String>,
        spatial: Result<super::ConceptType, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for LegalIdentifierType {
        fn default() -> Self {
            Self {
                creator: Ok(Default::default()),
                date_issued: Ok(Default::default()),
                dc_type: Ok(Default::default()),
                id: Ok(Default::default()),
                notation: Err("no value supplied for notation".to_string()),
                scheme_agency: Ok(Default::default()),
                scheme_id: Ok(Default::default()),
                scheme_name: Ok(Default::default()),
                scheme_version: Ok(Default::default()),
                spatial: Err("no value supplied for spatial".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl LegalIdentifierType {
        pub fn creator<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IriType>>,
            T::Error: std::fmt::Display,
        {
            self.creator = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for creator: {}", e)
                });
            self
        }
        pub fn date_issued<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.date_issued = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for date_issued: {}", e)
                });
            self
        }
        pub fn dc_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.dc_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dc_type: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn notation<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::LiteralType>,
            T::Error: std::fmt::Display,
        {
            self.notation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for notation: {}", e)
                });
            self
        }
        pub fn scheme_agency<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.scheme_agency = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for scheme_agency: {}", e)
                });
            self
        }
        pub fn scheme_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::UriType>>,
            T::Error: std::fmt::Display,
        {
            self.scheme_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for scheme_id: {}", e)
                });
            self
        }
        pub fn scheme_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringType>>,
            T::Error: std::fmt::Display,
        {
            self.scheme_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for scheme_name: {}", e)
                });
            self
        }
        pub fn scheme_version<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringType>>,
            T::Error: std::fmt::Display,
        {
            self.scheme_version = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for scheme_version: {}", e)
                });
            self
        }
        pub fn spatial<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ConceptType>,
            T::Error: std::fmt::Display,
        {
            self.spatial = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for spatial: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<LegalIdentifierType> for super::LegalIdentifierType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LegalIdentifierType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                creator: value.creator?,
                date_issued: value.date_issued?,
                dc_type: value.dc_type?,
                id: value.id?,
                notation: value.notation?,
                scheme_agency: value.scheme_agency?,
                scheme_id: value.scheme_id?,
                scheme_name: value.scheme_name?,
                scheme_version: value.scheme_version?,
                spatial: value.spatial?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::LegalIdentifierType> for LegalIdentifierType {
        fn from(value: super::LegalIdentifierType) -> Self {
            Self {
                creator: Ok(value.creator),
                date_issued: Ok(value.date_issued),
                dc_type: Ok(value.dc_type),
                id: Ok(value.id),
                notation: Ok(value.notation),
                scheme_agency: Ok(value.scheme_agency),
                scheme_id: Ok(value.scheme_id),
                scheme_name: Ok(value.scheme_name),
                scheme_version: Ok(value.scheme_version),
                spatial: Ok(value.spatial),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LocationType {
        address: Result<super::ManyAddressType, String>,
        description: Result<Option<super::ManyLangStringType>, String>,
        geographic_name: Result<Option<super::ManyAddressType>, String>,
        geometry: Result<Option<super::ManyGeometryType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        identifier: Result<Option<super::IdentifierOrLegalIdentifierType>, String>,
        spatial_code: Result<Option<ObjectOrVector<ConceptType>>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for LocationType {
        fn default() -> Self {
            Self {
                address: Err("no value supplied for address".to_string()),
                description: Ok(Default::default()),
                geographic_name: Ok(Default::default()),
                geometry: Ok(Default::default()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                spatial_code: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl LocationType {
        pub fn address<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyAddressType>,
            T::Error: std::fmt::Display,
        {
            self.address = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for address: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn geographic_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyAddressType>>,
            T::Error: std::fmt::Display,
        {
            self.geographic_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for geographic_name: {}", e)
                });
            self
        }
        pub fn geometry<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyGeometryType>>,
            T::Error: std::fmt::Display,
        {
            self.geometry = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for geometry: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn spatial_code<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.spatial_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for spatial_code: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<LocationType> for super::LocationType {
        type Error = super::error::ConversionError;
        fn try_from(value: LocationType) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                address: value.address?,
                description: value.description?,
                geographic_name: value.geographic_name?,
                geometry: value.geometry?,
                id: value.id?,
                identifier: value.identifier?,
                spatial_code: value.spatial_code?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::LocationType> for LocationType {
        fn from(value: super::LocationType) -> Self {
            Self {
                address: Ok(value.address),
                description: Ok(value.description),
                geographic_name: Ok(value.geographic_name),
                geometry: Ok(value.geometry),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                spatial_code: Ok(value.spatial_code),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MailboxType {
        id: Result<Option<super::EmailType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for MailboxType {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl MailboxType {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::EmailType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<MailboxType> for super::MailboxType {
        type Error = super::error::ConversionError;
        fn try_from(value: MailboxType) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::MailboxType> for MailboxType {
        fn from(value: super::MailboxType) -> Self {
            Self {
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ManyAgentOrPersonOrOrganisationType {
        subtype_0: Result<Option<Box<super::AgentOrPersonOrOrganisationType>>, String>,
        subtype_1: Result<Option<Vec<super::AgentOrPersonOrOrganisationType>>, String>,
    }
    impl Default for ManyAgentOrPersonOrOrganisationType {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl ManyAgentOrPersonOrOrganisationType {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<Box<super::AgentOrPersonOrOrganisationType>>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_0: {}", e)
                });
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<Vec<super::AgentOrPersonOrOrganisationType>>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_1: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<ManyAgentOrPersonOrOrganisationType>
    for super::ManyAgentOrPersonOrOrganisationType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ManyAgentOrPersonOrOrganisationType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl From<super::ManyAgentOrPersonOrOrganisationType>
    for ManyAgentOrPersonOrOrganisationType {
        fn from(value: super::ManyAgentOrPersonOrOrganisationType) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ManyClaimNodeType {
        subtype_0: Result<Option<super::ClaimNodeType>, String>,
        subtype_1: Result<Option<Vec<super::ClaimNodeType>>, String>,
    }
    impl Default for ManyClaimNodeType {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl ManyClaimNodeType {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ClaimNodeType>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_0: {}", e)
                });
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Vec<super::ClaimNodeType>>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_1: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<ManyClaimNodeType> for super::ManyClaimNodeType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ManyClaimNodeType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl From<super::ManyClaimNodeType> for ManyClaimNodeType {
        fn from(value: super::ManyClaimNodeType) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ManyIdentifierOrLegalIdentifierType {
        subtype_0: Result<Option<super::IdentifierOrLegalIdentifierType>, String>,
        subtype_1: Result<Option<Vec<super::IdentifierOrLegalIdentifierType>>, String>,
    }
    impl Default for ManyIdentifierOrLegalIdentifierType {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl ManyIdentifierOrLegalIdentifierType {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_0: {}", e)
                });
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<Vec<super::IdentifierOrLegalIdentifierType>>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_1: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<ManyIdentifierOrLegalIdentifierType>
    for super::ManyIdentifierOrLegalIdentifierType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ManyIdentifierOrLegalIdentifierType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl From<super::ManyIdentifierOrLegalIdentifierType>
    for ManyIdentifierOrLegalIdentifierType {
        fn from(value: super::ManyIdentifierOrLegalIdentifierType) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ManyLearningAchievementSpecificationOrQualificationType {
        subtype_0: Result<
            Option<super::LearningAchievementSpecificationOrQualificationType>,
            String,
        >,
        subtype_1: Result<
            Option<Vec<super::LearningAchievementSpecificationOrQualificationType>>,
            String,
        >,
    }
    impl Default for ManyLearningAchievementSpecificationOrQualificationType {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl ManyLearningAchievementSpecificationOrQualificationType {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::LearningAchievementSpecificationOrQualificationType>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_0: {}", e)
                });
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<Vec<super::LearningAchievementSpecificationOrQualificationType>>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_1: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<ManyLearningAchievementSpecificationOrQualificationType>
    for super::ManyLearningAchievementSpecificationOrQualificationType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ManyLearningAchievementSpecificationOrQualificationType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl From<super::ManyLearningAchievementSpecificationOrQualificationType>
    for ManyLearningAchievementSpecificationOrQualificationType {
        fn from(
            value: super::ManyLearningAchievementSpecificationOrQualificationType,
        ) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MediaObjectType {
        attachment_type: Result<Option<super::ConceptType>, String>,
        content: Result<super::StringType, String>,
        content_encoding: Result<super::ConceptType, String>,
        content_size: Result<Option<super::IntegerType>, String>,
        content_type: Result<super::ConceptType, String>,
        content_url: Result<Option<super::UriType>, String>,
        description: Result<Option<super::ManyLangStringType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        title: Result<Option<super::ManyLangStringType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for MediaObjectType {
        fn default() -> Self {
            Self {
                attachment_type: Ok(Default::default()),
                content: Err("no value supplied for content".to_string()),
                content_encoding: Err(
                    "no value supplied for content_encoding".to_string(),
                ),
                content_size: Ok(Default::default()),
                content_type: Err("no value supplied for content_type".to_string()),
                content_url: Ok(Default::default()),
                description: Ok(Default::default()),
                id: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl MediaObjectType {
        pub fn attachment_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ConceptType>>,
            T::Error: std::fmt::Display,
        {
            self.attachment_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for attachment_type: {}", e)
                });
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::StringType>,
            T::Error: std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for content: {}", e)
                });
            self
        }
        pub fn content_encoding<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ConceptType>,
            T::Error: std::fmt::Display,
        {
            self.content_encoding = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for content_encoding: {}", e
                    )
                });
            self
        }
        pub fn content_size<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IntegerType>>,
            T::Error: std::fmt::Display,
        {
            self.content_size = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for content_size: {}", e)
                });
            self
        }
        pub fn content_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ConceptType>,
            T::Error: std::fmt::Display,
        {
            self.content_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for content_type: {}", e)
                });
            self
        }
        pub fn content_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::UriType>>,
            T::Error: std::fmt::Display,
        {
            self.content_url = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for content_url: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for title: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<MediaObjectType> for super::MediaObjectType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MediaObjectType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                attachment_type: value.attachment_type?,
                content: value.content?,
                content_encoding: value.content_encoding?,
                content_size: value.content_size?,
                content_type: value.content_type?,
                content_url: value.content_url?,
                description: value.description?,
                id: value.id?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::MediaObjectType> for MediaObjectType {
        fn from(value: super::MediaObjectType) -> Self {
            Self {
                attachment_type: Ok(value.attachment_type),
                content: Ok(value.content),
                content_encoding: Ok(value.content_encoding),
                content_size: Ok(value.content_size),
                content_type: Ok(value.content_type),
                content_url: Ok(value.content_url),
                description: Ok(value.description),
                id: Ok(value.id),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NoteType {
        id: Result<Option<super::GenericIdType>, String>,
        note_format: Result<Option<super::ConceptType>, String>,
        note_literal: Result<super::ManyLangStringType, String>,
        subject: Result<Option<super::ConceptType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for NoteType {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                note_format: Ok(Default::default()),
                note_literal: Err("no value supplied for note_literal".to_string()),
                subject: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl NoteType {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn note_format<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ConceptType>>,
            T::Error: std::fmt::Display,
        {
            self.note_format = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for note_format: {}", e)
                });
            self
        }
        pub fn note_literal<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyLangStringType>,
            T::Error: std::fmt::Display,
        {
            self.note_literal = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for note_literal: {}", e)
                });
            self
        }
        pub fn subject<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ConceptType>>,
            T::Error: std::fmt::Display,
        {
            self.subject = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subject: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<NoteType> for super::NoteType {
        type Error = super::error::ConversionError;
        fn try_from(value: NoteType) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                note_format: value.note_format?,
                note_literal: value.note_literal?,
                subject: value.subject?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::NoteType> for NoteType {
        fn from(value: super::NoteType) -> Self {
            Self {
                id: Ok(value.id),
                note_format: Ok(value.note_format),
                note_literal: Ok(value.note_literal),
                subject: Ok(value.subject),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganisationType {
        accreditation: Result<Option<super::ManyAccreditationType>, String>,
        additional_note: Result<Option<super::ManyNoteType>, String>,
        alt_label: Result<Option<super::ManyLangStringType>, String>,
        contact_point: Result<Option<super::ManyContactPointType>, String>,
        date_modified: Result<Option<super::DateTimeType>, String>,
        dc_type: Result<Option<ObjectOrVector<ConceptType>>, String>,
        e_idas_identifier: Result<Option<super::LegalIdentifierType>, String>,
        group_member_of: Result<Option<super::ManyGroupType>, String>,
        has_member: Result<Option<super::ManyPersonType>, String>,
        has_sub_organization: Result<Box<Option<super::ManyOrganisationType>>, String>,
        homepage: Result<Option<super::ManyWebResourceType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        identifier: Result<Option<super::IdentifierOrLegalIdentifierType>, String>,
        legal_name: Result<super::ManyLangStringType, String>,
        location: Result<super::ManyLocationType, String>,
        logo: Result<Option<super::MediaObjectType>, String>,
        registration: Result<Option<super::LegalIdentifierType>, String>,
        sub_organization_of: Result<Option<Box<super::OrganisationType>>, String>,
        tax_identifier: Result<Option<super::ManyLegalIdentifierType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
        vat_identifier: Result<Option<super::ManyLegalIdentifierType>, String>,
    }
    impl Default for OrganisationType {
        fn default() -> Self {
            Self {
                accreditation: Ok(Default::default()),
                additional_note: Ok(Default::default()),
                alt_label: Ok(Default::default()),
                contact_point: Ok(Default::default()),
                date_modified: Ok(Default::default()),
                dc_type: Ok(Default::default()),
                e_idas_identifier: Ok(Default::default()),
                group_member_of: Ok(Default::default()),
                has_member: Ok(Default::default()),
                has_sub_organization: Ok(Default::default()),
                homepage: Ok(Default::default()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                legal_name: Err("no value supplied for legal_name".to_string()),
                location: Err("no value supplied for location".to_string()),
                logo: Ok(Default::default()),
                registration: Ok(Default::default()),
                sub_organization_of: Ok(Default::default()),
                tax_identifier: Ok(Default::default()),
                type_: Ok(Default::default()),
                vat_identifier: Ok(Default::default()),
            }
        }
    }
    impl OrganisationType {
        pub fn accreditation<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyAccreditationType>>,
            T::Error: std::fmt::Display,
        {
            self.accreditation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for accreditation: {}", e)
                });
            self
        }
        pub fn additional_note<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyNoteType>>,
            T::Error: std::fmt::Display,
        {
            self.additional_note = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for additional_note: {}", e)
                });
            self
        }
        pub fn alt_label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.alt_label = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for alt_label: {}", e)
                });
            self
        }
        pub fn contact_point<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyContactPointType>>,
            T::Error: std::fmt::Display,
        {
            self.contact_point = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for contact_point: {}", e)
                });
            self
        }
        pub fn date_modified<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.date_modified = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for date_modified: {}", e)
                });
            self
        }
        pub fn dc_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.dc_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dc_type: {}", e)
                });
            self
        }
        pub fn e_idas_identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.e_idas_identifier = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for e_idas_identifier: {}", e
                    )
                });
            self
        }
        pub fn group_member_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyGroupType>>,
            T::Error: std::fmt::Display,
        {
            self.group_member_of = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for group_member_of: {}", e)
                });
            self
        }
        pub fn has_member<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyPersonType>>,
            T::Error: std::fmt::Display,
        {
            self.has_member = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for has_member: {}", e)
                });
            self
        }
        pub fn has_sub_organization<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<Option<super::ManyOrganisationType>>>,
            T::Error: std::fmt::Display,
        {
            self.has_sub_organization = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for has_sub_organization: {}", e
                    )
                });
            self
        }
        pub fn homepage<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.homepage = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for homepage: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn legal_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyLangStringType>,
            T::Error: std::fmt::Display,
        {
            self.legal_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for legal_name: {}", e)
                });
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyLocationType>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for location: {}", e)
                });
            self
        }
        pub fn logo<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::MediaObjectType>>,
            T::Error: std::fmt::Display,
        {
            self.logo = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logo: {}", e));
            self
        }
        pub fn registration<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.registration = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for registration: {}", e)
                });
            self
        }
        pub fn sub_organization_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Box<super::OrganisationType>>>,
            T::Error: std::fmt::Display,
        {
            self.sub_organization_of = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for sub_organization_of: {}", e
                    )
                });
            self
        }
        pub fn tax_identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.tax_identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for tax_identifier: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn vat_identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.vat_identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for vat_identifier: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<OrganisationType> for super::OrganisationType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrganisationType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                accreditation: value.accreditation?,
                additional_note: value.additional_note?,
                alt_label: value.alt_label?,
                contact_point: value.contact_point?,
                date_modified: value.date_modified?,
                dc_type: value.dc_type?,
                e_idas_identifier: value.e_idas_identifier?,
                group_member_of: value.group_member_of?,
                has_member: value.has_member?,
                has_sub_organization: value.has_sub_organization?,
                homepage: value.homepage?,
                id: value.id?,
                identifier: value.identifier?,
                legal_name: value.legal_name?,
                location: value.location?,
                logo: value.logo?,
                registration: value.registration?,
                sub_organization_of: value.sub_organization_of?,
                tax_identifier: value.tax_identifier?,
                type_: value.type_?,
                vat_identifier: value.vat_identifier?,
            })
        }
    }
    impl From<super::OrganisationType> for OrganisationType {
        fn from(value: super::OrganisationType) -> Self {
            Self {
                accreditation: Ok(value.accreditation),
                additional_note: Ok(value.additional_note),
                alt_label: Ok(value.alt_label),
                contact_point: Ok(value.contact_point),
                date_modified: Ok(value.date_modified),
                dc_type: Ok(value.dc_type),
                e_idas_identifier: Ok(value.e_idas_identifier),
                group_member_of: Ok(value.group_member_of),
                has_member: Ok(value.has_member),
                has_sub_organization: Ok(value.has_sub_organization),
                homepage: Ok(value.homepage),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                legal_name: Ok(value.legal_name),
                location: Ok(value.location),
                logo: Ok(value.logo),
                registration: Ok(value.registration),
                sub_organization_of: Ok(value.sub_organization_of),
                tax_identifier: Ok(value.tax_identifier),
                type_: Ok(value.type_),
                vat_identifier: Ok(value.vat_identifier),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PeriodOfTimeType {
        end_date: Result<Option<super::DateTimeType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        pref_label: Result<Option<super::ManyLangStringType>, String>,
        start_date: Result<Option<super::DateTimeType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for PeriodOfTimeType {
        fn default() -> Self {
            Self {
                end_date: Ok(Default::default()),
                id: Ok(Default::default()),
                pref_label: Ok(Default::default()),
                start_date: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl PeriodOfTimeType {
        pub fn end_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.end_date = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for end_date: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn pref_label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.pref_label = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for pref_label: {}", e)
                });
            self
        }
        pub fn start_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.start_date = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for start_date: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<PeriodOfTimeType> for super::PeriodOfTimeType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PeriodOfTimeType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                end_date: value.end_date?,
                id: value.id?,
                pref_label: value.pref_label?,
                start_date: value.start_date?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::PeriodOfTimeType> for PeriodOfTimeType {
        fn from(value: super::PeriodOfTimeType) -> Self {
            Self {
                end_date: Ok(value.end_date),
                id: Ok(value.id),
                pref_label: Ok(value.pref_label),
                start_date: Ok(value.start_date),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PersonType {
        birth_name: Result<Option<super::ManyLangStringType>, String>,
        citizenship_country: Result<Option<ObjectOrVector<ConceptType>>, String>,
        contact_point: Result<Option<super::ManyContactPointType>, String>,
        date_modified: Result<Option<super::DateTimeType>, String>,
        date_of_birth: Result<Option<super::DateTimeType>, String>,
        family_name: Result<Option<super::LangStringType>, String>,
        full_name: Result<Option<super::LangStringType>, String>,
        gender: Result<Option<super::ConceptType>, String>,
        given_name: Result<Option<super::LangStringType>, String>,
        group_member_of: Result<Option<super::ManyGroupType>, String>,
        has_claim: Result<Option<super::ManyClaimNodeType>, String>,
        has_credential: Result<Option<super::ManyEuropeanDigitalCredentialType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        identifier: Result<Option<super::IdentifierOrLegalIdentifierType>, String>,
        location: Result<Option<super::LocationType>, String>,
        member_of: Result<Box<Option<super::ManyOrganisationType>>, String>,
        national_id: Result<Option<super::LegalIdentifierType>, String>,
        patronymic_name: Result<Option<super::ManyLangStringType>, String>,
        place_of_birth: Result<Option<super::LocationType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for PersonType {
        fn default() -> Self {
            Self {
                birth_name: Ok(Default::default()),
                citizenship_country: Ok(Default::default()),
                contact_point: Ok(Default::default()),
                date_modified: Ok(Default::default()),
                date_of_birth: Ok(Default::default()),
                family_name: Ok(Default::default()),
                full_name: Ok(Default::default()),
                gender: Ok(Default::default()),
                given_name: Ok(Default::default()),
                group_member_of: Ok(Default::default()),
                has_claim: Ok(Default::default()),
                has_credential: Ok(Default::default()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                location: Ok(Default::default()),
                member_of: Ok(Default::default()),
                national_id: Ok(Default::default()),
                patronymic_name: Ok(Default::default()),
                place_of_birth: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl PersonType {
        pub fn birth_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.birth_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for birth_name: {}", e)
                });
            self
        }
        pub fn citizenship_country<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.citizenship_country = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for citizenship_country: {}", e
                    )
                });
            self
        }
        pub fn contact_point<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.contact_point = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for contact_point: {}", e)
                });
            self
        }
        pub fn date_modified<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.date_modified = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for date_modified: {}", e)
                });
            self
        }
        pub fn date_of_birth<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.date_of_birth = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for date_of_birth: {}", e)
                });
            self
        }
        pub fn family_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.family_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for family_name: {}", e)
                });
            self
        }
        pub fn full_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.full_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for full_name: {}", e)
                });
            self
        }
        pub fn gender<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ConceptType>>,
            T::Error: std::fmt::Display,
        {
            self.gender = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for gender: {}", e)
                });
            self
        }
        pub fn given_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.given_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for given_name: {}", e)
                });
            self
        }
        pub fn group_member_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyGroupType>>,
            T::Error: std::fmt::Display,
        {
            self.group_member_of = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for group_member_of: {}", e)
                });
            self
        }
        pub fn has_claim<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyClaimNodeType>>,
            T::Error: std::fmt::Display,
        {
            self.has_claim = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for has_claim: {}", e)
                });
            self
        }
        pub fn has_credential<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyEuropeanDigitalCredentialType>>,
            T::Error: std::fmt::Display,
        {
            self.has_credential = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for has_credential: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LocationType>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for location: {}", e)
                });
            self
        }
        pub fn member_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<Option<super::ManyOrganisationType>>>,
            T::Error: std::fmt::Display,
        {
            self.member_of = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for member_of: {}", e)
                });
            self
        }
        pub fn national_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.national_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for national_id: {}", e)
                });
            self
        }
        pub fn patronymic_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.patronymic_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for patronymic_name: {}", e)
                });
            self
        }
        pub fn place_of_birth<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LocationType>>,
            T::Error: std::fmt::Display,
        {
            self.place_of_birth = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for place_of_birth: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<PersonType> for super::PersonType {
        type Error = super::error::ConversionError;
        fn try_from(value: PersonType) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                birth_name: value.birth_name?,
                citizenship_country: value.citizenship_country?,
                contact_point: value.contact_point?,
                date_modified: value.date_modified?,
                date_of_birth: value.date_of_birth?,
                family_name: value.family_name?,
                full_name: value.full_name?,
                gender: value.gender?,
                given_name: value.given_name?,
                group_member_of: value.group_member_of?,
                has_claim: value.has_claim?,
                has_credential: value.has_credential?,
                id: value.id?,
                identifier: value.identifier?,
                location: value.location?,
                member_of: value.member_of?,
                national_id: value.national_id?,
                patronymic_name: value.patronymic_name?,
                place_of_birth: value.place_of_birth?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::PersonType> for PersonType {
        fn from(value: super::PersonType) -> Self {
            Self {
                birth_name: Ok(value.birth_name),
                citizenship_country: Ok(value.citizenship_country),
                contact_point: Ok(value.contact_point),
                date_modified: Ok(value.date_modified),
                date_of_birth: Ok(value.date_of_birth),
                family_name: Ok(value.family_name),
                full_name: Ok(value.full_name),
                gender: Ok(value.gender),
                given_name: Ok(value.given_name),
                group_member_of: Ok(value.group_member_of),
                has_claim: Ok(value.has_claim),
                has_credential: Ok(value.has_credential),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                location: Ok(value.location),
                member_of: Ok(value.member_of),
                national_id: Ok(value.national_id),
                patronymic_name: Ok(value.patronymic_name),
                place_of_birth: Ok(value.place_of_birth),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PhoneType {
        area_dialing: Result<Option<super::StringType>, String>,
        country_dialing: Result<Option<super::StringType>, String>,
        dial_number: Result<Option<super::StringType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        phone_number: Result<Option<super::StringType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for PhoneType {
        fn default() -> Self {
            Self {
                area_dialing: Ok(Default::default()),
                country_dialing: Ok(Default::default()),
                dial_number: Ok(Default::default()),
                id: Ok(Default::default()),
                phone_number: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl PhoneType {
        pub fn area_dialing<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringType>>,
            T::Error: std::fmt::Display,
        {
            self.area_dialing = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for area_dialing: {}", e)
                });
            self
        }
        pub fn country_dialing<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringType>>,
            T::Error: std::fmt::Display,
        {
            self.country_dialing = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for country_dialing: {}", e)
                });
            self
        }
        pub fn dial_number<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringType>>,
            T::Error: std::fmt::Display,
        {
            self.dial_number = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dial_number: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn phone_number<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringType>>,
            T::Error: std::fmt::Display,
        {
            self.phone_number = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for phone_number: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<PhoneType> for super::PhoneType {
        type Error = super::error::ConversionError;
        fn try_from(value: PhoneType) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                area_dialing: value.area_dialing?,
                country_dialing: value.country_dialing?,
                dial_number: value.dial_number?,
                id: value.id?,
                phone_number: value.phone_number?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::PhoneType> for PhoneType {
        fn from(value: super::PhoneType) -> Self {
            Self {
                area_dialing: Ok(value.area_dialing),
                country_dialing: Ok(value.country_dialing),
                dial_number: Ok(value.dial_number),
                id: Ok(value.id),
                phone_number: Ok(value.phone_number),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PriceDetailType {
        additional_note: Result<Option<super::ManyNoteType>, String>,
        amount: Result<Option<super::AmountType>, String>,
        description: Result<Option<super::ManyLangStringType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        identifier: Result<Option<super::ManyIdentifierOrLegalIdentifierType>, String>,
        pref_label: Result<Option<super::ManyLangStringType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for PriceDetailType {
        fn default() -> Self {
            Self {
                additional_note: Ok(Default::default()),
                amount: Ok(Default::default()),
                description: Ok(Default::default()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                pref_label: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl PriceDetailType {
        pub fn additional_note<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyNoteType>>,
            T::Error: std::fmt::Display,
        {
            self.additional_note = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for additional_note: {}", e)
                });
            self
        }
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AmountType>>,
            T::Error: std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for amount: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyIdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn pref_label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.pref_label = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for pref_label: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<PriceDetailType> for super::PriceDetailType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PriceDetailType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_note: value.additional_note?,
                amount: value.amount?,
                description: value.description?,
                id: value.id?,
                identifier: value.identifier?,
                pref_label: value.pref_label?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::PriceDetailType> for PriceDetailType {
        fn from(value: super::PriceDetailType) -> Self {
            Self {
                additional_note: Ok(value.additional_note),
                amount: Ok(value.amount),
                description: Ok(value.description),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                pref_label: Ok(value.pref_label),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProofType {
        id: Result<Option<super::GenericIdType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for ProofType {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl ProofType {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<ProofType> for super::ProofType {
        type Error = super::error::ConversionError;
        fn try_from(value: ProofType) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::ProofType> for ProofType {
        fn from(value: super::ProofType) -> Self {
            Self {
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct QualificationType {
        accreditation: Result<Option<super::ManyAccreditationType>, String>,
        additional_note: Result<Option<super::ManyNoteType>, String>,
        alt_label: Result<Option<super::ManyLangStringType>, String>,
        awarding_opportunity: Result<Option<super::ManyAwardingOpportunityType>, String>,
        category: Result<Option<super::ManyLangStringType>, String>,
        credit_point: Result<Option<super::ManyCreditPointType>, String>,
        date_modified: Result<Option<super::DateTimeType>, String>,
        dc_type: Result<Option<ObjectOrVector<ConceptType>>, String>,
        description: Result<Option<super::ManyLangStringType>, String>,
        education_level: Result<Option<ObjectOrVector<ConceptType>>, String>,
        education_subject: Result<Option<ObjectOrVector<ConceptType>>, String>,
        entitles_to: Result<
            Option<super::ManyLearningEntitlementSpecificationType>,
            String,
        >,
        entry_requirement: Result<Option<super::NoteType>, String>,
        eqf_level: Result<Option<super::ConceptType>, String>,
        generalisation_of: Result<Option<super::ManyQualificationType>, String>,
        has_part: Result<Option<super::ManyQualificationType>, String>,
        homepage: Result<Option<super::ManyWebResourceType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        identifier: Result<Option<super::IdentifierOrLegalIdentifierType>, String>,
        influenced_by: Result<
            Option<super::ManyLearningActivitySpecificationType>,
            String,
        >,
        is_part_of: Result<Option<super::ManyQualificationType>, String>,
        is_partial_qualification: Result<Option<super::BooleanType>, String>,
        language: Result<Option<ObjectOrVector<ConceptType>>, String>,
        learning_outcome: Result<Option<super::ManyLearningOutcomeType>, String>,
        learning_outcome_summary: Result<Option<super::NoteType>, String>,
        learning_setting: Result<Option<super::ConceptType>, String>,
        maximum_duration: Result<Option<super::DurationType>, String>,
        mode: Result<Option<ObjectOrVector<ConceptType>>, String>,
        nqf_level: Result<Option<ObjectOrVector<ConceptType>>, String>,
        proven_by: Result<
            Box<Option<super::ManyLearningAssessmentSpecificationType>>,
            String,
        >,
        qualification_code: Result<Option<ObjectOrVector<ConceptType>>, String>,
        specialisation_of: Result<Option<super::ManyQualificationType>, String>,
        status: Result<Option<super::StringType>, String>,
        supplementary_document: Result<Option<super::ManyWebResourceType>, String>,
        target_group: Result<Option<ObjectOrVector<ConceptType>>, String>,
        thematic_area: Result<Option<ObjectOrVector<ConceptType>>, String>,
        title: Result<super::ManyLangStringType, String>,
        type_: Result<Option<serde_json::Value>, String>,
        volume_of_learning: Result<Option<super::DurationType>, String>,
    }
    impl Default for QualificationType {
        fn default() -> Self {
            Self {
                accreditation: Ok(Default::default()),
                additional_note: Ok(Default::default()),
                alt_label: Ok(Default::default()),
                awarding_opportunity: Ok(Default::default()),
                category: Ok(Default::default()),
                credit_point: Ok(Default::default()),
                date_modified: Ok(Default::default()),
                dc_type: Ok(Default::default()),
                description: Ok(Default::default()),
                education_level: Ok(Default::default()),
                education_subject: Ok(Default::default()),
                entitles_to: Ok(Default::default()),
                entry_requirement: Ok(Default::default()),
                eqf_level: Ok(Default::default()),
                generalisation_of: Ok(Default::default()),
                has_part: Ok(Default::default()),
                homepage: Ok(Default::default()),
                id: Ok(Default::default()),
                identifier: Ok(Default::default()),
                influenced_by: Ok(Default::default()),
                is_part_of: Ok(Default::default()),
                is_partial_qualification: Ok(Default::default()),
                language: Ok(Default::default()),
                learning_outcome: Ok(Default::default()),
                learning_outcome_summary: Ok(Default::default()),
                learning_setting: Ok(Default::default()),
                maximum_duration: Ok(Default::default()),
                mode: Ok(Default::default()),
                nqf_level: Ok(Default::default()),
                proven_by: Ok(Default::default()),
                qualification_code: Ok(Default::default()),
                specialisation_of: Ok(Default::default()),
                status: Ok(Default::default()),
                supplementary_document: Ok(Default::default()),
                target_group: Ok(Default::default()),
                thematic_area: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                type_: Ok(Default::default()),
                volume_of_learning: Ok(Default::default()),
            }
        }
    }
    impl QualificationType {
        pub fn accreditation<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyAccreditationType>>,
            T::Error: std::fmt::Display,
        {
            self.accreditation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for accreditation: {}", e)
                });
            self
        }
        pub fn additional_note<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyNoteType>>,
            T::Error: std::fmt::Display,
        {
            self.additional_note = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for additional_note: {}", e)
                });
            self
        }
        pub fn alt_label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.alt_label = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for alt_label: {}", e)
                });
            self
        }
        pub fn awarding_opportunity<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyAwardingOpportunityType>>,
            T::Error: std::fmt::Display,
        {
            self.awarding_opportunity = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for awarding_opportunity: {}", e
                    )
                });
            self
        }
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for category: {}", e)
                });
            self
        }
        pub fn credit_point<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyCreditPointType>>,
            T::Error: std::fmt::Display,
        {
            self.credit_point = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for credit_point: {}", e)
                });
            self
        }
        pub fn date_modified<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeType>>,
            T::Error: std::fmt::Display,
        {
            self.date_modified = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for date_modified: {}", e)
                });
            self
        }
        pub fn dc_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.dc_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dc_type: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn education_level<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.education_level = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for education_level: {}", e)
                });
            self
        }
        pub fn education_subject<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.education_subject = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for education_subject: {}", e
                    )
                });
            self
        }
        pub fn entitles_to<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::ManyLearningEntitlementSpecificationType>,
            >,
            T::Error: std::fmt::Display,
        {
            self.entitles_to = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for entitles_to: {}", e)
                });
            self
        }
        pub fn entry_requirement<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::NoteType>>,
            T::Error: std::fmt::Display,
        {
            self.entry_requirement = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for entry_requirement: {}", e
                    )
                });
            self
        }
        pub fn eqf_level<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ConceptType>>,
            T::Error: std::fmt::Display,
        {
            self.eqf_level = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for eqf_level: {}", e)
                });
            self
        }
        pub fn generalisation_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyQualificationType>>,
            T::Error: std::fmt::Display,
        {
            self.generalisation_of = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for generalisation_of: {}", e
                    )
                });
            self
        }
        pub fn has_part<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyQualificationType>>,
            T::Error: std::fmt::Display,
        {
            self.has_part = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for has_part: {}", e)
                });
            self
        }
        pub fn homepage<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.homepage = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for homepage: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IdentifierOrLegalIdentifierType>>,
            T::Error: std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identifier: {}", e)
                });
            self
        }
        pub fn influenced_by<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::ManyLearningActivitySpecificationType>,
            >,
            T::Error: std::fmt::Display,
        {
            self.influenced_by = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for influenced_by: {}", e)
                });
            self
        }
        pub fn is_part_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyQualificationType>>,
            T::Error: std::fmt::Display,
        {
            self.is_part_of = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_part_of: {}", e)
                });
            self
        }
        pub fn is_partial_qualification<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::BooleanType>>,
            T::Error: std::fmt::Display,
        {
            self.is_partial_qualification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for is_partial_qualification: {}",
                        e
                    )
                });
            self
        }
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.language = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for language: {}", e)
                });
            self
        }
        pub fn learning_outcome<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLearningOutcomeType>>,
            T::Error: std::fmt::Display,
        {
            self.learning_outcome = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for learning_outcome: {}", e
                    )
                });
            self
        }
        pub fn learning_outcome_summary<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::NoteType>>,
            T::Error: std::fmt::Display,
        {
            self.learning_outcome_summary = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for learning_outcome_summary: {}",
                        e
                    )
                });
            self
        }
        pub fn learning_setting<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ConceptType>>,
            T::Error: std::fmt::Display,
        {
            self.learning_setting = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for learning_setting: {}", e
                    )
                });
            self
        }
        pub fn maximum_duration<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DurationType>>,
            T::Error: std::fmt::Display,
        {
            self.maximum_duration = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for maximum_duration: {}", e
                    )
                });
            self
        }
        pub fn mode<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mode: {}", e));
            self
        }
        pub fn nqf_level<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.nqf_level = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for nqf_level: {}", e)
                });
            self
        }
        pub fn proven_by<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Box<Option<super::ManyLearningAssessmentSpecificationType>>,
            >,
            T::Error: std::fmt::Display,
        {
            self.proven_by = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for proven_by: {}", e)
                });
            self
        }
        pub fn qualification_code<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.qualification_code = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for qualification_code: {}", e
                    )
                });
            self
        }
        pub fn specialisation_of<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyQualificationType>>,
            T::Error: std::fmt::Display,
        {
            self.specialisation_of = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for specialisation_of: {}", e
                    )
                });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringType>>,
            T::Error: std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status: {}", e)
                });
            self
        }
        pub fn supplementary_document<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyWebResourceType>>,
            T::Error: std::fmt::Display,
        {
            self.supplementary_document = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for supplementary_document: {}",
                        e
                    )
                });
            self
        }
        pub fn target_group<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.target_group = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for target_group: {}", e)
                });
            self
        }
        pub fn thematic_area<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<ObjectOrVector<ConceptType>>>,
            T::Error: std::fmt::Display,
        {
            self.thematic_area = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for thematic_area: {}", e)
                });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ManyLangStringType>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for title: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn volume_of_learning<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DurationType>>,
            T::Error: std::fmt::Display,
        {
            self.volume_of_learning = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for volume_of_learning: {}", e
                    )
                });
            self
        }
    }
    impl std::convert::TryFrom<QualificationType> for super::QualificationType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: QualificationType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                accreditation: value.accreditation?,
                additional_note: value.additional_note?,
                alt_label: value.alt_label?,
                awarding_opportunity: value.awarding_opportunity?,
                category: value.category?,
                credit_point: value.credit_point?,
                date_modified: value.date_modified?,
                dc_type: value.dc_type?,
                description: value.description?,
                education_level: value.education_level?,
                education_subject: value.education_subject?,
                entitles_to: value.entitles_to?,
                entry_requirement: value.entry_requirement?,
                eqf_level: value.eqf_level?,
                generalisation_of: value.generalisation_of?,
                has_part: value.has_part?,
                homepage: value.homepage?,
                id: value.id?,
                identifier: value.identifier?,
                influenced_by: value.influenced_by?,
                is_part_of: value.is_part_of?,
                is_partial_qualification: value.is_partial_qualification?,
                language: value.language?,
                learning_outcome: value.learning_outcome?,
                learning_outcome_summary: value.learning_outcome_summary?,
                learning_setting: value.learning_setting?,
                maximum_duration: value.maximum_duration?,
                mode: value.mode?,
                nqf_level: value.nqf_level?,
                proven_by: value.proven_by?,
                qualification_code: value.qualification_code?,
                specialisation_of: value.specialisation_of?,
                status: value.status?,
                supplementary_document: value.supplementary_document?,
                target_group: value.target_group?,
                thematic_area: value.thematic_area?,
                title: value.title?,
                type_: value.type_?,
                volume_of_learning: value.volume_of_learning?,
            })
        }
    }
    impl From<super::QualificationType> for QualificationType {
        fn from(value: super::QualificationType) -> Self {
            Self {
                accreditation: Ok(value.accreditation),
                additional_note: Ok(value.additional_note),
                alt_label: Ok(value.alt_label),
                awarding_opportunity: Ok(value.awarding_opportunity),
                category: Ok(value.category),
                credit_point: Ok(value.credit_point),
                date_modified: Ok(value.date_modified),
                dc_type: Ok(value.dc_type),
                description: Ok(value.description),
                education_level: Ok(value.education_level),
                education_subject: Ok(value.education_subject),
                entitles_to: Ok(value.entitles_to),
                entry_requirement: Ok(value.entry_requirement),
                eqf_level: Ok(value.eqf_level),
                generalisation_of: Ok(value.generalisation_of),
                has_part: Ok(value.has_part),
                homepage: Ok(value.homepage),
                id: Ok(value.id),
                identifier: Ok(value.identifier),
                influenced_by: Ok(value.influenced_by),
                is_part_of: Ok(value.is_part_of),
                is_partial_qualification: Ok(value.is_partial_qualification),
                language: Ok(value.language),
                learning_outcome: Ok(value.learning_outcome),
                learning_outcome_summary: Ok(value.learning_outcome_summary),
                learning_setting: Ok(value.learning_setting),
                maximum_duration: Ok(value.maximum_duration),
                mode: Ok(value.mode),
                nqf_level: Ok(value.nqf_level),
                proven_by: Ok(value.proven_by),
                qualification_code: Ok(value.qualification_code),
                specialisation_of: Ok(value.specialisation_of),
                status: Ok(value.status),
                supplementary_document: Ok(value.supplementary_document),
                target_group: Ok(value.target_group),
                thematic_area: Ok(value.thematic_area),
                title: Ok(value.title),
                type_: Ok(value.type_),
                volume_of_learning: Ok(value.volume_of_learning),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResultCategoryType {
        count: Result<super::PositiveIntegerType, String>,
        id: Result<Option<super::GenericIdType>, String>,
        label: Result<super::StringType, String>,
        maximum_score: Result<Option<super::StringType>, String>,
        minimum_score: Result<Option<super::StringType>, String>,
        score: Result<Option<super::StringType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for ResultCategoryType {
        fn default() -> Self {
            Self {
                count: Err("no value supplied for count".to_string()),
                id: Ok(Default::default()),
                label: Err("no value supplied for label".to_string()),
                maximum_score: Ok(Default::default()),
                minimum_score: Ok(Default::default()),
                score: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl ResultCategoryType {
        pub fn count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::PositiveIntegerType>,
            T::Error: std::fmt::Display,
        {
            self.count = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for count: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::StringType>,
            T::Error: std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for label: {}", e)
                });
            self
        }
        pub fn maximum_score<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringType>>,
            T::Error: std::fmt::Display,
        {
            self.maximum_score = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for maximum_score: {}", e)
                });
            self
        }
        pub fn minimum_score<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringType>>,
            T::Error: std::fmt::Display,
        {
            self.minimum_score = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for minimum_score: {}", e)
                });
            self
        }
        pub fn score<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringType>>,
            T::Error: std::fmt::Display,
        {
            self.score = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for score: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<ResultCategoryType> for super::ResultCategoryType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResultCategoryType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                count: value.count?,
                id: value.id?,
                label: value.label?,
                maximum_score: value.maximum_score?,
                minimum_score: value.minimum_score?,
                score: value.score?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::ResultCategoryType> for ResultCategoryType {
        fn from(value: super::ResultCategoryType) -> Self {
            Self {
                count: Ok(value.count),
                id: Ok(value.id),
                label: Ok(value.label),
                maximum_score: Ok(value.maximum_score),
                minimum_score: Ok(value.minimum_score),
                score: Ok(value.score),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResultDistributionType {
        description: Result<Option<super::ManyLangStringType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        result_category: Result<Option<super::ManyResultCategoryType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for ResultDistributionType {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                id: Ok(Default::default()),
                result_category: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl ResultDistributionType {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn result_category<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyResultCategoryType>>,
            T::Error: std::fmt::Display,
        {
            self.result_category = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for result_category: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<ResultDistributionType>
    for super::ResultDistributionType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResultDistributionType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                id: value.id?,
                result_category: value.result_category?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::ResultDistributionType> for ResultDistributionType {
        fn from(value: super::ResultDistributionType) -> Self {
            Self {
                description: Ok(value.description),
                id: Ok(value.id),
                result_category: Ok(value.result_category),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ShaclValidator2017Type {
        id: Result<Option<super::GenericIdType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for ShaclValidator2017Type {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl ShaclValidator2017Type {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<ShaclValidator2017Type>
    for super::ShaclValidator2017Type {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ShaclValidator2017Type,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::ShaclValidator2017Type> for ShaclValidator2017Type {
        fn from(value: super::ShaclValidator2017Type) -> Self {
            Self {
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ShortenedGradingType {
        id: Result<Option<super::GenericIdType>, String>,
        percentage_equal: Result<super::IntegerType, String>,
        percentage_higher: Result<super::IntegerType, String>,
        percentage_lower: Result<super::IntegerType, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for ShortenedGradingType {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                percentage_equal: Err(
                    "no value supplied for percentage_equal".to_string(),
                ),
                percentage_higher: Err(
                    "no value supplied for percentage_higher".to_string(),
                ),
                percentage_lower: Err(
                    "no value supplied for percentage_lower".to_string(),
                ),
                type_: Ok(Default::default()),
            }
        }
    }
    impl ShortenedGradingType {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn percentage_equal<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IntegerType>,
            T::Error: std::fmt::Display,
        {
            self.percentage_equal = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for percentage_equal: {}", e
                    )
                });
            self
        }
        pub fn percentage_higher<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IntegerType>,
            T::Error: std::fmt::Display,
        {
            self.percentage_higher = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for percentage_higher: {}", e
                    )
                });
            self
        }
        pub fn percentage_lower<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IntegerType>,
            T::Error: std::fmt::Display,
        {
            self.percentage_lower = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for percentage_lower: {}", e
                    )
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<ShortenedGradingType> for super::ShortenedGradingType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ShortenedGradingType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                percentage_equal: value.percentage_equal?,
                percentage_higher: value.percentage_higher?,
                percentage_lower: value.percentage_lower?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::ShortenedGradingType> for ShortenedGradingType {
        fn from(value: super::ShortenedGradingType) -> Self {
            Self {
                id: Ok(value.id),
                percentage_equal: Ok(value.percentage_equal),
                percentage_higher: Ok(value.percentage_higher),
                percentage_lower: Ok(value.percentage_lower),
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
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<TermsOfUse> for super::TermsOfUse {
        type Error = super::error::ConversionError;
        fn try_from(value: TermsOfUse) -> Result<Self, super::error::ConversionError> {
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
    #[derive(Clone, Debug)]
    pub struct TermsOfUseType {
        id: Result<Option<super::GenericIdType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for TermsOfUseType {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl TermsOfUseType {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<TermsOfUseType> for super::TermsOfUseType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TermsOfUseType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::TermsOfUseType> for TermsOfUseType {
        fn from(value: super::TermsOfUseType) -> Self {
            Self {
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VerificationCheckType {
        dc_type: Result<super::ConceptType, String>,
        description: Result<Option<super::ManyLangStringType>, String>,
        elm_subject: Result<Option<super::EuropeanDigitalCredentialType>, String>,
        id: Result<Option<super::GenericIdType>, String>,
        subject: Result<serde_json::Value, String>,
        type_: Result<Option<serde_json::Value>, String>,
        verification_status: Result<super::ConceptType, String>,
    }
    impl Default for VerificationCheckType {
        fn default() -> Self {
            Self {
                dc_type: Err("no value supplied for dc_type".to_string()),
                description: Ok(Default::default()),
                elm_subject: Ok(Default::default()),
                id: Ok(Default::default()),
                subject: Err("no value supplied for subject".to_string()),
                type_: Ok(Default::default()),
                verification_status: Err(
                    "no value supplied for verification_status".to_string(),
                ),
            }
        }
    }
    impl VerificationCheckType {
        pub fn dc_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ConceptType>,
            T::Error: std::fmt::Display,
        {
            self.dc_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dc_type: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn elm_subject<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::EuropeanDigitalCredentialType>>,
            T::Error: std::fmt::Display,
        {
            self.elm_subject = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for elm_subject: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn subject<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self.subject = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subject: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn verification_status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ConceptType>,
            T::Error: std::fmt::Display,
        {
            self.verification_status = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for verification_status: {}", e
                    )
                });
            self
        }
    }
    impl std::convert::TryFrom<VerificationCheckType> for super::VerificationCheckType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VerificationCheckType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                dc_type: value.dc_type?,
                description: value.description?,
                elm_subject: value.elm_subject?,
                id: value.id?,
                subject: value.subject?,
                type_: value.type_?,
                verification_status: value.verification_status?,
            })
        }
    }
    impl From<super::VerificationCheckType> for VerificationCheckType {
        fn from(value: super::VerificationCheckType) -> Self {
            Self {
                dc_type: Ok(value.dc_type),
                description: Ok(value.description),
                elm_subject: Ok(value.elm_subject),
                id: Ok(value.id),
                subject: Ok(value.subject),
                type_: Ok(value.type_),
                verification_status: Ok(value.verification_status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WebResourceType {
        content_url: Result<super::UriType, String>,
        id: Result<Option<super::GenericIdType>, String>,
        language: Result<Option<super::ConceptType>, String>,
        title: Result<Option<super::ManyLangStringType>, String>,
        type_: Result<Option<serde_json::Value>, String>,
    }
    impl Default for WebResourceType {
        fn default() -> Self {
            Self {
                content_url: Err("no value supplied for content_url".to_string()),
                id: Ok(Default::default()),
                language: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl WebResourceType {
        pub fn content_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::UriType>,
            T::Error: std::fmt::Display,
        {
            self.content_url = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for content_url: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GenericIdType>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ConceptType>>,
            T::Error: std::fmt::Display,
        {
            self.language = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for language: {}", e)
                });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ManyLangStringType>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for title: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<WebResourceType> for super::WebResourceType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WebResourceType,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                content_url: value.content_url?,
                id: value.id?,
                language: value.language?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::WebResourceType> for WebResourceType {
        fn from(value: super::WebResourceType) -> Self {
            Self {
                content_url: Ok(value.content_url),
                id: Ok(value.id),
                language: Ok(value.language),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
}
