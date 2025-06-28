extern crate alloc;

use std::fmt::{Debug, Formatter, Result as FmtResult};

use alloc::{boxed::Box, string::String, vec::Vec};

use prost::bytes::Bytes;
use rustls_pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use spiffe_id::SpiffeId;

use super::{InvalidDerDataError, SpiffeError, split_certificates};
use crate::proto::{JwtSvid as ProtoJwtSvid, X509Svid as ProtoX509Svid};

#[derive(Clone, PartialEq, Eq, Hash)]
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

    #[inline]
    pub fn into_parts(self) -> (SpiffeId, String) {
        (self.spiffe_id, self.svid.into())
    }

    #[cfg(feature = "unchecked-api")]
    #[inline]
    pub fn new_unchecked(spiffe_id: SpiffeId, svid: Box<str>, hint: Option<Box<str>>) -> Self {
        Self {
            spiffe_id,
            svid,
            hint,
        }
    }
}

impl Debug for JwtSvid {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("JwtSvid")
            .field("spiffe_id", &self.spiffe_id)
            .field("svid", &"[secret elided]")
            .field("hint", &self.hint)
            .finish()
    }
}

impl TryFrom<ProtoJwtSvid> for JwtSvid {
    type Error = SpiffeError;

    fn try_from(
        ProtoJwtSvid {
            spiffe_id,
            svid,
            hint,
        }: ProtoJwtSvid,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            spiffe_id: SpiffeId::new(spiffe_id)?,
            svid: svid.into(),
            hint: if hint.is_empty() {
                None
            } else {
                Some(hint.into())
            },
        })
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct X509Bundle {
    bundle: Box<[CertificateDer<'static>]>,
}

impl X509Bundle {
    #[inline]
    pub fn bundle(&self) -> &[CertificateDer<'static>] {
        &self.bundle
    }

    #[inline]
    pub fn into_parts(self) -> Vec<CertificateDer<'static>> {
        self.bundle.into()
    }

    #[cfg(feature = "unchecked-api")]
    #[inline]
    pub fn new_unchecked(bundle: Box<[CertificateDer<'static>]>) -> Self {
        Self { bundle }
    }
}

impl TryFrom<Bytes> for X509Bundle {
    type Error = SpiffeError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(SpiffeError::InvalidDerData(InvalidDerDataError));
        }

        Ok(Self {
            bundle: split_certificates(&value)
                .map(|x| x.map(CertificateDer::into_owned))
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct X509Svid {
    spiffe_id: SpiffeId,
    svid: Box<[CertificateDer<'static>]>,
    key: PrivatePkcs8KeyDer<'static>,
    bundle: X509Bundle,
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
    pub fn key(&self) -> PrivateKeyDer<'_> {
        PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(self.key.secret_pkcs8_der()))
    }

    #[inline]
    pub fn bundle(&self) -> &X509Bundle {
        &self.bundle
    }

    #[inline]
    pub fn hint(&self) -> Option<&str> {
        self.hint.as_deref()
    }

    pub fn into_parts(
        self,
    ) -> (
        SpiffeId,
        Vec<CertificateDer<'static>>,
        PrivateKeyDer<'static>,
        X509Bundle,
    ) {
        (
            self.spiffe_id,
            self.svid.into(),
            PrivateKeyDer::Pkcs8(self.key),
            self.bundle,
        )
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

impl Clone for X509Svid {
    fn clone(&self) -> Self {
        Self {
            spiffe_id: self.spiffe_id.clone(),
            svid: self.svid.clone(),
            key: self.key.clone_key(),
            bundle: self.bundle.clone(),
            hint: self.hint.clone(),
        }
    }
}

impl TryFrom<ProtoX509Svid> for X509Svid {
    type Error = SpiffeError;

    fn try_from(
        ProtoX509Svid {
            spiffe_id,
            x509_svid,
            x509_svid_key,
            bundle,
            hint,
        }: ProtoX509Svid,
    ) -> Result<Self, Self::Error> {
        if x509_svid.is_empty() || x509_svid_key.is_empty() || bundle.is_empty() {
            return Err(SpiffeError::EmptySvid);
        }

        Ok(Self {
            spiffe_id: SpiffeId::new(spiffe_id)?,
            svid: split_certificates(&x509_svid)
                .map(|x| x.map(CertificateDer::into_owned))
                .collect::<Result<_, _>>()?,
            key: x509_svid_key.to_vec().into(),
            bundle: bundle.try_into()?,
            hint: if hint.is_empty() {
                None
            } else {
                Some(hint.into())
            },
        })
    }
}
