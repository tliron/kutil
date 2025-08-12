use super::{
    super::{extras::*, stack::*},
    address::*,
};

use std::io;

impl IPStack {
    /// Get all reachable addresses.
    ///
    /// Reachable addresses are always *specified*. As such they provide an explicit alternative to
    /// using unspecified addresses (e.g. "::" for IPv6 and "0.0.0.0" for IP4) for binding
    /// functions.
    ///
    /// This function works by enumerating all the interfaces and their addresses and checking each
    /// address for reachability.
    ///
    /// The result could be an empty vector if no reachable addresses are found. Also, there if no
    /// guarantee for order. In particular, IPv6 and IPv4 are not grouped separately for
    /// [Dual](Self::Dual).
    pub fn reachable_addresses(&self, include_loopbacks: bool) -> io::Result<Vec<ListenableAddress>> {
        let mut reachable_addresses = Vec::default();

        for interface in netdev::get_interfaces() {
            if interface.is_up() && interface.is_running() {
                if self.allows_ipv6() {
                    for address in &interface.ipv6 {
                        let address = address.addr();
                        if (include_loopbacks && address.is_loopback()) || address.is_reachable() {
                            reachable_addresses.push(ListenableAddress::new_with(address.into(), None, None));
                        }
                    }
                }

                if self.allows_ipv4() {
                    for address in &interface.ipv4 {
                        let address = address.addr();
                        if (include_loopbacks && address.is_loopback()) || address.is_reachable() {
                            reachable_addresses.push(ListenableAddress::new(address.into()));
                        }
                    }
                }
            }
        }

        Ok(reachable_addresses)
    }
}
