mod r#trait;

#[allow(unused_imports)]
pub use r#trait::*;

#[cfg(any(all(feature = "tls-self-contained", not(feature = "tls-platform")), feature = "_blanket"))]
mod self_contained;

#[cfg(all(feature = "tls-platform", not(feature = "tls-self-contained"), not(feature = "_blanket")))]
mod platform;
