use crate::tests::assert_eq_json_value;
use openbadges::{
    achievement_credential::{CredentialSchema, CredentialSchemaBuilder, CredentialStatus, CredentialStatusBuilder},
    endorsement::{EndorsementCredential, EndorsementCredentialBuilder, EndorsementSubject, EndorsementSubjectBuilder, EndorsementCredentialSchema},
    profile::{Profile, ProfileBuilder}, general::{RefreshService, RefreshServiceBuilder},
};
use std::fs::File;

#[test]
fn endorsement_credential() {

    // Testing if serialization and deserialization between the OBv3 examples and our rust code works as needed.

    assert_eq_json_value::<EndorsementCredential>("tests/obv3_json_examples/endorsement_credential.json");
    
    // Next, the builders are tested against the OBv3 examples

    let endorsement_credential: EndorsementCredential = EndorsementCredentialBuilder::default()
    .context(vec![
        "https://www.w3.org/2018/credentials/v1".into(),
        "https://purl.imsglobal.org/spec/ob/v3p0/context-3.0.2.json".into(),
        "https://purl.imsglobal.org/spec/ob/v3p0/extensions.json".into()
    ])
    .id("http://1edtech.edu/endorsementcredential/3732")
    .type_(vec![
        "VerifiableCredential",
        "EndorsementCredential"
    ])
    .name("SDE endorsement")
    .issuer({
        let issuer: Profile = ProfileBuilder::default()
        .id("https://state.gov/issuers/565049")
        .type_("Profile")
        .name("State Department of Education".to_string())
        .try_into()
        .unwrap();

        issuer
    })
    .issuance_date("2010-01-01T00:00:00Z".parse::<chrono::DateTime<chrono::offset::Utc>>().unwrap())
    .expiration_date("2030-01-01T00:00:00Z".parse::<chrono::DateTime<chrono::offset::Utc>>().unwrap())
    .credential_subject({
        let credential_subject: EndorsementSubject = EndorsementSubjectBuilder::default()
        .id("https://1edtech.edu/issuers/565049")
        .type_("EndorsementSubject")
        .endorsement_comment("1EdTech University is in good standing".to_string())
        .try_into()
        .unwrap();

        credential_subject
    })
    .credential_schema({
        let schema1: CredentialSchema = CredentialSchemaBuilder::default()
        .id("https://purl.imsglobal.org/spec/ob/v3p0/schema/json/ob_v3p0_endorsementcredential_schema.json")
        .type_("1EdTechJsonSchemaValidator2019")
        .try_into()
        .unwrap();

        let schema2: CredentialSchema = CredentialSchemaBuilder::default()
        .id("https://state.gov/schema/endorsementcredential.json")
        .type_("1EdTechJsonSchemaValidator2019")
        .try_into()
        .unwrap();

        EndorsementCredentialSchema::from(vec![schema1, schema2])
    })
    .credential_status({
        let credential_status: CredentialStatus = CredentialStatusBuilder::default()
        .id("https://state.gov/credentials/3732/revocations")
        .type_("1EdTechRevocationList")
        .try_into()
        .unwrap();

        credential_status
    })
    .refresh_service({
        let refresh_service: RefreshService = RefreshServiceBuilder::default()
        .id("http://state.gov/credentials/3732")
        .type_("1EdTechCredentialRefresh")
        .try_into()
        .unwrap();

        refresh_service
    })
    .try_into()
    .unwrap();

    // Here we test the built struct against the struct deserialized from the example .json file.
    
    let file = File::open("tests/obv3_json_examples/endorsement_credential.json").expect("Failed to open file");
    let endorsement_cred_from_file: EndorsementCredential = serde_json::from_reader(&file).expect("Couldn't read from file");
    
    assert_eq!(endorsement_credential, endorsement_cred_from_file);
    
    // Here we test the built struct converted to a json_value against the json_value deserialized from the example .json file

    let file = File::open("tests/obv3_json_examples/endorsement_credential.json").expect("Failed to open file");
    let json_value_from_file: serde_json::Value = serde_json::from_reader(file).expect("Couldn't read from file");

    assert_eq!(serde_json::to_value(endorsement_credential).unwrap(), json_value_from_file);

}
