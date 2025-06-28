use std::collections::HashMap;

use prost::bytes::Bytes;
use rustls_pki_types::CertificateRevocationListDer;
use spiffe_id::TrustDomain;

use crate::{
    proto,
    wrapper::{JwtSvid, SpiffeError, X509Svid, types::X509Bundle},
};

fn parse_x509_bundles(
    bundles: HashMap<String, Bytes>,
) -> Result<HashMap<TrustDomain<'static>, X509Bundle>, SpiffeError> {
    bundles
        .into_iter()
        .map(|(td, bundle)| {
            let td = TrustDomain::try_from(td)?;
            let bundle = X509Bundle::try_from(bundle)?;

            Ok((td, bundle))
        })
        .collect()
}

#[derive(Clone, Debug)]
pub struct X509SvidContext {
    pub svids: Vec<X509Svid>,
    pub crl: Vec<CertificateRevocationListDer<'static>>,
    pub federated_bundles: HashMap<TrustDomain<'static>, X509Bundle>,
}

impl TryFrom<proto::X509SvidResponse> for X509SvidContext {
    type Error = SpiffeError;

    fn try_from(
        proto::X509SvidResponse {
            svids,
            crl,
            federated_bundles,
        }: proto::X509SvidResponse,
    ) -> Result<Self, Self::Error> {
        let svids = svids
            .into_iter()
            .map(X509Svid::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        let crl = crl
            .into_iter()
            .map(Into::<Vec<u8>>::into)
            .map(CertificateRevocationListDer::from)
            .collect();
        let federated_bundles = federated_bundles
            .into_iter()
            .map(|(td, bundle)| {
                let td: TrustDomain<'_> = TrustDomain::try_from(td)?;
                let bundle = X509Bundle::try_from(bundle)?;

                Ok::<_, Self::Error>((td, bundle))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(X509SvidContext {
            svids,
            crl,
            federated_bundles,
        })
    }
}

#[derive(Clone, Debug)]
pub struct X509BundlesContext {
    pub crl: Vec<CertificateRevocationListDer<'static>>,
    pub bundles: HashMap<TrustDomain<'static>, X509Bundle>,
}

impl TryFrom<proto::X509BundlesResponse> for X509BundlesContext {
    type Error = SpiffeError;

    fn try_from(
        proto::X509BundlesResponse { crl, bundles }: proto::X509BundlesResponse,
    ) -> Result<Self, Self::Error> {
        Ok(X509BundlesContext {
            crl: crl
                .into_iter()
                .map(Into::<Vec<u8>>::into)
                .map(CertificateRevocationListDer::from)
                .collect(),
            bundles: parse_x509_bundles(bundles)?,
        })
    }
}

// structs below is internal purpose only

#[derive(Clone, Debug)]
pub(super) struct JwtSvidContext {
    pub svids: Vec<JwtSvid>,
}

impl TryFrom<proto::JwtSvidResponse> for JwtSvidContext {
    type Error = SpiffeError;

    fn try_from(
        proto::JwtSvidResponse { svids }: proto::JwtSvidResponse,
    ) -> Result<Self, Self::Error> {
        svids
            .into_iter()
            .map(JwtSvid::try_from)
            .collect::<Result<_, _>>()
            .map(|svids| JwtSvidContext { svids })
    }
}

impl TryFrom<proto::JwtBundlesResponse> for HashMap<TrustDomain<'static>, String> {
    type Error = SpiffeError;

    fn try_from(
        proto::JwtBundlesResponse { bundles }: proto::JwtBundlesResponse,
    ) -> Result<Self, Self::Error> {
        let bundles = bundles
            .into_iter()
            .map(|(td, bundle)| {
                let td = TrustDomain::try_from(td)?;
                let bundle =
                    String::from_utf8(bundle.into()).map_err(|_| SpiffeError::InvalidJwtBundle)?;

                Ok::<_, Self::Error>((td, bundle))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(bundles)
    }
}
