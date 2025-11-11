use super::r#trait::*;

use {
    rustls::{client::*, *},
    rustls_pki_types::*,
    rustls_platform_verifier::*,
    std::sync::*,
};

impl WithStandardVerifier for ConfigBuilder<ClientConfig, WantsVerifier> {
    fn with_standard_verifier(
        self,
        root_certificates: Option<Vec<CertificateDer<'static>>>,
    ) -> Result<ConfigBuilder<ClientConfig, WantsClientCert>, Error> {
        match root_certificates {
            Some(root_certificates) => {
                let crypto_provider = self.crypto_provider().clone();
                let verifier = Verifier::new_with_extra_roots(root_certificates, crypto_provider)?;
                Ok(self.dangerous().with_custom_certificate_verifier(Arc::new(verifier)))
            }

            None => self.with_platform_verifier(),
        }
    }
}
