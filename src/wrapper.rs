use rustls_pki_types::{CertificateDer, PrivatePkcs8KeyDer};
use spiffe_id::{SpiffeId, SpiffeIdError};
use thiserror::Error;

use crate::{Jwtsvid, X509svid};

#[derive(Error, Debug)]
pub enum InvalidSvidError {
    #[error("invalid SPIFFE ID: {0}")]
    InvalidSpiffeId(#[from] SpiffeIdError),

    #[error("major part of SVID is empty")]
    EmptySvid,

    #[error("invalid DER data")]
    InvalidDerData,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct JwtSvid {
    spiffe_id: SpiffeId,
    svid: String,
    hint: Option<String>,
}

impl JwtSvid {
    pub fn spiffe_id(&self) -> &SpiffeId {
        &self.spiffe_id
    }

    pub fn svid(&self) -> &str {
        &self.svid
    }

    pub fn hint(&self) -> Option<&str> {
        self.hint.as_deref()
    }
}

impl TryFrom<Jwtsvid> for JwtSvid {
    type Error = InvalidSvidError;

    fn try_from(value: Jwtsvid) -> Result<Self, Self::Error> {
        Ok(Self {
            spiffe_id: SpiffeId::parse(value.spiffe_id)?,
            svid: value.svid,
            hint: if value.hint.is_empty() {
                None
            } else {
                Some(value.hint)
            },
        })
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct X509Svid {
    spiffe_id: SpiffeId,
    svid: Vec<CertificateDer<'static>>,
    key: PrivatePkcs8KeyDer<'static>,
    bundle: Vec<CertificateDer<'static>>,
    hint: Option<String>,
}

impl X509Svid {
    pub fn clone_with_key(&self) -> Self {
        Self {
            spiffe_id: self.spiffe_id.clone(),
            svid: self.svid.clone(),
            key: self.key.clone_key(),
            bundle: self.bundle.clone(),
            hint: self.hint.clone(),
        }
    }

    pub fn spiffe_id(&self) -> &SpiffeId {
        &self.spiffe_id
    }

    pub fn svid(&self) -> &[CertificateDer<'static>] {
        &self.svid
    }

    pub fn key(&self) -> &PrivatePkcs8KeyDer<'static> {
        &self.key
    }

    pub fn bundle(&self) -> &[CertificateDer<'static>] {
        &self.bundle
    }

    pub fn hint(&self) -> Option<&str> {
        self.hint.as_deref()
    }
}

impl TryFrom<X509svid> for X509Svid {
    type Error = InvalidSvidError;

    fn try_from(value: X509svid) -> Result<Self, Self::Error> {
        if value.x509_svid.is_empty() || value.x509_svid_key.is_empty() || value.bundle.is_empty() {
            return Err(InvalidSvidError::EmptySvid);
        }

        Ok(Self {
            spiffe_id: SpiffeId::parse(value.spiffe_id)?,
            svid: split_certs(&value.x509_svid)?,
            key: value.x509_svid_key.into(),
            bundle: split_certs(&value.bundle)?,
            hint: if value.hint.is_empty() {
                None
            } else {
                Some(value.hint)
            },
        })
    }
}

fn split_cert<'a>(raw: &'a [u8]) -> Result<(&'a [u8], CertificateDer<'a>), InvalidSvidError> {
    const SHORT_FORM_LEN_MAX: u8 = 127;
    const TAG_SEQUENCE: u8 = 0x30;

    // We expect all key formats to begin with a SEQUENCE, which requires at least 2 bytes
    // in the short length encoding.
    if raw.first() != Some(&TAG_SEQUENCE) || raw.len() < 2 {
        return Err(InvalidSvidError::InvalidDerData);
    }

    let (meta_len, value_len) = if raw[1] < SHORT_FORM_LEN_MAX {
        // short form length
        (2, raw[1] as usize)
    } else {
        // long form length
        let length_of_length_bytes = (raw[1] & 0x7F) as usize; // pick the low 7 bits

        if raw[2..].len() < length_of_length_bytes {
            return Err(InvalidSvidError::InvalidDerData);
        }

        let length_bytes = raw
            .get(2..2 + length_of_length_bytes)
            .ok_or(InvalidSvidError::InvalidDerData)?;

        let mut aligned_length_bytes = [0u8; size_of::<usize>()];
        aligned_length_bytes[size_of::<usize>() - length_of_length_bytes..]
            .copy_from_slice(length_bytes);

        let content_len = usize::from_be_bytes(aligned_length_bytes);

        (2 + length_of_length_bytes, content_len)
    };

    raw.split_at_checked(meta_len + value_len)
        .map(|(cert_bytes, rem)| (rem, CertificateDer::from_slice(cert_bytes)))
        .ok_or(InvalidSvidError::InvalidDerData)
}

pub fn split_certs(mut raw: &[u8]) -> Result<Vec<CertificateDer<'static>>, InvalidSvidError> {
    let mut certs = Vec::new();

    while !raw.is_empty() {
        let (remainder, cert) = split_cert(raw)?;
        certs.push(cert.into_owned());
        raw = remainder;
    }

    Ok(certs)
}
