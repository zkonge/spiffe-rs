use core::str;

use rustls_pki_types::CertificateDer;
use spiffe_id::SpiffeId;

use super::{InvalidDerDataError, SpiffeError};

type Tlv<'a> = (u8, &'a [u8]);

const fn read_der_tlv(der: &[u8]) -> Option<(&[u8], Tlv<'_>)> {
    let [tag, first_len_byte, rem @ ..] = der else {
        return None;
    };

    if *first_len_byte & 0x80 == 0 {
        let Some((value, rem)) = rem.split_at_checked(*first_len_byte as usize) else {
            return None;
        };
        return Some((rem, (*tag, value)));
    }

    let len_len = *first_len_byte & 0x7f;
    let Some((len_bytes, rem)) = rem.split_at_checked(len_len as usize) else {
        return None;
    };

    let len: usize = match len_bytes {
        [a] => u32::from_le_bytes([*a, 0, 0, 0]) as _,
        [a, b] => u32::from_le_bytes([*b, *a, 0, 0]) as _,
        [a, b, c] => u32::from_le_bytes([*c, *b, *a, 0]) as _,
        // Is it possible to have a 16 MiB+ X.509 certificate in real world?
        _ => return None,
    };

    let Some((value, rem)) = rem.split_at_checked(len) else {
        return None;
    };

    Some((rem, (*tag, value)))
}

fn split_cert(raw: &[u8]) -> Option<(&[u8], CertificateDer<'_>)> {
    let (rem, (0x30, _)) = read_der_tlv(raw)? else {
        return None;
    };

    let cert = raw.get(..raw.len() - rem.len())?;

    Some((rem, CertificateDer::from_slice(cert)))
}

#[derive(Clone, Debug)]
pub struct CertificateIter<'a> {
    der: &'a [u8],
}

impl<'a> Iterator for CertificateIter<'a> {
    type Item = Result<CertificateDer<'a>, InvalidDerDataError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.der.is_empty() {
            return None;
        }

        let (rem, cert) = match split_cert(self.der) {
            Some((r, c)) => (r, Ok(c)),
            None => ([].as_slice(), Err(InvalidDerDataError)),
        };
        self.der = rem;

        Some(cert)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut count = 0;
        let mut rem = self.der;

        while !rem.is_empty() {
            if let Some((r, _)) = split_cert(rem) {
                count += 1;
                rem = r;
            } else {
                break;
            }
        }

        (count, Some(count))
    }
}

/// SPIFFE workload standard uses a very ugly way to represent multiple DER-encoded X.509 certificates:
/// they are concatenated together without any separator. This function splits them into individual
pub fn split_certificates(der: &[u8]) -> CertificateIter<'_> {
    CertificateIter { der }
}

const SAN_OID_ASN1_BYTES: [u8; 5] = [0x06, 0x03, 0x55, 0x1D, 0x11];

