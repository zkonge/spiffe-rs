use std::{fmt::Display, future::Future, marker::PhantomData, sync::Arc};

use spiffe::client::X509SvidContextStream;
use spiffe_id::SpiffeId;
use tokio_rustls::rustls::{Error, ServerConfig, crypto::CryptoProvider, version};

use super::{
    BuilderCore, MaterialSource, Missing, Present, SourceFromSvid, SourceMissing, SvidSelector,
    build_material_stream, default_svid_selector, make_svid_stream_maker,
};
use crate::{
    policy::PeerAuthorizePolicy, resolver::SpiffeCertResolver, verifier::SpiffeCertVerifier,
};

pub struct ClientAuthMandatory;
pub struct ClientAuthOptional;

pub struct ServerConfigBuilder<P = Missing, S = SourceMissing, A = Missing, R = Missing> {
    core: BuilderCore<P, S, A>,
    _require_peer_cert_state: PhantomData<R>,
}

impl ServerConfigBuilder<Missing, SourceMissing, Missing, Missing> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            core: BuilderCore {
                crypto_provider: None,
                source: None,
                peer_policy: None,
                svid_selector: default_svid_selector,
                _state: PhantomData,
            },
            _require_peer_cert_state: PhantomData,
        }
    }
}

impl<P, A, R> ServerConfigBuilder<P, SourceFromSvid, A, R> {
    #[must_use]
    pub fn with_svid_selector(mut self, selector: SvidSelector) -> Self {
        self.core.svid_selector = selector;
        self
    }
}

impl<P, S> ServerConfigBuilder<P, S, Missing, Missing> {
    #[must_use]
    pub fn client_auth_mandatory(self) -> ServerConfigBuilder<P, S, Missing, ClientAuthMandatory> {
        ServerConfigBuilder {
            core: self.core,
            _require_peer_cert_state: PhantomData,
        }
    }

    #[must_use]
    pub fn client_auth_optional(self) -> ServerConfigBuilder<P, S, Missing, ClientAuthOptional> {
        ServerConfigBuilder {
            core: BuilderCore {
                crypto_provider: self.core.crypto_provider,
                source: self.core.source,
                peer_policy: None,
                svid_selector: self.core.svid_selector,
                _state: PhantomData,
            },
            _require_peer_cert_state: PhantomData,
        }
    }
}

impl<S, A, R> ServerConfigBuilder<Missing, S, A, R> {
    #[must_use]
    pub fn with_crypto_provider(
        self,
        crypto_provider: Arc<CryptoProvider>,
    ) -> ServerConfigBuilder<Present, S, A, R> {
        ServerConfigBuilder {
            core: BuilderCore {
                crypto_provider: Some(crypto_provider),
                source: self.core.source,
                peer_policy: self.core.peer_policy,
                svid_selector: self.core.svid_selector,
                _state: PhantomData,
            },
            _require_peer_cert_state: PhantomData,
        }
    }
}

impl<P, A, R> ServerConfigBuilder<P, SourceMissing, A, R> {
    #[must_use]
    pub fn with_x509_svid_stream<MakeSvid, FutSvid, ESvid>(
        self,
        make_svid_stream: MakeSvid,
    ) -> ServerConfigBuilder<P, SourceFromSvid, A, R>
    where
        MakeSvid: Fn() -> FutSvid + Send + Sync + 'static,
        FutSvid: Future<Output = Result<X509SvidContextStream, ESvid>> + Send + 'static,
        ESvid: Display,
    {
        ServerConfigBuilder {
            core: BuilderCore {
                crypto_provider: self.core.crypto_provider,
                source: Some(MaterialSource::Svid(make_svid_stream_maker(
                    make_svid_stream,
                ))),
                peer_policy: self.core.peer_policy,
                svid_selector: self.core.svid_selector,
                _state: PhantomData,
            },
            _require_peer_cert_state: PhantomData,
        }
    }
}

impl<P, S> ServerConfigBuilder<P, S, Missing, ClientAuthMandatory> {
    #[must_use]
    pub fn allow_any_authenticated_client(
        self,
    ) -> ServerConfigBuilder<P, S, Present, ClientAuthMandatory> {
        self.with_peer_policy(PeerAuthorizePolicy::AllowAny)
    }

    #[must_use]
    pub fn expect_client(
        self,
        client_id: SpiffeId,
    ) -> ServerConfigBuilder<P, S, Present, ClientAuthMandatory> {
        self.with_peer_policy(PeerAuthorizePolicy::Exact(client_id))
    }

    #[must_use]
    pub fn authorize_client_with(
        self,
        verifier: fn(&SpiffeId) -> bool,
    ) -> ServerConfigBuilder<P, S, Present, ClientAuthMandatory> {
        self.with_peer_policy(PeerAuthorizePolicy::Dynamic(verifier))
    }

    fn with_peer_policy(
        self,
        peer_policy: PeerAuthorizePolicy,
    ) -> ServerConfigBuilder<P, S, Present, ClientAuthMandatory> {
        ServerConfigBuilder {
            core: BuilderCore {
                crypto_provider: self.core.crypto_provider,
                source: self.core.source,
                peer_policy: Some(peer_policy),
                svid_selector: self.core.svid_selector,
                _state: PhantomData,
            },
            _require_peer_cert_state: PhantomData,
        }
    }
}

impl ServerConfigBuilder<Present, SourceFromSvid, Present, ClientAuthMandatory> {
    pub async fn build(self) -> Result<ServerConfig, Error> {
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

        let config = ServerConfig::builder_with_provider(crypto_provider)
            .with_protocol_versions(&[&version::TLS13])?
            .with_client_cert_verifier(verifier)
            .with_cert_resolver(resolver);

        Ok(config)
    }
}

impl<A> ServerConfigBuilder<Present, SourceFromSvid, A, ClientAuthOptional> {
    pub async fn build(self) -> Result<ServerConfig, Error> {
        let crypto_provider = self
            .core
            .crypto_provider
            .expect("builder state mismatch: crypto provider");

        let material = build_material_stream(
            self.core.source,
            crypto_provider.clone(),
            self.core.svid_selector,
        )
        .await?;

        let verifier = Arc::new(SpiffeCertVerifier::new(
            material.clone(),
            crypto_provider.clone(),
            PeerAuthorizePolicy::AllowAny,
            false,
        ));
        let resolver = Arc::new(SpiffeCertResolver::new(material));

        let config = ServerConfig::builder_with_provider(crypto_provider)
            .with_protocol_versions(&[&version::TLS13])?
            .with_client_cert_verifier(verifier)
            .with_cert_resolver(resolver);

        Ok(config)
    }
}

impl Default for ServerConfigBuilder<Missing, SourceMissing, Missing, Missing> {
    fn default() -> Self {
        Self::new()
    }
}
