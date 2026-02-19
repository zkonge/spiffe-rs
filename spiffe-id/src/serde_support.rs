use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    string::String,
    vec::Vec,
};
use core::fmt::{Formatter, Result as FmtResult};

use serde_core::{
    Deserialize, Deserializer, Serialize, Serializer, de,
    de::{Error, Unexpected, Visitor},
};

use crate::{SpiffeId, TrustDomain, tri};

struct CowStrVisitor;

impl<'a> Visitor<'a> for CowStrVisitor {
    type Value = Cow<'a, str>;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("a string")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(Cow::Owned(v.to_owned()))
    }

    fn visit_borrowed_str<E: Error>(self, v: &'a str) -> Result<Self::Value, E> {
        Ok(Cow::Borrowed(v))
    }

    fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
        Ok(Cow::Owned(v))
    }

    fn visit_bytes<E: Error>(self, v: &[u8]) -> Result<Self::Value, E> {
        match str::from_utf8(v) {
            Ok(s) => Ok(Cow::Owned(s.to_owned())),
            Err(_) => Err(Error::invalid_value(Unexpected::Bytes(v), &self)),
        }
    }

    fn visit_borrowed_bytes<E: Error>(self, v: &'a [u8]) -> Result<Self::Value, E> {
        match str::from_utf8(v) {
            Ok(s) => Ok(Cow::Borrowed(s)),
            Err(_) => Err(Error::invalid_value(Unexpected::Bytes(v), &self)),
        }
    }

    fn visit_byte_buf<E: Error>(self, v: Vec<u8>) -> Result<Self::Value, E> {
        match String::from_utf8(v) {
            Ok(s) => Ok(Cow::Owned(s)),
            Err(e) => Err(Error::invalid_value(
                Unexpected::Bytes(&e.into_bytes()),
                &self,
            )),
        }
    }
}

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
        let id: Box<str> = tri!(Deserialize::deserialize(deserializer));

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
        let id: Cow<'a, str> = tri!(deserializer.deserialize_str(CowStrVisitor));

        id.try_into().map_err(de::Error::custom)
    }
}
