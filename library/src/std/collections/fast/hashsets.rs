use super::build_hasher::*;

use std::collections::*;

/// Fast [HashSet].
///
/// Note that the implementation relies on a
/// [non-cryptographic hash function](https://en.wikipedia.org/wiki/Non-cryptographic_hash_function).
pub type FastHashSet<ValueT> = HashSet<ValueT, FastBuildHasher>;

/// Fast concurrent hash set.
///
/// Note that the implementation relies on a
/// [non-cryptographic hash function](https://en.wikipedia.org/wiki/Non-cryptographic_hash_function).
pub type FastConcurrentHashSet<ValueT> = papaya::HashSet<ValueT, FastBuildHasher>;

pub use ahash::HashSetExt;
