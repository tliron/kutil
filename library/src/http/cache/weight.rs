//
// CacheWeight
//

/// Cache weight.
pub trait CacheWeight {
    /// Cache weight as a byte count.
    ///
    /// It is *not* the amount of memory used, but rather an indicator of *potential* storage
    /// requirements.
    ///
    /// Its intended use is for apples-to-apples comparisons, e.g. to find out which of two items
    /// of the same type weighs more. But even then it may be misleading in some cases, e.g. if
    /// storage involves compression then the "heavier" item might end up taking less storage then
    /// the "lighter" item.
    ///
    /// Note that *sums* of weights can be especially misleading in terms of memory use because
    /// there might be memory shared between items, e.g. via the use of [Bytes](kutil_std::immutable::Bytes).
    fn cache_weight(&self) -> usize;
}