/// Extracts SPIFFE ID from a trusted X.509 SVID
pub fn spiffe_id_from_x509_svid(cert: &CertificateDer) -> Result<SpiffeId, SpiffeError> {
    const INVALID_DER: SpiffeError = SpiffeError::InvalidDerData(InvalidDerDataError);

    // unpack the `Certificate`, ensure only one certificate is present
    let Some(([], (0x30, cert))) = read_der_tlv(cert.as_ref()) else {
        return Err(INVALID_DER);
    };

    // unpack the `TBSCertificate`
    let Some((_, (0x30, tbs_certificate))) = read_der_tlv(cert) else {
        return Err(INVALID_DER);
    };
    // begin to process `TBSCertificate` value
    let mut rem = tbs_certificate;

    // find the `extensions` field
    // RFC5280: extensions [3] Extensions OPTIONAL
    let extensions = loop {
        if rem.is_empty() {
            // No `extensions` field found
            return Err(INVALID_DER);
        }

        match read_der_tlv(rem) {
            Some((_, (0xa3, maybe_ext))) => break maybe_ext,
            Some((r, (_, _))) => rem = r,
            None => return Err(INVALID_DER),
        }
    };

    // unpack the `Extensions`
    // `Extensions` should be the last field in `TBSCertificate`
    let Some(([], (0x30, extensions))) = read_der_tlv(extensions) else {
        return Err(INVALID_DER);
    };
    // begin to process `Extensions` value
    let mut rem = extensions;

    // find the `subjectAltName` extension
    let san = loop {
        if rem.is_empty() {
            // No SAN found
            return Err(INVALID_DER);
        }

        match read_der_tlv(rem) {
            Some((_, (0x30, maybe_san))) if maybe_san.starts_with(&SAN_OID_ASN1_BYTES) => {
                break maybe_san;
            }
            Some((r, (_, _))) => rem = r,
            None => return Err(INVALID_DER),
        }
    };
    // begin to process `subjectAltName` extension value
    let rem = san;

    // skip extnID
    let Some((rem, (_, _))) = read_der_tlv(rem) else {
        return Err(INVALID_DER);
    };

    // skip criticality
    // begin to process `extnValue`
    let extn_value = if let Some((rem_no_crit, (0x01, _))) = read_der_tlv(rem) {
        // criticality is present and skipped
        rem_no_crit
    } else {
        // criticality is not present
        rem
    };

    // unpack the `extnValue`
    let Some((_, (0x04, san_oct_string_value))) = read_der_tlv(extn_value) else {
        return Err(INVALID_DER);
    };

    // unpack the `OCTET STRING` value, it should contains a SEQUENCE
    let Some((_, (0x30, san_values))) = read_der_tlv(san_oct_string_value) else {
        return Err(INVALID_DER);
    };
    // begin to process `SAN` values
    let mut rem = san_values;

    // find the URI SAN
    let mut final_uri_san: Option<&[u8]> = None;

    while !rem.is_empty() {
        rem = match read_der_tlv(rem) {
            Some((r, (0x86, uri_san))) => match final_uri_san {
                // https://github.com/spiffe/spiffe/blob/main/standards/X509-SVID.md#52-leaf-validation
                // "SVIDs containing more than one URI SAN MUST be rejected."
                Some(_) => return Err(INVALID_DER),
                None => {
                    final_uri_san = Some(uri_san);
                    r
                }
            },
            Some((r, (_, _))) => r,
            None => break,
        };
    }

    let uri_san = final_uri_san.ok_or(INVALID_DER)?;
    let uri_str = str::from_utf8(uri_san).map_err(|_| INVALID_DER)?;

    SpiffeId::new(uri_str).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use const_decoder::{Pem, decode};

    use super::*;

    const CERT: &[u8] = &decode!(
        Pem,
        b"-----BEGIN CERTIFICATE-----
MIICPTCCAeKgAwIBAgIRAN/j0z/qhstB4YUG05bFODowCgYIKoZIzj0EAwIwUDEL
MAkGA1UEBhMCVVMxDzANBgNVBAoTBlNQSUZGRTEwMC4GA1UEBRMnMzE1OTkyNjkz
MjA3Mjk3NzgxNDM2MzgzNDM0ODE1NzAwOTg0MDY0MB4XDTI0MTEwNDEwNDAxNloX
DTI0MTEwNjEwNDAyNlowNTELMAkGA1UEBhMCVVMxDjAMBgNVBAoTBVNQSVJFMRYw
FAYDVQQDEw10ZXN0LmtvbmdlLnB3MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE
1Gk6PBAXw2o+yb/uDKvsSTJhjJCK6uOSdSQc/JrrOR6t9T22yhzmgZlYKVMR1Fja
OI17RtpUDnktHPlqMQdL36OBtzCBtDAOBgNVHQ8BAf8EBAMCA6gwHQYDVR0lBBYw
FAYIKwYBBQUHAwEGCCsGAQUFBwMCMAwGA1UdEwEB/wQCMAAwHQYDVR0OBBYEFJeC
FQUJ8f02uWhlPIfVoPBRiiyTMB8GA1UdIwQYMBaAFApqNiTE4a4P7SYPUy+VQU2c
mfY/MDUGA1UdEQQuMCyCDXRlc3Qua29uZ2UucHeGG3NwaWZmZTovL2V4YW1wbGUu
b3JnL3prb25nZTAKBggqhkjOPQQDAgNJADBGAiEApYklAyReuj1UbAbJghpeXylZ
X+dAAYszyO2TWG8AvD0CIQC7Nj63pq6JpBZeag/+ZWazJf1N1Ah6fqo/Py6nj7uW
SQ==
-----END CERTIFICATE-----"
    );

    #[test]
    fn test_spiffe_id_from_x509_svid() {
        assert_eq!(
            spiffe_id_from_x509_svid(&CertificateDer::from_slice(CERT)).unwrap(),
            SpiffeId::new("spiffe://example.org/zkonge").unwrap()
        );
    }
}
