/// Fast [BuildHasher](std::hash::BuildHasher).
///
/// Note that the implementation relies on a
/// [non-cryptographic hash function](https://en.wikipedia.org/wiki/Non-cryptographic_hash_function).
pub type FastBuildHasher = ahash::random_state::RandomState;

// Note: rapidhash and foldhash may be faster but they have subtle bugs with papaya
// https://github.com/ibraheemdev/papaya/issues/85
