
use rustls_pki_types::CertificateDer;

use super::InvalidDerDataError;

type Tlv<'a> = (u8, usize, &'a [u8]);

fn read_der_tlv(der: &[u8]) -> Option<(&[u8], Tlv)> {
    let [tag, first_len_byte, rem @ ..] = der else {
        return None;
    };

    if *first_len_byte & 0x80 == 0 {
        let (value, rem) = rem.split_at_checked(*first_len_byte as usize)?;
        return Some((rem, (*tag, *first_len_byte as usize, value)));
    }

    let len_len = *first_len_byte & 0x7f;
    let (len_bytes, rem) = rem.split_at_checked(len_len as usize)?;

    let len: usize = match len_bytes {
        [a] => u32::from_le_bytes([*a, 0, 0, 0]) as _,
        [a, b] => u32::from_le_bytes([*b, *a, 0, 0]) as _,
        [a, b, c] => u32::from_le_bytes([*c, *b, *a, 0]) as _,
        // Is it possible to have a 16 MiB+ X.509 certificate in real world?
        _ => return None,
    };

    let (value, rem) = rem.split_at_checked(len)?;

    Some((rem, (*tag, len, value)))
}

fn split_cert(raw: &[u8]) -> Option<(&[u8], CertificateDer<'_>)> {
    let (rem, (0x30, _, _)) = read_der_tlv(raw)? else {
        return None;
    };

    let cert = raw.get(..raw.len() - rem.len())?;

    Some((rem, CertificateDer::from_slice(cert)))
}

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
        (if self.der.is_empty() { 0 } else { 1 }, None)
    }
}

/// SPIFFE workload standard uses a very ugly way to represent multiple DER-encoded X.509 certificates:
/// they are concatenated together without any separator. This function splits them into individual
pub fn split_certificates(der: &[u8]) -> CertificateIter<'_> {
    CertificateIter { der }
}
