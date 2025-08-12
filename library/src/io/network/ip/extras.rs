use std::net::*;

//
// IPExtras
//

/// IP address extras.
pub trait IPExtras {
    /// True if the address is reachable.
    fn is_reachable(&self) -> bool;
}

impl IPExtras for IpAddr {
    fn is_reachable(&self) -> bool {
        match self {
            Self::V6(address) => address.is_reachable(),
            Self::V4(address) => address.is_reachable(),
        }
    }
}

impl IPExtras for Ipv6Addr {
    fn is_reachable(&self) -> bool {
        self.extras_is_unicast_global()
    }
}

impl IPExtras for Ipv4Addr {
    fn is_reachable(&self) -> bool {
        self.extras_is_almost_global()
    }
}

//
// IPv6Extras
//

/// IPv6 address extras.
pub trait IPv6Extras {
    /// See [Ipv6Addr::is_unicast_global] (unstable).
    fn extras_is_unicast_global(&self) -> bool;

    /// See [Ipv6Addr::is_unicast] (unstable).
    fn extras_is_unicast(&self) -> bool;

    /// See [Ipv6Addr::is_documentation] (unstable).
    fn extras_is_documentation(&self) -> bool;

    /// See [Ipv6Addr::is_benchmarking] (unstable).
    fn extras_is_benchmarking(&self) -> bool;
}

// Copied from unstable

impl IPv6Extras for Ipv6Addr {
    fn extras_is_unicast_global(&self) -> bool {
        self.extras_is_unicast()
            && !self.is_loopback()
            && !self.is_unicast_link_local()
            && !self.is_unique_local()
            && !self.is_unspecified()
            && !self.extras_is_documentation()
            && !self.extras_is_benchmarking()
    }

    fn extras_is_unicast(&self) -> bool {
        !self.is_multicast()
    }

    fn extras_is_documentation(&self) -> bool {
        matches!(self.segments(), [0x2001, 0xdb8, ..] | [0x3fff, 0..=0x0fff, ..])
    }

    fn extras_is_benchmarking(&self) -> bool {
        (self.segments()[0] == 0x2001) && (self.segments()[1] == 0x2) && (self.segments()[2] == 0)
    }
}

//
// IPv4Extras
//

/// IPv4 address extras.
pub trait IPv4Extras {
    /// See [Ipv4Addr::is_global] (unstable).
    fn extras_is_almost_global(&self) -> bool;

    /// See [Ipv4Addr::is_shared] (unstable).
    fn extras_is_shared(&self) -> bool;

    /// See [Ipv4Addr::is_benchmarking] (unstable).
    fn extras_is_benchmarking(&self) -> bool;

    /// See [Ipv4Addr::is_reserved] (unstable).
    fn extras_is_reserved(&self) -> bool;
}

// (Mostl) copied from unstable

impl IPv4Extras for Ipv4Addr {
    fn extras_is_almost_global(&self) -> bool {
        !(self.octets()[0] == 0
            //|| self.is_private()
            || self.extras_is_shared()
            || self.is_loopback()
            || self.is_link_local()
            || (self.octets()[0] == 192
                && self.octets()[1] == 0
                && self.octets()[2] == 0
                && self.octets()[3] != 9
                && self.octets()[3] != 10)
            || self.is_documentation()
            || self.extras_is_benchmarking()
            || self.extras_is_reserved()
            || self.is_broadcast())
    }

    fn extras_is_shared(&self) -> bool {
        self.octets()[0] == 100 && (self.octets()[1] & 0b1100_0000 == 0b0100_0000)
    }

    fn extras_is_benchmarking(&self) -> bool {
        self.octets()[0] == 198 && (self.octets()[1] & 0xfe) == 18
    }

    fn extras_is_reserved(&self) -> bool {
        self.octets()[0] & 240 == 240 && !self.is_broadcast()
    }
}
