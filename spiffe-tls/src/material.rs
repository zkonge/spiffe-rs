use std::{collections::HashMap, sync::Arc};

use spiffe_id::TrustDomain;
use tokio_rustls::rustls::{pki_types::TrustAnchor, sign::CertifiedKey};
use webpki::CertRevocationList;

#[derive(Debug)]
pub(crate) struct TlsMaterial {
    pub(crate) trust_anchors: HashMap<TrustDomain<'static>, Vec<TrustAnchor<'static>>>,
    pub(crate) crls: Vec<CertRevocationList<'static>>,
    pub(crate) certified_key: Option<Arc<CertifiedKey>>,
}
