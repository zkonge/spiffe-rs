use rustls_pki_types::{CertificateDer, PrivatePkcs8KeyDer};
use spiffe_id::SpiffeId;

use super::{SpiffeError, split_certificates};
use crate::{JwtSvid as ProtoJwtSvid, X509Svid as ProtoX509Svid};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct JwtSvid {
    spiffe_id: SpiffeId,
    svid: Box<str>,
    hint: Option<Box<str>>,
}

impl JwtSvid {
    #[inline]
    pub fn spiffe_id(&self) -> &SpiffeId {
        &self.spiffe_id
    }

    #[inline]
    pub fn svid(&self) -> &str {
        &self.svid
    }

    #[inline]
    pub fn hint(&self) -> Option<&str> {
        self.hint.as_deref()
    }

    #[cfg(feature = "unchecked-api")]
    #[inline]
    pub fn new_unchecked(
        spiffe_id: SpiffeId,
        svid: Box<str>,
        hint: Option<Box<str>>,
    ) -> Self {
        Self {
            spiffe_id,
            svid,
            hint,
        }
    }
}

impl TryFrom<ProtoJwtSvid> for JwtSvid {
    type Error = SpiffeError;

    fn try_from(value: ProtoJwtSvid) -> Result<Self, Self::Error> {
        Ok(Self {
            spiffe_id: SpiffeId::new(value.spiffe_id)?,
            svid: value.svid.into(),
            hint: if value.hint.is_empty() {
                None
            } else {
                Some(value.hint.into())
            },
        })
    }
}

type OwnedPrivatePkcs8KeyDer = Box<[u8]>;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct X509Svid {
    spiffe_id: SpiffeId,
    svid: Box<[CertificateDer<'static>]>,
    key: OwnedPrivatePkcs8KeyDer,
    bundle: Box<[CertificateDer<'static>]>,
    hint: Option<Box<str>>,
}

impl X509Svid {
    #[inline]
    pub fn spiffe_id(&self) -> &SpiffeId {
        &self.spiffe_id
    }

    #[inline]
    pub fn svid(&self) -> &[CertificateDer<'static>] {
        &self.svid
    }

    #[inline]
    pub fn key(&self) -> PrivatePkcs8KeyDer<'_> {
        PrivatePkcs8KeyDer::from(self.key.as_ref())
    }

    #[inline]
    pub fn bundle(&self) -> &[CertificateDer<'static>] {
        &self.bundle
    }

    #[inline]
    pub fn hint(&self) -> Option<&str> {
        self.hint.as_deref()
    }

    #[cfg(feature = "unchecked-api")]
    #[inline]
    pub fn new_unchecked(
        spiffe_id: SpiffeId,
        svid: Box<[CertificateDer<'static>]>,
        key: OwnedPrivatePkcs8KeyDer,
        bundle: Box<[CertificateDer<'static>]>,
        hint: Option<Box<str>>,
    ) -> Self {
        Self {
            spiffe_id,
            svid,
            key,
            bundle,
            hint,
        }
    }
}

impl TryFrom<ProtoX509Svid> for X509Svid {
    type Error = SpiffeError;

    fn try_from(value: ProtoX509Svid) -> Result<Self, Self::Error> {
        if value.x509_svid.is_empty() || value.x509_svid_key.is_empty() || value.bundle.is_empty() {
            return Err(SpiffeError::EmptySvid);
        }

        Ok(Self {
            spiffe_id: SpiffeId::new(value.spiffe_id)?,
            svid: split_certificates(&value.x509_svid)
                .map(|x| x.map(CertificateDer::into_owned))
                .collect::<Result<_, _>>()?,
            key: value.x509_svid_key.to_vec().into(),
            bundle: split_certificates(&value.bundle)
                .map(|x| x.map(CertificateDer::into_owned))
                .collect::<Result<_, _>>()?,
            hint: if value.hint.is_empty() {
                None
            } else {
                Some(value.hint.into())
            },
        })
    }
}
