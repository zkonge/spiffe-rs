use std::sync::Arc;

use tokio_rustls::rustls::{
    SignatureScheme,
    client::ResolvesClientCert,
    server::{ClientHello, ResolvesServerCert},
    sign::CertifiedKey,
};
use upstre::Upstre;

use crate::material::TlsMaterial;

#[derive(Debug)]
pub(crate) struct SpiffeCertResolver {
    material: Upstre<TlsMaterial>,
}

impl SpiffeCertResolver {
    pub(crate) fn new(material: Upstre<TlsMaterial>) -> Self {
        Self { material }
    }
}

impl ResolvesServerCert for SpiffeCertResolver {
    fn resolve(&self, _: ClientHello<'_>) -> Option<Arc<CertifiedKey>> {
        self.material.value().certified_key.clone()
    }
}

impl ResolvesClientCert for SpiffeCertResolver {
    fn resolve(&self, _: &[&[u8]], _: &[SignatureScheme]) -> Option<Arc<CertifiedKey>> {
        self.material.value().certified_key.clone()
    }

    fn has_certs(&self) -> bool {
        self.material.value().certified_key.is_some()
    }
}
