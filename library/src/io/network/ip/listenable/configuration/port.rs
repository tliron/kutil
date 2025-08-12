use super::{super::address::*, addresses::*};

use std::{io, net::*, vec};

//
// ListenablePortConfiguration
//

/// Configuration for a listenable TCP or UDP port.
///
/// Implements [ToSocketAddrs], which provides zero or more [SocketAddr] on which to listen.
#[derive(Clone, Debug)]
pub struct ListenablePortConfiguration {
    /// Port.
    pub port: u16,

    /// Optional address or hint.
    pub address_hint: Option<IpAddr>,

    /// Optional zone for IPv6 address.
    pub zone: Option<String>,

    /// Optional flowinfo for IPv6 address.
    pub flowinfo: Option<u32>,

    /// Whether to allow unspecified addresses for [ToSocketAddrs].
    pub allow_unspecified: bool,

    /// Whether to include loopbacks when providing reachable addresses for [ToSocketAddrs].
    pub include_loopbacks: bool,
}

impl ListenablePortConfiguration {
    /// Constructor.
    pub fn new(
        port: u16,
        address_hint: Option<IpAddr>,
        zone: Option<String>,
        flowinfo: Option<u32>,
        allow_unspecified: bool,
        include_loopbacks: bool,
    ) -> Self {
        Self { port, address_hint, zone, flowinfo, allow_unspecified, include_loopbacks }
    }

    /// Addresses.
    ///
    /// See [ListenableAddressesConfiguration].
    pub fn addresses(&self) -> io::Result<Vec<ListenableAddress>> {
        ListenableAddressesConfiguration::new(
            self.address_hint,
            self.zone.clone(),
            self.flowinfo,
            self.allow_unspecified,
            self.include_loopbacks,
        )
        .addresses()
    }
}

impl ToSocketAddrs for ListenablePortConfiguration {
    type Iter = vec::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        let addresses = self.addresses()?;
        let mut socket_addresses = Vec::with_capacity(addresses.len());
        for address in addresses {
            socket_addresses.push(address.to_socket_address(self.port)?);
        }
        Ok(socket_addresses.into_iter())
    }
}
