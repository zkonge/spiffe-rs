use prost::Message;

use crate::{JwtSvid, SpiffeId, Status, WitSvid, X509Svid};

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct MintX509SvidRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub csr: Vec<u8>,

    #[prost(int32, tag = "2")]
    pub ttl: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct MintX509SvidResponse {
    #[prost(message, optional, tag = "1")]
    pub svid: Option<X509Svid>,
}

#[derive(Clone, PartialEq, Message)]
pub struct MintWitSvidRequest {
    #[prost(message, optional, tag = "1")]
    pub id: Option<SpiffeId>,

    #[prost(bytes = "vec", tag = "2")]
    pub public_key: Vec<u8>,

    #[prost(int32, tag = "3")]
    pub ttl: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct MintWitSvidResponse {
    #[prost(message, optional, tag = "1")]
    pub svid: Option<WitSvid>,
}

#[derive(Clone, PartialEq, Message)]
pub struct MintJwtSvidRequest {
    #[prost(message, optional, tag = "1")]
    pub id: Option<SpiffeId>,

    #[prost(string, repeated, tag = "2")]
    pub audience: Vec<String>,

    #[prost(int32, tag = "3")]
    pub ttl: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct MintJwtSvidResponse {
    #[prost(message, optional, tag = "1")]
    pub svid: Option<JwtSvid>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchNewX509SvidRequest {
    #[prost(message, repeated, tag = "1")]
    pub params: Vec<NewX509SvidParams>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchNewX509SvidResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<BatchNewX509SvidResponseResult>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchNewX509SvidResponseResult {
    #[prost(message, optional, tag = "1")]
    pub status: Option<Status>,

    #[prost(message, optional, tag = "2")]
    pub svid: Option<X509Svid>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct NewJwtSvidRequest {
    #[prost(string, tag = "1")]
    pub entry_id: String,

    #[prost(string, repeated, tag = "2")]
    pub audience: Vec<String>,
}

#[derive(Clone, PartialEq, Message)]
pub struct NewJwtSvidResponse {
    #[prost(message, optional, tag = "1")]
    pub svid: Option<JwtSvid>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchNewWitSvidRequest {
    #[prost(message, repeated, tag = "1")]
    pub params: Vec<NewWitSvidParams>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchNewWitSvidResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<BatchNewWitSvidResponseResult>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchNewWitSvidResponseResult {
    #[prost(message, optional, tag = "1")]
    pub status: Option<Status>,

    #[prost(message, optional, tag = "2")]
    pub svid: Option<WitSvid>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct NewDownstreamX509CaRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub csr: Vec<u8>,

    #[prost(int32, tag = "2")]
    pub preferred_ttl: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct NewDownstreamX509CaResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub ca_cert_chain: Vec<Vec<u8>>,

    #[prost(bytes = "vec", repeated, tag = "2")]
    pub x509_authorities: Vec<Vec<u8>>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct NewX509SvidParams {
    #[prost(string, tag = "1")]
    pub entry_id: String,

    #[prost(bytes = "vec", tag = "2")]
    pub csr: Vec<u8>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct NewWitSvidParams {
    #[prost(string, tag = "1")]
    pub entry_id: String,

    #[prost(bytes = "vec", tag = "2")]
    pub public_key: Vec<u8>,
}
