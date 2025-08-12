use std::collections::*;

/// Fast [HashMap].
///
/// The implementation uses [rapidhash::fast::RandomState].
///
/// Note that rapidhash is a
/// [non-cryptographic hash function](https://en.wikipedia.org/wiki/Non-cryptographic_hash_function).
pub type FastHashMap<KeyT, ValueT> = HashMap<KeyT, ValueT, rapidhash::fast::RandomState>;

/// Fast concurrent [HashMap].
///
/// The implementation uses [papaya::HashMap] and [rapidhash::fast::RandomState].
///
/// Note that rapidhash is a
/// [non-cryptographic hash function](https://en.wikipedia.org/wiki/Non-cryptographic_hash_function).
pub type FastConcurrentHashMap<KeyT, ValueT> = papaya::HashMap<KeyT, ValueT, rapidhash::fast::RandomState>;

pub use rapidhash::HashMapExt;
