use std::fmt::{Formatter, Result as FmtResult};

use base64ct::{Base64UrlUnpadded, Encoding};
use serde_core::{
    Deserialize, Deserializer,
    de::{Error, IgnoredAny, MapAccess, Visitor},
};
use serde_json::Deserializer as JsonDeserializer;
use spiffe_id::SpiffeId;

use crate::SpiffeError;

struct SubIdentifierVisitor;

impl Visitor<'_> for SubIdentifierVisitor {
    type Value = IsSub;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("field identifier")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(IsSub(v == "sub"))
    }

    fn visit_bytes<E: Error>(self, v: &[u8]) -> Result<Self::Value, E> {
        Ok(IsSub(v == b"sub"))
    }
}

struct IsSub(bool);

impl<'de> Deserialize<'de> for IsSub {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_identifier(SubIdentifierVisitor)
    }
}

struct SubClaimVisitor;

impl<'de> Visitor<'de> for SubClaimVisitor {
    type Value = Box<str>;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("claim with field `sub`")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut sub = None;

        while let Some(is_sub) = map.next_key()? {
            match is_sub {
                IsSub(true) => {
                    if sub.is_some() {
                        return Err(Error::duplicate_field("sub"));
                    }
                    sub = Some(map.next_value()?);
                }
                IsSub(false) => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }

        match sub {
            Some(sub) => Ok(sub),
            None => Err(Error::missing_field("sub")),
        }
    }
}

/// Extracts SPIFFE ID from a trusted JWT-SVID
///
/// It is assumed that the JWT-SVID is a valid JWT token with a `sub` claim containing the SPIFFE ID.
///
/// Usually, this function is used to extract SPIFFE ID from a JWT-SVID that is already verified.
pub fn spiffe_id_from_jwt_svid_unchecked(svid: &str) -> Result<SpiffeId, SpiffeError> {
    const INVALID_JWT_ERR: SpiffeError = SpiffeError::InvalidJwtSvid;

    let (prefix, _signature) = svid.rsplit_once('.').ok_or(INVALID_JWT_ERR)?;
    let (_header, body) = prefix.rsplit_once('.').ok_or(INVALID_JWT_ERR)?;

    let body_json = Base64UrlUnpadded::decode_vec(body).map_err(|_| INVALID_JWT_ERR)?;

    let sub: Box<str> = JsonDeserializer::from_slice(&body_json)
        .deserialize_struct("Claims", &["sub"], SubClaimVisitor)
        .map_err(|_| INVALID_JWT_ERR)?;

    SpiffeId::new(sub).map_err(SpiffeError::SpiffeId)
}
