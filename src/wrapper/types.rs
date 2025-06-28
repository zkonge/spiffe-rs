use prost::bytes::Bytes;
use rustls_pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use spiffe_id::SpiffeId;

use super::{InvalidDerDataError, SpiffeError, split_certificates};
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

impl From<X509Bundle> for Vec<CertificateDer<'static>> {
    fn from(bundle: X509Bundle) -> Self {
        bundle.bundle.into()
    }
}

type OwnedPrivatePkcs8KeyDer = Box<[u8]>;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct X509Svid {
    spiffe_id: SpiffeId,
    svid: Box<[CertificateDer<'static>]>,
    key: OwnedPrivatePkcs8KeyDer,
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
        PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(self.key.as_ref()))
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
        let key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(Vec::from(self.key)));

        (self.spiffe_id, self.svid.into(), key, self.bundle)
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
