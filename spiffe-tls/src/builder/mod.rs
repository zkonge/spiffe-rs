mod client;
mod server;

use std::{collections::HashMap, fmt::Display, future::Future, marker::PhantomData, sync::Arc};

use futures_util::{StreamExt, future::BoxFuture, stream::BoxStream};
use rustls_pki_types::{CertificateRevocationListDer, TrustAnchor};
use spiffe::{
    X509Bundle, X509Svid,
    client::{X509BundlesContextStream, X509SvidContextStream},
};
use spiffe_id::TrustDomain;
use tokio_rustls::rustls::{
    Error,
    crypto::CryptoProvider,
    pki_types::{CertificateDer, PrivateKeyDer},
    sign::CertifiedKey,
};
use upstre::{Upstre, UpstreBuilder};
use webpki::{CertRevocationList, OwnedCertRevocationList, anchor_from_trusted_cert};

pub use self::{client::ClientConfigBuilder, server::ServerConfigBuilder};
use crate::{error::pki_error, material::TlsMaterial};

type SourceFuture<S> = BoxFuture<'static, Result<S, Error>>;
type SourceMaker<S> = Arc<dyn Fn() -> SourceFuture<S> + Send + Sync>;
type SvidSelector = fn(&X509Svid) -> bool;

type SvidStreamMaker = SourceMaker<X509SvidContextStream>;
type BundleStreamMaker = SourceMaker<X509BundlesContextStream>;

#[derive(Clone)]
pub(super) enum MaterialSource {
    Svid(SvidStreamMaker),
    Bundle(BundleStreamMaker),
}

#[derive(Debug)]
pub struct Missing;

#[derive(Debug)]
pub struct Present;

#[derive(Debug)]
pub struct SourceMissing;

#[derive(Debug)]
pub struct SourceFromSvid;

#[derive(Debug)]
pub struct SourceFromBundle;

pub(super) struct BuilderCore<P, S, A> {
    pub(super) crypto_provider: Option<Arc<CryptoProvider>>,
    pub(super) source: Option<MaterialSource>,
    pub(super) peer_policy: Option<crate::policy::PeerAuthorizePolicy>,
    pub(super) svid_selector: SvidSelector,
    pub(super) _state: PhantomData<(P, S, A)>,
}

pub(super) fn make_svid_stream_maker<MakeSvid, FutSvid, ESvid>(
    make_svid_stream: MakeSvid,
) -> SvidStreamMaker
where
    MakeSvid: Fn() -> FutSvid + Send + Sync + 'static,
    FutSvid: Future<Output = Result<X509SvidContextStream, ESvid>> + Send + 'static,
    ESvid: Display,
{
    Arc::new(move || {
        let fut = make_svid_stream();
        Box::pin(async move { fut.await.map_err(display_error) })
    })
}

pub(super) fn make_bundle_stream_maker<MakeBundles, FutBundles, EBundle>(
    make_bundles_stream: MakeBundles,
) -> BundleStreamMaker
where
    MakeBundles: Fn() -> FutBundles + Send + Sync + 'static,
    FutBundles: Future<Output = Result<X509BundlesContextStream, EBundle>> + Send + 'static,
    EBundle: Display,
{
    Arc::new(move || {
        let fut = make_bundles_stream();
        Box::pin(async move { fut.await.map_err(display_error) })
    })
}

pub(super) async fn build_material_stream(
    source: Option<MaterialSource>,
    crypto_provider: Arc<CryptoProvider>,
    svid_selector: SvidSelector,
) -> Result<Upstre<TlsMaterial>, Error> {
    UpstreBuilder::default()
        .build(move || {
            let source = source.clone();
            let crypto_provider = crypto_provider.clone();

            async move {
                match source {
                    Some(MaterialSource::Svid(make_svid_stream)) => {
                        let svid_stream = make_svid_stream().await?;
                        Ok(material_stream_from_svid_contexts(
                            svid_stream,
                            crypto_provider,
                            svid_selector,
                        ))
                    }
                    Some(MaterialSource::Bundle(make_bundle_stream)) => {
                        let bundles_stream = make_bundle_stream().await?;
                        Ok(material_stream_from_bundle_contexts(bundles_stream))
                    }
                    None => Err(Error::General(
                        "exactly one x509 context stream source is required".into(),
                    )),
                }
            }
        })
        .await
        .map_err(map_upstre_error)
}

