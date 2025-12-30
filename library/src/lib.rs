// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![doc = include_str!("../../README.md")]

/// CLI utilities.
#[cfg(feature = "cli")]
pub mod cli;

/// HTTP utilities.
#[cfg(feature = "http")]
pub mod http;

/// I/O utilities.
#[cfg(feature = "io")]
pub mod io;

/// pyo3 utilities.
#[cfg(feature = "pyo3")]
pub mod pyo3;

/// Standard library utilities.
#[cfg(feature = "std")]
pub mod std;

/// TLS utilities.
#[cfg(feature = "tls")]
pub mod tls;

/// Transcoding utilities.
#[cfg(feature = "transcoding")]
pub mod transcoding;

// This allows us to use derive macros.
extern crate self as kutil;
