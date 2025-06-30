#[cfg(feature = "unchecked-api")]
use spiffe_id::SpiffeId;

#[cfg(feature = "unchecked-api")]
use crate::SpiffeError;

/// Extracts SPIFFE ID from a trusted JWT-SVID
///
/// It is assumed that the JWT-SVID is a valid JWT token with a `sub` claim containing the SPIFFE ID.
///
/// Usually, this function is used to extract SPIFFE ID from a JWT-SVID that is already verified.
#[cfg(feature = "unchecked-api")]
pub fn spiffe_id_from_jwt_svid_unchecked(svid: &str) -> Result<SpiffeId, SpiffeError> {
    use std::borrow::Cow;

    use base64ct::{Base64UrlUnpadded, Encoding};
    use serde::Deserialize;

    const INVALID_JWT_ERR: SpiffeError = SpiffeError::InvalidJwtSvid;
    let (prefix, _signature) = svid.split_once('.').ok_or(INVALID_JWT_ERR)?;
    let (_header, body) = prefix.split_once('.').ok_or(INVALID_JWT_ERR)?;

    let body_json = Base64UrlUnpadded::decode_vec(body).map_err(|_| INVALID_JWT_ERR)?;

    #[derive(Deserialize)]
    struct SubjectExtractor<'a> {
        sub: Cow<'a, str>,
    }

    let extactor: SubjectExtractor<'_> =
        serde_json::from_slice(&body_json).map_err(|_| INVALID_JWT_ERR)?;

    SpiffeId::new(extactor.sub).map_err(SpiffeError::SpiffeId)
}
