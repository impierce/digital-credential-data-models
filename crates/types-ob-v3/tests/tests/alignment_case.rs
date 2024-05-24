use crate::tests::assert_eq_json_value;
use std::{fs::File, str::FromStr};
use types_ob_v3::prelude::*;

#[test]
fn alignment_case() {
    // Testing if serialization and deserialization between the OBv3 examples and our rust code works as needed.

    assert_eq_json_value::<AchievementCredential>("tests/obv3_json_examples/alignment_case.json");

    // Next, the builders are used to replicate the data from the .json file

    let alignment_case: AchievementCredential = AchievementCredentialBuilder::default()
    .context(vec![
        "https://www.w3.org/2018/credentials/v1",
        "https://purl.imsglobal.org/spec/ob/v3p0/context-3.0.2.json"
    ])
    .id("http://example.edu/credentials/3732")
    .type_(AchievementCredentialType::from(vec!["VerifiableCredential", "OpenBadgeCredential"]))
    .issuer(
        ProfileBuilder::default()
        .id("https://example.edu/issuers/565049")
        .type_("Profile")
        .name("Example University".to_string())
    )
    .issuance_date("2010-01-01T00:00:00Z")
    .name("Example University Degree")
    .credential_subject(
        AchievementSubjectBuilder::default()
        .id("did:example:ebfeb1f712ebc6f1c276e12ec21".to_string())
        .type_("AchievementSubject",)
        .achievement(
            AchievementBuilder::default()
            .id("https://1edtech.edu/achievements/1")
            .type_("Achievement")
            .criteria(
                CriteriaBuilder::default()
                .narrative("Cite strong and thorough textual evidence to support analysis of what the text says explicitly as well as inferences drawn from the text, including determining where the text leaves matters uncertain".to_string())
            )
            .description("Analyze a sample text")
            .name("Text analysis")
            .alignment(
                vec![
                AlignmentBuilder::default()
                .type_("Alignment")
                .target_code("ce-cf4dee18-7cea-443a-b920-158a0762c6bf".to_string())
                .target_framework("Edmonds College Course Catalog".to_string())
                .target_name("Requirements Analysis")
                .target_type(AlignmentTargetType::from_str("ceterms:Credential").unwrap())
                .target_url("https://credentialfinder.org/credential/20229/Requirements_Analysis")
            ])
        )
    )
    .try_into()
    .unwrap();

    // Here we test the built struct against the struct deserialized from the example .json file.

    let file = File::open("tests/obv3_json_examples/alignment_case.json").expect("Failed to open file");
    let alignment_case_from_file: AchievementCredential =
        serde_json::from_reader(&file).expect("Couldn't read from file");

    assert_eq!(alignment_case, alignment_case_from_file);

    // Here we test the built struct converted to a json_value against the json_value deserialized from the example .json file

    let file = File::open("tests/obv3_json_examples/alignment_case.json").expect("Failed to open file");
    let json_value_from_file: serde_json::Value = serde_json::from_reader(file).expect("Couldn't read from file");

    assert_eq!(serde_json::to_value(alignment_case).unwrap(), json_value_from_file);
}
