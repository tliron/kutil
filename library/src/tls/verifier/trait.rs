use {
    rustls::{client::*, *},
    rustls_pki_types::*,
};

//
// WithStandardVerifier
//

/// Use the standard certificate verifier.
pub trait WithStandardVerifier {
    /// Use the standard certificate verifier. Optionally add extra root certificates.
    fn with_standard_verifier(
        self,
        root_certificates: Option<Vec<CertificateDer<'static>>>,
    ) -> Result<ConfigBuilder<ClientConfig, WantsClientCert>, Error>;
}
