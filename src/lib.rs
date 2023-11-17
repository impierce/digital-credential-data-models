pub mod json_schema;

#[cfg(test)]
mod tests {
    use crate::json_schema::{
        Achievement, AchievementCredential, AchievementSubject, Context, Criteria,
    };

    #[test]
    fn test_achievement_credential() {
        let _achievement_credential: AchievementCredential = AchievementCredential::builder()
            .context(vec![Context::Variant1(
                "https://www.w3.org/2018/credentials/examples/v1".to_string(),
            )])
            .credential_subject(
                AchievementSubject::builder()
                    .achievement(Achievement::builder().criteria(Criteria::builder())),
            )
            .try_into()
            .unwrap();
    }
}
