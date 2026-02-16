mod stream;
mod types;

use std::iter;

use http::Uri;
use http_body::Body;
use prost::bytes::Bytes;
use tonic::{Result, Status, body::Body as TonicBody, client::GrpcService};

use self::types::JwtSvidContext;
pub use self::{
    stream::{JwtBundlesStream, X509BundlesContextStream, X509SvidContextStream},
    types::{X509BundlesContext, X509SvidContext},
};
use crate::{JwtSvid, SpiffeError, StdError};

impl From<SpiffeError> for Status {
    fn from(e: SpiffeError) -> Self {
        Status::from_error(Box::new(e))
    }
}

#[derive(Clone, Debug)]
pub struct SpiffeWorkloadApiClient<T> {
    client: spiffe_proto::client::SpiffeWorkloadApiClient<T>,
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
            client: spiffe_proto::client::SpiffeWorkloadApiClient::with_origin(transport, origin),
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
        let request = spiffe_proto::X509SvidRequest {};
        let response = self.client.clone().fetch_x509_svid(request).await?;

        Ok(X509SvidContextStream(response.into_inner()))
    }

    pub async fn fetch_x509_bundles(&self) -> Result<X509BundlesContextStream> {
        let request = spiffe_proto::X509BundlesRequest {};
        let response = self.client.clone().fetch_x509_bundles(request).await?;

        Ok(X509BundlesContextStream(response.into_inner()))
    }

    pub async fn fetch_jwt_svid(
        &self,
        audience: impl Into<String>,
        more_audiences: impl Into<Vec<String>>,
        spiffe_id: Option<String>,
    ) -> Result<Vec<JwtSvid>> {
        let request = spiffe_proto::JwtSvidRequest {
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
        let request = spiffe_proto::JwtBundlesRequest {};
        let response = self.client.clone().fetch_jwt_bundles(request).await?;

        Ok(JwtBundlesStream(response.into_inner()))
    }

    pub async fn validate_jwt_svid(
        &self,
        audience: impl Into<String>,
        svid: impl Into<String>,
    ) -> Result<(spiffe_id::SpiffeId, serde_json::Value)> {
        let request = spiffe_proto::ValidateJwtSvidRequest {
            audience: audience.into(),
            svid: svid.into(),
        };
        let response = self.client.clone().validate_jwt_svid(request).await?;

        let spiffe_proto::ValidateJwtSvidResponse { spiffe_id, claims } = response.into_inner();
        let spiffe_id = spiffe_id::SpiffeId::new(spiffe_id).map_err(SpiffeError::SpiffeId)?;

        Ok((
            spiffe_id,
            claims.map_or(serde_json::Value::Null, |v| {
                claims_from_prost_value(prost_types::Value {
                    kind: Some(prost_types::value::Kind::StructValue(v)),
                })
            }),
        ))
    }
}

// blocked by https://github.com/tokio-rs/prost/issues/852
fn claims_from_prost_value(i: prost_types::Value) -> serde_json::Value {
    use prost_types::value::Kind;
    use serde_json::{Number, Value};

    let i = match i.kind {
        Some(i) => i,
        None => return Value::Null,
    };

    match i {
        Kind::NullValue(_) => Value::Null,
        Kind::NumberValue(n) => Number::from_f64(n).map_or(Value::Null, Value::Number),
        Kind::StringValue(s) => Value::String(s),
        Kind::BoolValue(b) => Value::Bool(b),
        Kind::StructValue(x) => x
            .fields
            .into_iter()
            .map(|(k, v)| (k, claims_from_prost_value(v)))
            .collect(),
        Kind::ListValue(x) => x.values.into_iter().map(claims_from_prost_value).collect(),
    }
}
