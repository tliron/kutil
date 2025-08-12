pub use bimap::{BiBTreeMap, BiHashMap};

/// Fast [BiHashMap](BiHashMap).
///
/// The implementation uses [rapidhash::fast::RandomState].
///
/// Note that rapidhash is a
/// [non-cryptographic hash function](https://en.wikipedia.org/wiki/Non-cryptographic_hash_function).
#[cfg(feature = "fast_collections")]
pub type FastBiHashMap<LeftT, RightT> = BiHashMap<LeftT, RightT, rapidhash::fast::RandomState>;
