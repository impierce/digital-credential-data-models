use serde::Deserialize;

impl<'de, T> Deserialize<'de> for T where T: EnumDeserialize  {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let serde_value = serde_json::Value::deserialize(deserializer)?;

        let obj_type = serde_value
            .get("type")
            .map(|t| t.as_str().map(|s| s.to_string()))
            .ok_or(de::Error::missing_field("type"))?;

        let variants = Self::variants();

        if let Some(obj_type) = obj_type {
            if variants.iter().any(|var_type| *var_type == obj_type) {
                return Self::into_enum(&obj_type, serde_value).map_err(de::Error::custom);
            } else {
                return Err(de::Error::unknown_variant(&obj_type, variants));
            }
        }

        Err(de::Error::unknown_variant("", variants))
    }
}
