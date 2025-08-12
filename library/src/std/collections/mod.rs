#[cfg(feature = "bimap")]
mod bimap;
#[cfg(feature = "fast_collections")]
mod hashmaps;
#[cfg(feature = "fast_collections")]
mod hashsets;

#[cfg(feature = "fast_collections")]
#[allow(unused_imports)]
pub use {bimap::*, hashmaps::*, hashsets::*};
