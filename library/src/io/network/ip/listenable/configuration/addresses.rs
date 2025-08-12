use super::super::{super::stack::*, address::*};

use std::{io, net::*};

//
// ListenableAddressesConfiguration
//

/// Configuration for a set of listenable [IpAddr].
#[derive(Clone, Debug)]
pub struct ListenableAddressesConfiguration {
    /// Optional address or hint.
    pub hint: Option<IpAddr>,

    /// Optional zone for explicit IPv6 address.
    pub zone: Option<String>,

    /// Optional flowinfo for explicit IPv6 address.
    pub flowinfo: Option<u32>,

    /// Whether to allow unspecified addresses to be provided.
    pub allow_unspecified: bool,

    /// Whether to include loopbacks when providing reachable addresses.
    pub include_loopbacks: bool,
}

impl ListenableAddressesConfiguration {
    /// Constructor.
    pub fn new(
        hint: Option<IpAddr>,
        zone: Option<String>,
        flowinfo: Option<u32>,
        allow_unspecified: bool,
        include_loopbacks: bool,
    ) -> Self {
        Self { hint, zone, flowinfo, allow_unspecified, include_loopbacks }
    }

    /// Provides zero or more [IpAddr] on which to listen.
    ///
    /// If `allow_unspecified` is true:
    ///
    /// * If the hint is [None] we'll provide the two unspecified addresses for both IP versions,
    ///   "::" (IPv6) and "0.0.0.0" (IPv4).
    /// * Otherwise we will use the hint as is. If it's IPv6 it will include `zone` and `flowinfo`.
    ///
    /// If `allow_unspecified` is false, we'll *only* provided specified addresses (never "::" or
    /// "0.0.0.0"):
    ///
    /// * If the hint is [None] we'll provide reachable addresses (both IPv6 and IPv4).
    /// * If the hint is unspecified IPv6 ("::") we'll provide reachable IPv6 addresses.
    /// * If the hint is unspecified IPv4 ("0.0.0.0") we'll provide reachable IPv4 addresses.
    /// * Otherwise the hint must be a specified address so we will use it as is. If it's IPv6 it
    ///   will include `zone` and `flowinfo`.
    pub fn addresses(&self) -> io::Result<Vec<ListenableAddress>> {
        match self.hint {
            Some(address) => {
                if self.allow_unspecified || !address.is_unspecified() {
                    Ok(vec![ListenableAddress::new_with(address, self.zone.clone(), self.flowinfo)])
                } else {
                    match address {
                        IpAddr::V6(_) => IPStack::IPv6.reachable_addresses(self.include_loopbacks),
                        IpAddr::V4(_) => IPStack::IPv4.reachable_addresses(self.include_loopbacks),
                    }
                }
            }

            None => {
                if self.allow_unspecified {
                    Ok(ListenableAddress::UNSPECIFIED_DUAL.into())
                } else {
                    IPStack::Dual.reachable_addresses(self.include_loopbacks)
                }
            }
        }
    }
}
