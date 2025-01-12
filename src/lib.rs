mod proto;
#[cfg(feature = "wrapper")]
pub mod wrapper;

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub use crate::proto::*;
