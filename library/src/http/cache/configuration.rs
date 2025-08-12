use super::hooks::*;

//
// CachingConfiguration
//

/// Caching configuration.
#[derive(Clone)]
pub struct CachingConfiguration {
    /// Minimum body size.
    pub min_body_size: usize,

    /// Maximum body size.
    pub max_body_size: usize,

    /// Cacheable by default.
    pub cacheable_by_default: bool,

    /// Cache duration (hook).
    pub cache_duration: Option<CacheDurationHook>,
}

//
// EncodingConfiguration
//

/// Encoding configuration.
#[derive(Clone, Debug)]
pub struct EncodingConfiguration {
    /// Minimum body size.
    pub min_body_size: usize,

    /// Encodable by default.
    pub encodable_by_default: bool,

    /// Keep identity encoding.
    pub keep_identity_encoding: bool,
}
