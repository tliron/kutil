use std::net::*;

//
// IPStack
//

/// IP stack.
#[derive(Clone, Debug, Default)]
pub enum IPStack {
    /// IPv6 only.
    IPv6,

    /// IPv4 only.
    IPv4,

    /// IPv6 and IPv4.
    #[default]
    Dual,
}

impl IPStack {
    /// Whether the stack allows IPv6.
    pub fn allows_ipv6(&self) -> bool {
        matches!(self, Self::IPv6 | Self::Dual)
    }

    /// Whether the stack allows IPv4.
    pub fn allows_ipv4(&self) -> bool {
        matches!(self, Self::IPv4 | Self::Dual)
    }

    /// Whether the stack allows the address.
    pub fn allows(&self, address: &IpAddr) -> bool {
        match self {
            Self::IPv6 => address.is_ipv6(),
            Self::IPv4 => address.is_ipv4(),
            Self::Dual => true,
        }
    }
}
