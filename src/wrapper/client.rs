mod stream;
mod types;

use std::iter;

use http_body::Body;
use prost::bytes::Bytes;
use tonic::{Result, Status, body::Body as TonicBody, client::GrpcService};

pub use self::{
    stream::{JwtBundlesStream, X509BundlesResponseStream, X509SvidResponseStream},
    types::{X509BundlesResponse, X509SvidResponse},
};
use crate::{
    StdError,
    client::SpiffeWorkloadApiClient,
    proto,
    wrapper::{JwtSvid, SpiffeError, client::types::JwtSvidResponse},
};

impl From<SpiffeError> for Status {
    fn from(e: SpiffeError) -> Self {
        Status::from_error(Box::new(e))
    }
}

#[derive(Clone, Debug)]
pub struct Client<T> {
    client: SpiffeWorkloadApiClient<T>,
}

impl<T> Client<T>
where
    T: GrpcService<TonicBody> + Clone,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
{
    pub fn new(client: SpiffeWorkloadApiClient<T>) -> Self {
        Self { client }
    }

    pub async fn fetch_x509_svid(&self) -> Result<X509SvidResponseStream> {
        let request = proto::X509SvidRequest {};
        let response = self.client.clone().fetch_x509_svid(request).await?;

        Ok(X509SvidResponseStream(response.into_inner()))
    }

    pub async fn fetch_x509_bundles(&self) -> Result<X509BundlesResponseStream> {
        let request = proto::X509BundlesRequest {};
        let response = self.client.clone().fetch_x509_bundles(request).await?;

        Ok(X509BundlesResponseStream(response.into_inner()))
    }

    pub async fn fetch_jwt_svid(
        &self,
        audience: impl Into<String>,
        more_audiences: impl Into<Vec<String>>,
        spiffe_id: Option<String>,
    ) -> Result<Vec<JwtSvid>> {
        let request = proto::JwtSvidRequest {
            audience: iter::once(audience.into())
                .chain(more_audiences.into().into_iter())
                .collect(),
            spiffe_id: spiffe_id.unwrap_or_default(),
        };
        let response = self.client.clone().fetch_jwt_svid(request).await?;

        response
            .into_inner()
            .try_into()
            .map(|r: JwtSvidResponse| r.svids)
            .map_err(Into::into)
    }

    pub async fn fetch_jwt_bundles(&self) -> Result<JwtBundlesStream> {
        let request = proto::JwtBundlesRequest {};
        let response = self.client.clone().fetch_jwt_bundles(request).await?;

        Ok(JwtBundlesStream(response.into_inner()))
    }
}
