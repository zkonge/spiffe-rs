use tonic::{
    metadata::{MetadataKey, MetadataValue},
    service::Interceptor,
    Request, Status,
};

const SPIFFE_METADATA_KEY: &str = "workload.spiffe.io";
const SPIFFE_METADATA_VALUE: &str = "true";

/// Used by the client to add the security header
#[derive(Debug)]
pub struct SpiffeMetadataAppender;

impl Interceptor for SpiffeMetadataAppender {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        request.metadata_mut().insert(
            MetadataKey::from_static(SPIFFE_METADATA_KEY),
            MetadataValue::from_static(SPIFFE_METADATA_VALUE),
        );

        Ok(request)
    }
}

/// Used by the server to verify the presence of the security header
#[derive(Debug)]
pub struct SpiffeMetadataVerifier;

impl Interceptor for SpiffeMetadataVerifier {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        if let Some(v) = request.metadata().get(SPIFFE_METADATA_KEY) {
            if v.as_bytes() == SPIFFE_METADATA_VALUE.as_bytes() {
                return Ok(request);
            }
        }

        Err(Status::invalid_argument(
            "security header missing from request",
        ))
    }
}
