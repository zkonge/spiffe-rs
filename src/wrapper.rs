use std::fmt::{Display, Formatter, Result as FmtResult};

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
    InvalidDerData(#[from] InvalidDerDataError),
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
            svid: split_certificates(&value.x509_svid)
                .map(|x| x.map(CertificateDer::into_owned))
                .collect::<Result<Vec<_>, _>>()?,
            key: value.x509_svid_key.into(),
            bundle: split_certificates(&value.bundle)
                .map(|x| x.map(CertificateDer::into_owned))
                .collect::<Result<Vec<_>, _>>()?,
            hint: if value.hint.is_empty() {
                None
            } else {
                Some(value.hint)
            },
        })
    }
}

#[derive(Error, Debug)]
pub struct InvalidDerDataError;

impl Display for InvalidDerDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "bad DER data")
    }
}

type Tlv<'a> = (u8, usize, &'a [u8]);

fn read_der_tlv(der: &[u8]) -> Option<(&[u8], Tlv)> {
    let [tag, first_len_byte, rem @ ..] = der else {
        return None;
    };

    if *first_len_byte & 0x80 == 0 {
        let (value, rem) = rem.split_at_checked(*first_len_byte as usize)?;
        return Some((rem, (*tag, *first_len_byte as usize, value)));
    }

    let len_len = *first_len_byte & 0x7f;
    let (len_bytes, rem) = rem.split_at_checked(len_len as usize)?;

    let len: usize = match len_bytes {
        [a] => u32::from_le_bytes([*a, 0, 0, 0]) as _,
        [a, b] => u32::from_le_bytes([*b, *a, 0, 0]) as _,
        [a, b, c] => u32::from_le_bytes([*c, *b, *a, 0]) as _,
        // Is it possible to have a 16 MiB+ X.509 certificate in real world?
        _ => return None,
    };

    let (value, rem) = rem.split_at_checked(len)?;

    Some((rem, (*tag, len, value)))
}

fn split_cert(raw: &[u8]) -> Option<(&[u8], CertificateDer<'_>)> {
    let (rem, (0x30, _, _)) = read_der_tlv(raw)? else {
        return None;
    };

    let cert = raw.get(..raw.len() - rem.len())?;

    Some((rem, CertificateDer::from_slice(cert)))
}

pub struct CertificateIter<'a> {
    der: &'a [u8],
}

impl<'a> Iterator for CertificateIter<'a> {
    type Item = Result<CertificateDer<'a>, InvalidDerDataError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.der.is_empty() {
            return None;
        }

        let (rem, cert) = match split_cert(self.der) {
            Some((r, c)) => (r, Ok(c)),
            None => ([].as_slice(), Err(InvalidDerDataError)),
        };
        self.der = rem;
        Some(cert)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (if self.der.is_empty() { 0 } else { 1 }, None)
    }
}

pub fn split_certificates(der: &[u8]) -> CertificateIter<'_> {
    CertificateIter { der }
}
