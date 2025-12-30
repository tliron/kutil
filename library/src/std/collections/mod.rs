mod bimap;

#[cfg(feature = "fast_collections")]
mod fast;

pub use bimap::*;

#[cfg(feature = "fast_collections")]
pub use fast::*;
