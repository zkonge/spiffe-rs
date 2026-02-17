use std::sync::Arc;

use rustls_pki_types::SignatureVerificationAlgorithm;
use spiffe::spiffe_id_from_x509_svid_unchecked;
use spiffe_id::SpiffeId;
use tokio_rustls::rustls::{
    CertificateError, DigitallySignedStruct, DistinguishedName, Error, PeerMisbehaved,
    SignatureAlgorithm, SignatureScheme,
    client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier},
    crypto::{CryptoProvider, WebPkiSupportedAlgorithms, hash::HashAlgorithm},
    pki_types::{CertificateDer, ServerName, UnixTime},
    server::danger::{ClientCertVerified, ClientCertVerifier},
};
use upstre::Upstre;
use webpki::{EndEntityCert, KeyUsage, RevocationOptionsBuilder, UnknownStatusPolicy};

use crate::{error::pki_error, material::TlsMaterial, policy::PeerAuthorizePolicy};

#[derive(Debug)]
pub(crate) struct SpiffeCertVerifier {
    material: Upstre<TlsMaterial>,
    supported_schemes: WebPkiSupportedAlgorithms,
    peer_spiffe_id_verifier: PeerAuthorizePolicy,
    require_peer_cert_in_server_side: bool,
}

impl SpiffeCertVerifier {
    pub(crate) fn new(
        material: Upstre<TlsMaterial>,
        crypto_provider: Arc<CryptoProvider>,
        peer_spiffe_id_verifier: PeerAuthorizePolicy,
        require_peer_cert_in_server_side: bool,
    ) -> Self {
        Self {
            material,
            supported_schemes: crypto_provider.signature_verification_algorithms,
            peer_spiffe_id_verifier,
            require_peer_cert_in_server_side,
        }
    }
}

impl ClientCertVerifier for SpiffeCertVerifier {
    fn root_hint_subjects(&self) -> &[DistinguishedName] {
        &[]
    }

    fn verify_client_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        now: UnixTime,
    ) -> Result<ClientCertVerified, Error> {
        let id = verify_cert(
            &self.material.value(),
            &self.supported_schemes,
            KeyUsage::client_auth(),
            end_entity,
            intermediates,
            now,
        )?;

        if !self.peer_spiffe_id_verifier.matches(&id) {
            return Err(Error::InvalidCertificate(
                CertificateError::ApplicationVerificationFailure,
            ));
        }

        Ok(ClientCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        verify_tls12_signature(message, cert, dss, &self.supported_schemes)
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        verify_tls13_signature(message, cert, dss, &self.supported_schemes)
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.supported_schemes.supported_schemes()
    }

    fn offer_client_auth(&self) -> bool {
        true
    }

    fn client_auth_mandatory(&self) -> bool {
        self.require_peer_cert_in_server_side
    }
}

impl ServerCertVerifier for SpiffeCertVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        _: &ServerName<'_>,
        _: &[u8],
        now: UnixTime,
    ) -> Result<ServerCertVerified, Error> {
        let peer_id = verify_cert(
            &self.material.value(),
            &self.supported_schemes,
            KeyUsage::server_auth(),
            end_entity,
            intermediates,
            now,
        )?;

        if !self.peer_spiffe_id_verifier.matches(&peer_id) {
            return Err(Error::InvalidCertificate(
                CertificateError::ApplicationVerificationFailure,
            ));
        }

        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        verify_tls12_signature(message, cert, dss, &self.supported_schemes)
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        verify_tls13_signature(message, cert, dss, &self.supported_schemes)
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.supported_schemes.supported_schemes()
    }
}

