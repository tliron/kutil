use {
    http::*,
    std::{sync::*, time::*},
};

/// Hook to get a response's cache duration.
pub type CacheDurationHook = Arc<Box<dyn Fn(CacheDurationHookContext) -> Option<Duration> + Send + Sync>>;

//
// CacheDurationHookContext
//

/// Context for [CacheDurationHook].
pub struct CacheDurationHookContext<'own> {
    /// URI.
    pub uri: &'own Uri,

    /// Headers.
    pub headers: &'own HeaderMap,
}

impl<'own> CacheDurationHookContext<'own> {
    /// Constructor.
    pub fn new(uri: &'own Uri, headers: &'own HeaderMap) -> Self {
        Self { uri, headers }
    }
}
