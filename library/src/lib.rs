// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

/*!
The word "kutil" means "do-it-yourselfer" in Czech.

For more information and usage examples see the
[home page](https://github.com/tliron/kutil).
*/

/// CLI utilities.
#[cfg(feature = "cli")]
pub mod cli;

/// HTTP utilities.
#[cfg(feature = "http")]
pub mod http;

/// I/O utilities.
#[cfg(feature = "io")]
pub mod io;

/// Standard library utilities.
#[cfg(feature = "std")]
pub mod std;

/// Transcoding utilities.
#[cfg(feature = "transcoding")]
pub mod transcoding;

// This allows us to use derive macros.
extern crate self as kutil;
