use super::{acme::*, container::*, error::*};

use axum_server::tls_rustls::*;

impl TlsContainer {
    /// Creates an axum [RustlsAcceptor].
    pub fn axum_acceptor(&self) -> Result<RustlsAcceptor, TlsContainerError> {
        Ok(RustlsAcceptor::new(self.axum_config()?))
    }

    /// Creates an [AxumAcceptor](rustls_acme::axum::AxumAcceptor).
    #[cfg(feature = "acme")]
    pub fn axum_acme_acceptor(&self, acme: ACME) -> Result<rustls_acme::axum::AxumAcceptor, TlsContainerError> {
        // TODO
        let state = acme.into_config().state();
        let acceptor = state.axum_acceptor(self.http_server_config()?.into());
        Ok(acceptor)
    }

    /// Creates an axum [RustlsConfig].
    pub fn axum_config(&self) -> Result<RustlsConfig, TlsContainerError> {
        Ok(RustlsConfig::from_config(self.http_server_config()?.into()))
    }
}
