mod service;
mod stream;

use std::path::Path;

use http::Uri;
use spiffe_id::SpiffeId;
use stream::X509SvidStream;

use self::service::UnixSpiffeClient;
use super::error::SpiffeWorkloadClientError;
use crate::{wrapper::JwtSvid, JwtSvidRequest, SpiffeWorkloadApiClient, X509SvidRequest};

type Result<T> = std::result::Result<T, SpiffeWorkloadClientError>;

pub struct SpiffeWorkloadClient {
    client: SpiffeWorkloadApiClient<UnixSpiffeClient>,
}

impl SpiffeWorkloadClient {
    pub async fn new(path: impl AsRef<Path>) -> Result<Self> {
        let client = UnixSpiffeClient::new(path).await?;
        let client = SpiffeWorkloadApiClient::with_origin(client, Uri::from_static("http://[::1]"));

        Ok(Self { client })
    }

    pub async fn fetch_jwt_svid(
        &self,
        audience: Vec<String>,
        spiffe_id: Option<SpiffeId>,
    ) -> Result<Vec<JwtSvid>> {
        let resp = self
            .client
            .clone()
            .fetch_jwt_svid(JwtSvidRequest {
                audience,
                spiffe_id: spiffe_id.map(Into::into).unwrap_or_default(),
            })
            .await?;

        resp.into_inner()
            .svids
            .into_iter()
            .map(|s| JwtSvid::try_from(s).map_err(Into::into))
            .collect()
    }

    pub async fn fetch_x509_svid(&self) -> Result<X509SvidStream> {
        let resp = self
            .client
            .clone()
            .fetch_x509_svid(X509SvidRequest {})
            .await?;

        Ok(resp.into_inner().into())
    }
}
