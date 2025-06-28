mod stream;
mod types;

use std::iter;

use http::Uri;
use http_body::Body;
use prost::bytes::Bytes;
pub use prost_types::Struct as Claims;
use spiffe_id::SpiffeId;
use tonic::{
    Result, Status, body::Body as TonicBody, client::GrpcService, codec::CompressionEncoding,
};

pub use self::{
    stream::{JwtBundlesStream, X509BundlesContextStream, X509SvidContextStream},
    types::{X509BundlesContext, X509SvidContext},
};
use crate::{JwtSvid, SpiffeError, StdError, client::types::JwtSvidContext, proto};

impl From<SpiffeError> for Status {
    fn from(e: SpiffeError) -> Self {
        Status::from_error(Box::new(e))
    }
}

#[derive(Clone, Debug)]
pub struct SpiffeWorkloadApiClient<T> {
    client: proto::client::SpiffeWorkloadApiClient<T>,
}

impl<T> SpiffeWorkloadApiClient<T>
where
    T: GrpcService<TonicBody> + Clone,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
{
    pub fn with_origin(transport: T, origin: Uri) -> Self {
        Self {
            client: proto::client::SpiffeWorkloadApiClient::with_origin(transport, origin),
        }
    }

    /// Compress requests with the given encoding.
    /// This requires the server to support it otherwise it might respond with an
    /// error.
    #[must_use]
    pub fn send_compressed(self, encoding: CompressionEncoding) -> Self {
        Self {
            client: self.client.send_compressed(encoding),
        }
    }

    /// Enable decompressing responses.
    #[must_use]
    pub fn accept_compressed(self, encoding: CompressionEncoding) -> Self {
        Self {
            client: self.client.accept_compressed(encoding),
        }
    }

    /// Limits the maximum size of a decoded message.
    /// Default: `4MB`
    #[must_use]
    pub fn max_decoding_message_size(self, limit: usize) -> Self {
        Self {
            client: self.client.max_decoding_message_size(limit),
        }
    }

    /// Limits the maximum size of an encoded message.
    /// Default: [`usize::MAX`]
    #[must_use]
    pub fn max_encoding_message_size(self, limit: usize) -> Self {
        Self {
            client: self.client.max_encoding_message_size(limit),
        }
    }

    pub async fn fetch_x509_svid(&self) -> Result<X509SvidContextStream> {
        let request = proto::X509SvidRequest {};
        let response = self.client.clone().fetch_x509_svid(request).await?;

        Ok(X509SvidContextStream(response.into_inner()))
    }

    pub async fn fetch_x509_bundles(&self) -> Result<X509BundlesContextStream> {
        let request = proto::X509BundlesRequest {};
        let response = self.client.clone().fetch_x509_bundles(request).await?;

        Ok(X509BundlesContextStream(response.into_inner()))
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
            .map(|r: JwtSvidContext| r.svids)
            .map_err(Into::into)
    }

    pub async fn fetch_jwt_bundles(&self) -> Result<JwtBundlesStream> {
        let request = proto::JwtBundlesRequest {};
        let response = self.client.clone().fetch_jwt_bundles(request).await?;

        Ok(JwtBundlesStream(response.into_inner()))
    }

    pub async fn validate_jwt_svid(
        &self,
        audience: impl Into<String>,
        svid: impl Into<String>,
    ) -> Result<(SpiffeId, Claims)> {
        let request = proto::ValidateJwtSvidRequest {
            audience: audience.into(),
            svid: svid.into(),
        };
        let response = self.client.clone().validate_jwt_svid(request).await?;

        let proto::ValidateJwtSvidResponse { spiffe_id, claims } = response.into_inner();
        let spiffe_id = SpiffeId::new(spiffe_id).map_err(SpiffeError::SpiffeId)?;

        Ok((spiffe_id, claims.unwrap_or_default()))
    }
}
