use crate::tests::assert_eq_json_value;
use openbadges::json_schema::{
    achievement::{Achievement, AchievementBuilder, Criteria, CriteriaBuilder},
    achievement_credential::{AchievementCredential, AchievementCredentialBuilder, AchievementCredentialType},
    achievement_subject::{AchievementSubject, AchievementSubjectBuilder},
    profile::{Profile, ProfileBuilder},
};

// The following tests all examples of the OBv3 website: https://www.imsglobal.org/spec/ob/v3p0#examples-0

#[test]
fn test_obv3_examples() {

    assert_eq_json_value::<AchievementCredential>("tests/json_examples/skill_assertion2.json");

}


