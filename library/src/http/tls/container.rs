use super::{
    super::super::std::{collections::*, immutable::*},
    error::*,
    pem::*,
    resolver::*,
};

use {
    rustls::{crypto::*, server::*, sign::*},
    std::sync::*,
};

//
// TlsContainer
//

/// TLS container based on Rustls.
#[derive(Clone, Debug, Default)]
pub struct TlsContainer {
    targets: FastHashMap<ByteString, SniResolverTarget>,
}

impl TlsContainer {
    /// True if empty.
    pub fn is_empty(&self) -> bool {
        self.targets.is_empty()
    }

    /// Add key.
    pub fn add_key(&mut self, sni: ByteString, certified_key: Arc<CertifiedKey>) -> Result<(), TlsContainerError> {
        if self.targets.contains_key(&sni) {
            return Err(format!("already has a target for: {}", sni).into());
        }

        self.targets.insert(sni, SniResolverTarget::Key(certified_key));
        Ok(())
    }

    /// Add delegate.
    pub fn add_delegate(
        &mut self,
        sni: ByteString,
        resolver: Arc<dyn ResolvesServerCert>,
    ) -> Result<(), TlsContainerError> {
        if self.targets.contains_key(&sni) {
            return Err(format!("already has a target for: {}", sni).into());
        }

        self.targets.insert(sni, SniResolverTarget::Delegate(resolver));
        Ok(())
    }

    /// Add key from PEM (Privacy-Enhanced Mail) files.
    pub fn add_key_from_pem(
        &mut self,
        sni: ByteString,
        certificates_pem: &[u8],
        private_key_pem: &[u8],
    ) -> Result<(), TlsContainerError> {
        if self.targets.contains_key(&sni) {
            return Err(format!("already has a target for: {}", sni).into());
        }

        self.targets.insert(
            sni,
            SniResolverTarget::Key(
                certified_key_from_pem(certificates_pem, private_key_pem).map_err(TlsContainerError::new_from)?.into(),
            ),
        );
        Ok(())
    }

    /// Creates a [ServerConfig] for HTTP, specifically for the "h2" and "http/1.1" ALPN
    /// (Application-Layer Protocol Negotiation) protocols.
    ///
    /// Will return an error if we have no keys.
    ///
    /// Otherwise, if we have more than one key then the configuration will use the SNI sent by the
    /// client in its TLS hello message to select the appropriate key in the store.
    pub fn http_server_config(&self) -> Result<ServerConfig, TlsContainerError> {
        let provider = aws_lc_rs::default_provider();

        let mut server_config = ServerConfig::builder_with_provider(provider.into())
            .with_safe_default_protocol_versions()
            .expect("ServerConfig::with_safe_default_protocol_versions")
            .with_no_client_auth()
            .with_cert_resolver(Arc::new(self.resolver()?));

        server_config.alpn_protocols = vec!["h2".into(), "http/1.1".into()];

        Ok(server_config)
    }

    /// Creates a [SniResolver].
    pub fn resolver(&self) -> Result<SniResolver, TlsContainerError> {
        if self.targets.is_empty() {
            Err("no targets".into())
        } else {
            Ok(if self.targets.len() == 1 {
                SniResolver::Single(self.targets.values().next().expect("iter not empty").clone())
            } else {
                SniResolver::BySNI(self.targets.clone())
            })
        }
    }
}