fn verify_cert(
    material: &TlsMaterial,
    supported_schemes: &WebPkiSupportedAlgorithms,
    key_usage: KeyUsage,
    end_entity: &CertificateDer<'_>,
    intermediates: &[CertificateDer<'_>],
    now: UnixTime,
) -> Result<SpiffeId, Error> {
    let peer_id = spiffe_id_from_x509_svid_unchecked(end_entity)
        .map_err(|_| Error::InvalidCertificate(CertificateError::ApplicationVerificationFailure))?;
    let trust_domain = peer_id.trust_domain();

    let trust_anchors = material
        .trust_anchors
        .get(&trust_domain)
        .ok_or(Error::InvalidCertificate(CertificateError::UnknownIssuer))?;

    let crl_refs = material.crls.iter().collect::<Vec<_>>();
    let revocation_options = if crl_refs.is_empty() {
        None
    } else {
        match RevocationOptionsBuilder::new(&crl_refs) {
            Ok(builder) => Some(
                builder
                    .with_status_policy(UnknownStatusPolicy::Allow)
                    .build(),
            ),
            Err(_) => None,
        }
    };

    EndEntityCert::try_from(end_entity)
        .map_err(pki_error)?
        .verify_for_usage(
            supported_schemes.all,
            trust_anchors,
            intermediates,
            now,
            key_usage,
            revocation_options,
            None,
        )
        .map_err(pki_error)?;

    Ok(peer_id)
}

fn verify_tls12_signature(
    message: &[u8],
    cert: &CertificateDer<'_>,
    dss: &DigitallySignedStruct,
    supported_schemes: &WebPkiSupportedAlgorithms,
) -> Result<HandshakeSignatureValid, Error> {
    let possible_algs = signature_algorithms_from_webpki_schemes(supported_schemes, dss.scheme)?;
    let ee = EndEntityCert::try_from(cert).map_err(pki_error)?;

    let mut error = None;
    for alg in possible_algs {
        match ee.verify_signature(*alg, message, dss.signature()) {
            Err(err @ webpki::Error::UnsupportedSignatureAlgorithmForPublicKeyContext(_)) => {
                error = Some(err);
                continue;
            }
            Err(e) => return Err(pki_error(e)),
            Ok(()) => return Ok(HandshakeSignatureValid::assertion()),
        }
    }

    #[allow(deprecated)]
    Err(pki_error(error.unwrap_or(
        webpki::Error::UnsupportedSignatureAlgorithmForPublicKey,
    )))
}

fn verify_tls13_signature(
    message: &[u8],
    cert: &CertificateDer<'_>,
    dss: &DigitallySignedStruct,
    supported_schemes: &WebPkiSupportedAlgorithms,
) -> Result<HandshakeSignatureValid, Error> {
    if !is_scheme_supported_in_tls13(dss.scheme) {
        return Err(PeerMisbehaved::SignedHandshakeWithUnadvertisedSigScheme.into());
    }

    let alg = signature_algorithm_from_webpki_scheme(supported_schemes, dss.scheme)
        .ok_or(PeerMisbehaved::SignedHandshakeWithUnadvertisedSigScheme)?;
    let sig = dss.signature();

    EndEntityCert::try_from(cert)
        .map_err(pki_error)?
        .verify_signature(alg, message, sig)
        .map_err(pki_error)
        .map(|_| HandshakeSignatureValid::assertion())
}

// for TLS 1.2, the signature scheme is a pair of hash and signature algorithms.
// Multiple signature verification algorithms may be supported for a given scheme
fn signature_algorithms_from_webpki_schemes(
    supported_schemes: &WebPkiSupportedAlgorithms,
    dss_scheme: SignatureScheme,
) -> Result<&[&'static dyn SignatureVerificationAlgorithm], Error> {
    supported_schemes
        .mapping
        .iter()
        .find_map(|item| {
            if item.0 == dss_scheme {
                Some(item.1)
            } else {
                None
            }
        })
        .ok_or(Error::PeerMisbehaved(
            PeerMisbehaved::SignedHandshakeWithUnadvertisedSigScheme,
        ))
}

// for TLS 1.3, the signature scheme is a single algorithm.
// So at most one signature verification algorithm may be supported for a given scheme
fn signature_algorithm_from_webpki_scheme(
    supported_schemes: &WebPkiSupportedAlgorithms,
    dss_scheme: SignatureScheme,
) -> Option<&'static dyn SignatureVerificationAlgorithm> {
    signature_algorithms_from_webpki_schemes(supported_schemes, dss_scheme)
        .ok()
        .and_then(|algs| algs.first().copied())
}

fn is_scheme_supported_in_tls13(dss_scheme: SignatureScheme) -> bool {
    let [hash, sign] = dss_scheme.to_array();

    match HashAlgorithm::from(hash) {
        HashAlgorithm::NONE | HashAlgorithm::MD5 | HashAlgorithm::SHA1 | HashAlgorithm::SHA224 => {
            return false;
        }
        _ => (),
    };

    !matches!(
        SignatureAlgorithm::from(sign),
        SignatureAlgorithm::Anonymous | SignatureAlgorithm::RSA | SignatureAlgorithm::DSA
    )
}
