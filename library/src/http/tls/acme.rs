use super::{super::super::std::immutable::*, container::*, error::*};

use {
    rustls_acme::{caches::*, *},
    std::{io, path::*},
};

// https://docs.rs/instant-acme/latest/instant_acme/
// https://docs.rs/rustls-acme/latest/rustls_acme/
// cli only? https://github.com/breard-r/acmed

pub use acme::{LETS_ENCRYPT_PRODUCTION_DIRECTORY, LETS_ENCRYPT_STAGING_DIRECTORY};

impl TlsContainer {
    /// Add [ResolvesServerCertAcme] for all hosts.
    pub fn add_resolver_from_acme(&mut self, acme: ACME) -> Result<(), TlsContainerError> {
        let hosts = acme.hosts.clone();
        let state = acme.into_config().state();
        let resolver = state.resolver();
        for host in hosts {
            self.add_delegate(host.clone(), resolver.clone())?;
        }
        Ok(())
    }
}

//
// ACME
//

/// ACME.
#[derive(Debug, Default)]
pub struct ACME {
    /// Hosts.
    pub hosts: Vec<ByteString>,

    /// Directory URL.
    pub directory: ByteString,

    /// Contacts (usually email addresses).
    pub contacts: Vec<ByteString>,

    /// Cache path.
    pub cache: PathBuf,
}

impl ACME {
    /// Into [AcmeConfig].
    pub fn into_config(self) -> AcmeConfig<io::Error> {
        let mut acme_config = AcmeConfig::new(self.hosts).directory(self.directory).cache(DirCache::new(self.cache));
        for contact in self.contacts {
            acme_config = acme_config.contact_push(String::from("mailto:") + &contact);
        }
        acme_config
    }
}
