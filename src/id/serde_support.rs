use std::borrow::Cow;

impl serde::Serialize for super::SpiffeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for super::SpiffeId {
    fn deserialize<D>(deserializer: D) -> Result<super::SpiffeId, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let maybe_id: Cow<'_, str> = serde::Deserialize::deserialize(deserializer)?;

        Self::parse(&maybe_id).map_err(serde::de::Error::custom)
    }
}
