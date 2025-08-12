use std::collections::*;

/// Fast [HashSet].
///
/// The implementation uses [rapidhash::fast::RandomState].
///
/// Note that this rapidhash a
/// [non-cryptographic hash function](https://en.wikipedia.org/wiki/Non-cryptographic_hash_function).
pub type FastHashSet<ValueT> = HashSet<ValueT, rapidhash::fast::RandomState>;

pub use rapidhash::HashSetExt;
