use super::{cache::*, key::*, response::*};

//
// TieredCache
//

/// Two-tiered [Cache].
///
/// The assumption is that the first cache is faster than the next.
///
/// For more tiers you can chain this type.
#[derive(Clone, Debug)]
pub struct TieredCache<FirstCacheT, NextCacheT> {
    /// First cache.
    pub first: FirstCacheT,

    /// Next cache.
    pub next: NextCacheT,
}

impl<FirstCacheT, NextCacheT> TieredCache<FirstCacheT, NextCacheT> {
    /// Constructor.
    pub fn new(first: FirstCacheT, next: NextCacheT) -> Self {
        Self { first, next }
    }
}

impl<CacheKeyT, FirstCacheT, NextCacheT> Cache<CacheKeyT> for TieredCache<FirstCacheT, NextCacheT>
where
    CacheKeyT: CacheKey,
    FirstCacheT: Cache<CacheKeyT>,
    NextCacheT: Cache<CacheKeyT>,
{
    async fn get(&self, key: &CacheKeyT) -> Option<CachedResponseRef> {
        match self.first.get(key).await {
            Some(cached_response) => Some(cached_response),
            None => self.next.get(key).await,
        }
    }

    async fn put(&self, key: CacheKeyT, cached_response: CachedResponseRef) {
        self.first.put(key.clone(), cached_response.clone()).await;
        self.next.put(key, cached_response).await
    }

    async fn invalidate(&self, key: &CacheKeyT) {
        self.first.invalidate(key).await;
        self.next.invalidate(key).await
    }

    async fn invalidate_all(&self) {
        self.first.invalidate_all().await;
        self.next.invalidate_all().await
    }
}
