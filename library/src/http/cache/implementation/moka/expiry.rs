use super::super::super::{key::*, response::*};

use {duration_str::*, moka::*, std::time::*};

//
// CachedResponseExpiry
//

/// Moka [Expiry] for [CachedResponse].
pub struct CachedResponseExpiry;

impl<CacheKeyT> Expiry<CacheKeyT, CachedResponseRef> for CachedResponseExpiry
where
    CacheKeyT: CacheKey,
{
    fn expire_after_create(
        &self,
        _cache_key: &CacheKeyT,
        cached_response: &CachedResponseRef,
        _created_at: Instant,
    ) -> Option<Duration> {
        if let Some(duration) = cached_response.duration {
            tracing::debug!("storing with duration: {}", duration.human_format());
        }

        cached_response.duration
    }
}
