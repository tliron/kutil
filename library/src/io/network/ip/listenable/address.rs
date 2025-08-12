use std::{io, net::*};

//
// ListenableAddress
//

/// Listenable [IpAddr].
#[derive(Clone, Debug)]
pub struct ListenableAddress {
    /// Address.
    pub address: IpAddr,

    /// Optional zone for IPv6 address.
    pub zone: Option<String>,

    /// Optional flowinfo for IPv6 address.
    pub flowinfo: Option<u32>,
}

impl ListenableAddress {
    /// Unspecified IPv6.
    pub const UNSPECIFIED_IPV6: ListenableAddress =
        ListenableAddress::new_with(IpAddr::V6(Ipv6Addr::UNSPECIFIED), None, None);

    /// Unspecified IPv4.
    pub const UNSPECIFIED_IPV4: ListenableAddress =
        ListenableAddress::new_with(IpAddr::V4(Ipv4Addr::UNSPECIFIED), None, None);

    /// Unspecified dual.
    pub const UNSPECIFIED_DUAL: &[ListenableAddress] = &[Self::UNSPECIFIED_IPV6, Self::UNSPECIFIED_IPV4];

    /// Constructor.
    pub const fn new(address: IpAddr) -> Self {
        Self { address, zone: None, flowinfo: None }
    }

    /// Constructor.
    pub const fn new_with(address: IpAddr, zone: Option<String>, flowinfo: Option<u32>) -> Self {
        Self { address, zone, flowinfo }
    }

    /// To [SocketAddr].
    pub fn to_socket_address(&self, port: u16) -> io::Result<SocketAddr> {
        Ok(match self.address {
            IpAddr::V6(address) => {
                // See: https://github.com/rust-lang/libs-team/issues/476#issuecomment-2825453898

                match &self.zone {
                    Some(zone) => {
                        // We can only set the zone via `to_socket_addrs`
                        (address.to_string() + "%" + zone, port)
                            .to_socket_addrs()?
                            .next()
                            .ok_or(io::Error::other("no socket address found"))?
                    }

                    None => SocketAddrV6::new(address, port, self.flowinfo.unwrap_or_default(), 0).into(),
                }
            }

            IpAddr::V4(address) => SocketAddrV4::new(address, port).into(),
        })
    }
}
