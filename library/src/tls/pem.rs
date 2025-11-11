use {
    rustls::{crypto::aws_lc_rs::sign::*, sign::*},
    rustls_pki_types::*,
    std::{io, sync::*},
};

/// Creates a [CertifiedKey] by parsing PEM (Privacy-Enhanced Mail) files as X.509 certificates.
pub fn certified_key_from_pem(certificates_pem: &[u8], private_key_pem: &[u8]) -> io::Result<CertifiedKey> {
    let certificates = parse_certificates_pem(certificates_pem)?;
    let signing_key = get_signing_key_from_pem(private_key_pem)?;
    Ok(CertifiedKey { cert: certificates, key: signing_key, ocsp: None })
}

/// Parses a PEM (Privacy-Enhanced Mail) file as X.509 certificates.
///
/// Returns them in DER (Distinguished Encoding Rules) format.
pub fn parse_certificates_pem(pem: &[u8]) -> io::Result<Vec<CertificateDer<'static>>> {
    let mut certificates = Vec::default();
    for certificate in rustls_pemfile::certs(&mut pem.as_ref()) {
        certificates.push(certificate?);
    }
    Ok(certificates)
}

/// Parses a PEM (Privacy-Enhanced Mail) file as a X.509 private key.
///
/// Only parses the first entry in the PEM, ignoring the rest.
///
/// Returns it in DER (Distinguished Encoding Rules) format.
pub fn parse_private_key_pem(pem: &[u8]) -> io::Result<PrivateKeyDer<'static>> {
    // Note: axum_server seems to not know about rustls_pemfile::private_key
    // https://github.com/programatik29/axum-server/blob/f60b6150bba0aecf4910f877fd0bc12ac24d030b/src/tls_rustls/mod.rs#L302

    match rustls_pemfile::private_key(&mut pem.as_ref())? {
        Some(private_key) => Ok(private_key),
        None => Err(io::Error::other("no private key in PEM")),
    }
}

/// Get a signing key from a PEM (Privacy-Enhanced Mail) file.
///
/// See [parse_private_key_pem].
pub fn get_signing_key_from_pem(pem: &[u8]) -> io::Result<Arc<dyn SigningKey>> {
    any_supported_type(&parse_private_key_pem(pem)?).map_err(io::Error::other)
}
