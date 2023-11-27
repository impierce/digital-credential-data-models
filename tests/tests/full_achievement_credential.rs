use crate::tests::assert_eq_json_value;
use openbadges::json_schema::{
    achievement::{Achievement, AchievementBuilder, Criteria, CriteriaBuilder},
    achievement_credential::{AchievementCredential, AchievementCredentialBuilder, AchievementCredentialType},
    achievement_subject::{AchievementSubject, AchievementSubjectBuilder},
    profile::{Profile, ProfileBuilder},
};
use std::fs::File;

#[test]
fn full_achievement_credential() {

    // Testing if serialization and deserialization between the OBv3 examples and our rust code works as needed.

    assert_eq_json_value::<AchievementCredential>("tests/obv3_json_examples/full_achievement_credential.json");
    
    // Next, the builders are tested against the OBv3 examples 

    
}
