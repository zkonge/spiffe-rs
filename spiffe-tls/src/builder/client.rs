use std::{fmt::Display, future::Future, sync::Arc};

use spiffe::client::{X509BundlesContextStream, X509SvidContextStream};
use spiffe_id::SpiffeId;
use tokio_rustls::rustls::{ClientConfig, Error, crypto::CryptoProvider, version};

use super::{
    BuilderCore, MaterialSource, Missing, Present, SourceFromBundle, SourceFromSvid, SourceMissing,
    SvidSelector, build_material_stream, default_svid_selector, make_bundle_stream_maker,
    make_svid_stream_maker,
};
use crate::{
    policy::PeerAuthorizePolicy, resolver::SpiffeCertResolver, verifier::SpiffeCertVerifier,
};

pub struct ClientConfigBuilder<P = Missing, S = SourceMissing, A = Missing> {
    core: BuilderCore<P, S, A>,
}

impl ClientConfigBuilder<Missing, SourceMissing, Missing> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            core: BuilderCore {
                crypto_provider: None,
                source: None,
                peer_policy: None,
                svid_selector: default_svid_selector,
                _state: std::marker::PhantomData,
            },
        }
    }
}

impl<P, A> ClientConfigBuilder<P, SourceFromSvid, A> {
    #[must_use]
    pub fn with_svid_selector(mut self, selector: SvidSelector) -> Self {
        self.core.svid_selector = selector;
        self
    }
}

impl<S, A> ClientConfigBuilder<Missing, S, A> {
    #[must_use]
    pub fn with_crypto_provider(
        self,
        crypto_provider: Arc<CryptoProvider>,
    ) -> ClientConfigBuilder<Present, S, A> {
        ClientConfigBuilder {
            core: BuilderCore {
                crypto_provider: Some(crypto_provider),
                source: self.core.source,
                peer_policy: self.core.peer_policy,
                svid_selector: self.core.svid_selector,
                _state: std::marker::PhantomData,
            },
        }
    }
}

impl<P, A> ClientConfigBuilder<P, SourceMissing, A> {
    #[must_use]
    pub fn with_x509_svid_stream<MakeSvid, FutSvid, ESvid>(
        self,
        make_svid_stream: MakeSvid,
    ) -> ClientConfigBuilder<P, SourceFromSvid, A>
    where
        MakeSvid: Fn() -> FutSvid + Send + Sync + 'static,
        FutSvid: Future<Output = Result<X509SvidContextStream, ESvid>> + Send + 'static,
        ESvid: Display,
    {
        ClientConfigBuilder {
            core: BuilderCore {
                crypto_provider: self.core.crypto_provider,
                source: Some(MaterialSource::Svid(make_svid_stream_maker(
                    make_svid_stream,
                ))),
                peer_policy: self.core.peer_policy,
                svid_selector: self.core.svid_selector,
                _state: std::marker::PhantomData,
            },
        }
    }

    #[must_use]
    pub fn with_x509_bundle_stream<MakeBundles, FutBundles, EBundle>(
        self,
        make_bundles_stream: MakeBundles,
    ) -> ClientConfigBuilder<P, SourceFromBundle, A>
    where
        MakeBundles: Fn() -> FutBundles + Send + Sync + 'static,
        FutBundles: Future<Output = Result<X509BundlesContextStream, EBundle>> + Send + 'static,
        EBundle: Display,
    {
        ClientConfigBuilder {
            core: BuilderCore {
                crypto_provider: self.core.crypto_provider,
                source: Some(MaterialSource::Bundle(make_bundle_stream_maker(
                    make_bundles_stream,
                ))),
                peer_policy: self.core.peer_policy,
                svid_selector: self.core.svid_selector,
                _state: std::marker::PhantomData,
            },
        }
    }
}

impl<P, S> ClientConfigBuilder<P, S, Missing> {
    #[must_use]
    pub fn expect_server(self, server_id: SpiffeId) -> ClientConfigBuilder<P, S, Present> {
        self.with_peer_policy(PeerAuthorizePolicy::Exact(server_id))
    }

    #[must_use]
    pub fn authorize_server_with(
        self,
        verifier: fn(&SpiffeId) -> bool,
    ) -> ClientConfigBuilder<P, S, Present> {
        self.with_peer_policy(PeerAuthorizePolicy::Dynamic(verifier))
    }

    fn with_peer_policy(
        self,
        peer_policy: PeerAuthorizePolicy,
    ) -> ClientConfigBuilder<P, S, Present> {
        ClientConfigBuilder {
            core: BuilderCore {
                crypto_provider: self.core.crypto_provider,
                source: self.core.source,
                peer_policy: Some(peer_policy),
                svid_selector: self.core.svid_selector,
                _state: std::marker::PhantomData,
            },
        }
    }
}

impl<S> ClientConfigBuilder<Present, S, Present> {
    pub async fn build(self) -> Result<ClientConfig, Error> {
        let crypto_provider = self
            .core
            .crypto_provider
            .expect("builder state mismatch: crypto provider");
        let peer_policy = self
            .core
            .peer_policy
            .expect("builder state mismatch: peer policy");

        let material = build_material_stream(
            self.core.source,
            crypto_provider.clone(),
            self.core.svid_selector,
        )
        .await?;

        let verifier = Arc::new(SpiffeCertVerifier::new(
            material.clone(),
            crypto_provider.clone(),
            peer_policy,
            true,
        ));
        let resolver = Arc::new(SpiffeCertResolver::new(material));

        let config = ClientConfig::builder_with_provider(crypto_provider)
            .with_protocol_versions(&[&version::TLS13])?
            .dangerous()
            .with_custom_certificate_verifier(verifier)
            .with_client_cert_resolver(resolver);

        Ok(config)
    }
}

impl Default for ClientConfigBuilder<Missing, SourceMissing, Missing> {
    fn default() -> Self {
        Self::new()
    }
}
