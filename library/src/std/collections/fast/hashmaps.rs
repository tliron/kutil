use super::build_hasher::*;

use std::collections::*;

/// Fast [HashMap].
///
/// Note that the implementation relies on a
/// [non-cryptographic hash function](https://en.wikipedia.org/wiki/Non-cryptographic_hash_function).
pub type FastHashMap<KeyT, ValueT> = HashMap<KeyT, ValueT, FastBuildHasher>;

/// Fast concurrent hash map.
///
/// Note that the implementation relies on a
/// [non-cryptographic hash function](https://en.wikipedia.org/wiki/Non-cryptographic_hash_function).
pub type FastConcurrentHashMap<KeyT, ValueT> = papaya::HashMap<KeyT, ValueT, FastBuildHasher>;

pub use ahash::HashMapExt;