fn material_stream_from_svid_contexts(
    svid_stream: X509SvidContextStream,
    crypto_provider: Arc<CryptoProvider>,
    svid_selector: SvidSelector,
) -> BoxStream<'static, Result<TlsMaterial, Error>> {
    Box::pin(svid_stream.map(move |svid_context| {
        let identity = svid_context
            .svids
            .iter()
            .find(|&x| svid_selector(x))
            .cloned()
            .ok_or_else(|| {
                Error::General(
                    "no suitable x509 svid in context based on current svid selector".into(),
                )
            })?;

        let (spiffe_id, cert_chain, key, local_bundle) = identity.into_parts();
        let mut bundles = svid_context.federated_bundles.clone();
        bundles.insert(spiffe_id.trust_domain().into_owned(), local_bundle);

        let trust_anchors = trust_anchors_from_bundles(bundles)?;
        let parsed_crls = build_certificate_revocation_list(svid_context.crl.clone())?;
        let certified_key = build_certified_key(cert_chain, key, &crypto_provider)?;

        Ok(TlsMaterial {
            trust_anchors,
            crls: parsed_crls,
            certified_key: Some(Arc::new(certified_key)),
        })
    }))
}

fn material_stream_from_bundle_contexts(
    bundles_stream: X509BundlesContextStream,
) -> BoxStream<'static, Result<TlsMaterial, Error>> {
    Box::pin(bundles_stream.map(|bundles_context| {
        Ok(TlsMaterial {
            trust_anchors: trust_anchors_from_bundles(bundles_context.bundles.clone())?,
            crls: build_certificate_revocation_list(bundles_context.crl.clone())?,
            certified_key: None,
        })
    }))
}

fn trust_anchors_from_bundle(bundle: X509Bundle) -> Result<Vec<TrustAnchor<'static>>, Error> {
    bundle
        .into_parts()
        .into_iter()
        .map(|cert| {
            anchor_from_trusted_cert(&cert)
                .map(|anchor| anchor.to_owned())
                .map_err(pki_error)
        })
        .collect()
}

fn trust_anchors_from_bundles(
    bundles: HashMap<TrustDomain<'static>, X509Bundle>,
) -> Result<HashMap<TrustDomain<'static>, Vec<TrustAnchor<'static>>>, Error> {
    bundles
        .into_iter()
        .map(|(trust_domain, bundle)| {
            trust_anchors_from_bundle(bundle).map(|anchors| (trust_domain, anchors))
        })
        .collect()
}

fn build_certified_key(
    cert_chain: Vec<CertificateDer<'static>>,
    key: PrivateKeyDer<'static>,
    crypto_provider: &CryptoProvider,
) -> Result<CertifiedKey, Error> {
    let signing_key = crypto_provider.key_provider.load_private_key(key)?;
    Ok(CertifiedKey::new(cert_chain, signing_key))
}

fn build_certificate_revocation_list(
    crls: Vec<CertificateRevocationListDer<'static>>,
) -> Result<Vec<CertRevocationList<'static>>, Error> {
    crls.into_iter()
        .map(|crl| OwnedCertRevocationList::from_der(crl.as_ref()))
        .map(|r| r.map(CertRevocationList::from))
        .collect::<Result<Vec<_>, _>>()
        .map_err(pki_error)
}

pub(super) fn default_svid_selector(_: &X509Svid) -> bool {
    true
}

fn display_error(error: impl Display) -> Error {
    Error::General(error.to_string())
}

fn map_upstre_error(error: upstre::Error<Error>) -> Error {
    match error {
        upstre::Error::Error(error) => error,
        upstre::Error::EndOfStream => Error::General("SPIFFE TLS material stream ended".into()),
    }
}
