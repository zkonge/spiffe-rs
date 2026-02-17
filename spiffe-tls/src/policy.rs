use std::fmt::Debug;

use spiffe_id::SpiffeId;

#[derive(Clone, Debug)]
pub enum PeerAuthorizePolicy {
    AllowAny,
    Exact(SpiffeId),
    Dynamic(fn(&SpiffeId) -> bool),
}

impl PeerAuthorizePolicy {
    pub(crate) fn matches(&self, id: &SpiffeId) -> bool {
        match self {
            Self::AllowAny => true,
            Self::Exact(expected) => id == expected,
            Self::Dynamic(f) => f(id),
        }
    }
}
