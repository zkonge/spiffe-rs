use alloc::borrow::Cow;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

use crate::{SpiffeId, TrustDomain};

impl Serialize for SpiffeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for SpiffeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let maybe_id: Cow<'_, str> = Deserialize::deserialize(deserializer)?;

        Self::new(maybe_id.into_owned()).map_err(de::Error::custom)
    }
}

impl Serialize for TrustDomain<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'a, 'de: 'a> Deserialize<'de> for TrustDomain<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let maybe_id: Cow<'_, str> = Deserialize::deserialize(deserializer)?;

        maybe_id.try_into().map_err(de::Error::custom)
    }
}
