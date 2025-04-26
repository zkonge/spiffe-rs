use alloc::{borrow::Cow, boxed::Box};

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
        let id: Box<str> = Deserialize::deserialize(deserializer)?;

        Self::new(id).map_err(de::Error::custom)
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
        // https://github.com/serde-rs/serde/issues/1852
        // waiting for Rust specialization so it could be zero-copy
        let id: Cow<str> = Deserialize::deserialize(deserializer)?;

        id.try_into().map_err(de::Error::custom)
    }
}
