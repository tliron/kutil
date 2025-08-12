use super::{
    super::super::{key::*, response::*},
    expiry::*,
    weigher::*,
};

//
// ForHttpResponse
//

/// Add support for [CachedResponse] weigher and [Expiry](moka::Expiry).
pub trait ForHttpResponse
where
    Self: Sized,
{
    /// Add support for [CachedResponse] weigher and [Expiry](moka::Expiry).
    fn for_http_response(self) -> Self;
}

impl<CacheKeyT> ForHttpResponse
    for moka::future::CacheBuilder<CacheKeyT, CachedResponseRef, moka::future::Cache<CacheKeyT, CachedResponseRef>>
where
    CacheKeyT: CacheKey,
{
    fn for_http_response(self) -> Self {
        self.weigher(weigher).expire_after(CachedResponseExpiry)
    }
}
