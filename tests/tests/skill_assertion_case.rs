use crate::tests::assert_eq_json_value;
use openbadges::json_schema::{
    achievement::{Achievement, AchievementBuilder, Criteria, CriteriaBuilder, AchievementType},
    achievement_credential::{AchievementCredential, AchievementCredentialBuilder, AchievementCredentialType, CredentialSchema, CredentialSchemaBuilder, AchievementCredentialSchema},
    achievement_subject::{AchievementSubject, AchievementSubjectBuilder},
    profile::{Profile, ProfileBuilder}, alignment::{Alignment, AlignmentBuilder, AlignmentTargetType}, general::{Image, ImageBuilder}, proof_evidence::{Evidence, EvidenceBuilder},
};
use std::fs::File;

#[test]
fn skill_assertion_case() {

    // Testing if serialization and deserialization between the OBv3 examples and our rust code works as needed.

    assert_eq_json_value::<AchievementCredential>("tests/obv3_json_examples/skill_assertion_case.json");

    // Next, the builders are tested against the OBv3 examples 

    let skill_assertion_case_builder: AchievementCredential = AchievementCredentialBuilder::default()
    .context(vec![
        "https://www.w3.org/2018/credentials/v1".into(),
		"https://purl.imsglobal.org/spec/ob/v3p0/context-3.0.2.json".into(),
		"https://purl.imsglobal.org/spec/ob/v3p0/extensions.json".into()
    ])
    .id("http://1edtech.edu/credentials/3732")
    .type_(AchievementCredentialType::from(vec!["VerifiableCredential", "OpenBadgeCredential"]))
    .name("Robot Programming Skill Credential")
    .description("A badge recognizing the development of skills in robot implementation, specifically the software".to_string())
    .credential_subject({
        let credential_subject: AchievementSubject = AchievementSubjectBuilder::default()
        .id("did:example:ebfeb1f712ebc6f1c276e12ec21".to_string())
        .type_("AchievementSubject",)
        .achievement({
            let achievement: Achievement = AchievementBuilder::default()
            .id("https://example.com/achievements/robotics/robot-programming")
            .type_("Achievement")
            .criteria({
                let criteria: Criteria = CriteriaBuilder::default()
                .narrative("Cite strong and thorough textual evidence to support analysis of what the text says explicitly as well as inferences drawn from the text, including determining where the text leaves matters uncertain".to_string())
                .try_into()
                .unwrap();
                
                criteria
            })
            .description("Analyze a sample text")
            .name("Text analysis")
            .alignment({
                let alignment: Alignment = AlignmentBuilder::default()
                .type_("Alignment")
                .target_description("Robot software is a set of commands and procedures robots use to respond to input and perform autonomous tasks.".to_string())
                .target_name("Robot Programming")
                .target_framework("Example Robotics Framework".to_string())
                .target_type(AlignmentTargetType::from("CFItem"))
                .target_url("https://robotics-competencies.example.com/competencies/robot-programming")
                .try_into()
                .unwrap();
    
                vec![alignment]
            })
            .achievement_type(AchievementType::from("Competency"))
            .creator({
                let creator: Profile = ProfileBuilder::default()
                .id("https://example.com/issuers/123767")
                .type_("Profile")
                .name("Example Industry Group".to_string())
                .url("https://example.com".to_string())
                .description("Example Industry Group is a consortium of luminaries who publish skills data for common usage.".to_string())
                .email("info@exammple.com".to_string())
                .try_into()
                .unwrap();

                creator
            })
            .criteria({
                let criteria: Criteria = CriteriaBuilder::default()
                .narrative("Learners must present source code showing the ability for a robot to accept manual or sensor input and perform conditional actions in response.".to_string())
                .try_into()
                .unwrap();
                
                criteria
            })
            .description("This achievement represents developing capability to develop software for robotic applications.".to_string())
            .image({
                let image: Image = ImageBuilder::default()
                .id("https://example.com/achievements/robotics/robot-programming/image")
                .type_("Image")
                .caption("A robot filled with ones and zeroes representing its programming".to_string())
                .try_into()
                .unwrap();

                image
            })
            .name("Robot Programming")
            .try_into()
            .unwrap();
    
            achievement
        })
        .try_into()
        .unwrap();
    
        credential_subject
    })
    .evidence({
        let evidence: Evidence = EvidenceBuilder::default()
        .id("https://github.com/somebody/project".to_string())
        .type_("Evidence")
        .name("Final Project Code".to_string())
        .description("The source code for the 'Beeper 1.0' robot project. It responds by saying 'beep' when the 'beep' button is pressed.".to_string())
        .try_into()
        .unwrap();

        vec![evidence]
    })
    .issuer({
        let issuer: Profile = ProfileBuilder::default()
        .id("https://1edtech.edu/issuers/565049")
        .type_("Profile")
        .name("1EdTech University".to_string())
        .url("https://1edtech.edu".to_string())
        .phone("1-222-333-4444".to_string())
        .description("1EdTech University provides online degree programs.".to_string())
        .image({
            let image: Image = ImageBuilder::default()
            .id("https://1edtech.edu/logo.png")
            .type_("Image")
            .caption("1EdTech University logo".to_string())
            .try_into()
            .unwrap();

            image
        })
        .email("registrar@1edtech.edu".to_string())
        .try_into()
        .unwrap();

        issuer
    })
    .issuance_date("2022-07-01T00:00:00Z".parse::<chrono::DateTime<chrono::offset::Utc>>().unwrap())
    .credential_schema({
        let schema: CredentialSchema = CredentialSchemaBuilder::default()
        .id("https://purl.imsglobal.org/spec/ob/v3p0/schema/json/ob_v3p0_achievementcredential_schema.json")
        .type_("1EdTechJsonSchemaValidator2019")
        .try_into()
        .unwrap();

        AchievementCredentialSchema::from(vec![schema])
    })
    .try_into()
    .unwrap();

    let file = File::open("tests/obv3_json_examples/skill_assertion_case.json").expect("Failed to open file");
    let json_value_from_file: serde_json::Value = serde_json::from_reader(file).expect("Couldn't read from file");

    assert_eq!(serde_json::to_value(skill_assertion_case_builder).unwrap(), json_value_from_file);

}
