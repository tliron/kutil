use super::super::super::{key::*, response::*, weight::*};

/// Moka cache entry weigher.
pub fn weigher<CacheKeyT>(cache_key: &CacheKeyT, cached_response: &CachedResponseRef) -> u32
where
    CacheKeyT: CacheKey,
{
    let weight = cache_key.cache_weight() + cached_response.cache_weight();
    let weight = weight.try_into().unwrap_or(u32::MAX);
    tracing::debug!("{} for {}", weight, cache_key);
    weight
}
