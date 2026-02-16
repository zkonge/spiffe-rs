use std::collections::HashMap;

use prost::bytes::Bytes;
use rustls_pki_types::CertificateRevocationListDer;
use spiffe_id::TrustDomain;

use crate::{JwtSvid, SpiffeError, X509Bundle, X509Svid};

fn paese_x509_crls(crls: Vec<Bytes>) -> Vec<CertificateRevocationListDer<'static>> {
    crls.into_iter()
        .map(Into::<Vec<u8>>::into)
        .map(Into::into)
        .collect()
}

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

impl TryFrom<spiffe_proto::X509SvidResponse> for X509SvidContext {
    type Error = SpiffeError;

    fn try_from(
        spiffe_proto::X509SvidResponse {
            svids,
            crl,
            federated_bundles,
        }: spiffe_proto::X509SvidResponse,
    ) -> Result<Self, Self::Error> {
        let svids = svids
            .into_iter()
            .map(X509Svid::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        let crl = paese_x509_crls(crl);
        let federated_bundles = parse_x509_bundles(federated_bundles)?;

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

impl TryFrom<spiffe_proto::X509BundlesResponse> for X509BundlesContext {
    type Error = SpiffeError;

    fn try_from(
        spiffe_proto::X509BundlesResponse { crl, bundles }: spiffe_proto::X509BundlesResponse,
    ) -> Result<Self, Self::Error> {
        Ok(X509BundlesContext {
            crl: paese_x509_crls(crl),
            bundles: parse_x509_bundles(bundles)?,
        })
    }
}

// following structs are internal purpose only

#[derive(Clone, Debug)]
pub(super) struct JwtSvidContext {
    pub svids: Vec<JwtSvid>,
}

impl TryFrom<spiffe_proto::JwtSvidResponse> for JwtSvidContext {
    type Error = SpiffeError;

    fn try_from(
        spiffe_proto::JwtSvidResponse { svids }: spiffe_proto::JwtSvidResponse,
    ) -> Result<Self, Self::Error> {
        svids
            .into_iter()
            .map(JwtSvid::try_from)
            .collect::<Result<_, _>>()
            .map(|svids| JwtSvidContext { svids })
    }
}

#[derive(Clone, Debug)]
pub(super) struct JwtBundlesContext {
    pub bundles: HashMap<TrustDomain<'static>, String>,
}

impl TryFrom<spiffe_proto::JwtBundlesResponse> for JwtBundlesContext {
    type Error = SpiffeError;

    fn try_from(
        spiffe_proto::JwtBundlesResponse { bundles }: spiffe_proto::JwtBundlesResponse,
    ) -> Result<Self, Self::Error> {
        bundles
            .into_iter()
            .map(|(td, bundle)| {
                let td = TrustDomain::try_from(td)?;
                let bundle =
                    String::from_utf8(bundle.into()).map_err(|_| SpiffeError::InvalidJwtBundle)?;

                Ok::<_, Self::Error>((td, bundle))
            })
            .collect::<Result<HashMap<_, _>, _>>()
            .map(|bundles| JwtBundlesContext { bundles })
    }
}
