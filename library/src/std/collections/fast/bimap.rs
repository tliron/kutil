use super::build_hasher::*;

use bimap::*;

/// Fast [BiHashMap].
///
/// Note that the implementation relies on a
/// [non-cryptographic hash function](https://en.wikipedia.org/wiki/Non-cryptographic_hash_function).
#[cfg(feature = "fast_collections")]
pub type FastBiHashMap<LeftT, RightT> = BiHashMap<LeftT, RightT, FastBuildHasher>;
