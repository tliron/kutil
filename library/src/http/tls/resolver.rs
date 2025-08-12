use super::super::super::std::{collections::*, immutable::*};

use {
    rustls::{server::*, sign::*},
    std::sync::*,
};

//
// SniResolver
//

/// [ResolvesServerCert] that can select the target by client SNI (Server Name Indication).
///
/// Also has an optimized mode for when there is only one target, which skips checking the SNI.
///
/// See [ResolvesServerCertUsingSni] for a simpler version.
#[derive(Clone, Debug)]
pub enum SniResolver {
    /// Select [SniResolverTarget] by SNI.
    BySNI(FastHashMap<ByteString, SniResolverTarget>),

    /// Single [SniResolverTarget].
    Single(SniResolverTarget),
}

impl ResolvesServerCert for SniResolver {
    fn resolve(&self, client_hello: ClientHello<'_>) -> Option<Arc<CertifiedKey>> {
        match self {
            Self::BySNI(targets) => match client_hello.server_name() {
                Some(sni) => match targets.get(sni) {
                    Some(target) => {
                        tracing::trace!("SNI: {}", sni);
                        target.resolve(client_hello)
                    }

                    None => {
                        tracing::warn!("unknown SNI: {}", sni);
                        None
                    }
                },

                // Client did not provide an SNI
                None => {
                    tracing::trace!("no SNI");
                    None
                }
            },

            Self::Single(target) => {
                tracing::trace!("single");
                target.resolve(client_hello)
            }
        }
    }
}

//
// SniResolverTarget
//

/// Target for [SniResolver].
#[derive(Clone, Debug)]
pub enum SniResolverTarget {
    /// Key.
    Key(Arc<CertifiedKey>),

    /// Delegate.
    Delegate(Arc<dyn ResolvesServerCert>),
}

impl ResolvesServerCert for SniResolverTarget {
    fn resolve(&self, client_hello: ClientHello<'_>) -> Option<Arc<rustls::sign::CertifiedKey>> {
        match self {
            Self::Key(certified_key) => {
                tracing::trace!("key");
                Some(certified_key.clone())
            }

            Self::Delegate(delegate) => {
                tracing::trace!("delegate");
                delegate.resolve(client_hello)
            }
        }
    }
}
