use super::r#trait::*;

use {
    rustls::{client::*, *},
    rustls_pki_types::*,
    webpki_roots::*,
};

// Root certificates (from Mozilla's webpki-roots) will be contained in our binary as static data
// Advantage: will work reliably and deterministically in any environment
// Disadvantages: will not use operating system's configured certificates; will become outdated over time

impl WithStandardVerifier for ConfigBuilder<ClientConfig, WantsVerifier> {
    fn with_standard_verifier(
        self,
        root_certificates: Option<Vec<CertificateDer<'static>>>,
    ) -> Result<ConfigBuilder<ClientConfig, WantsClientCert>, Error> {
        let mut store = RootCertStore { roots: TLS_SERVER_ROOTS.into() };

        if let Some(root_certificates) = root_certificates {
            for certificate in root_certificates {
                store.add(certificate)?;
            }
        }

        Ok(self.with_root_certificates(store))
    }
}
