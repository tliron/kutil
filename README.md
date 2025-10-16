[![crates.io](https://img.shields.io/crates/v/kutil?color=%23227700)](https://crates.io/crates/kutil)
[![docs.rs](https://img.shields.io/badge/docs.rs-latest?color=grey)](https://docs.rs/kutil/latest/kutil/)

Kutil
=====

Rust utilities collection.

The word "kutil" means "do-it-yourselfer" in Czech.

Highlights:

* [cli](https://docs.rs/kutil/latest/kutil/cli/index.html):
  * [Handle exit codes in `main()`](https://docs.rs/kutil/latest/kutil/cli/run/index.html)
  * [Clap](https://github.com/clap-rs/clap)
    [helpers](https://docs.rs/kutil/latest/kutil/cli/clap/index.html)
  * [Initialize logging](https://docs.rs/kutil/latest/kutil/cli/log/index.html)
    (via [tracing](https://github.com/tokio-rs/tracing))
* [http](https://docs.rs/kutil/latest/kutil/http/index.html):
  * [Easy access to headers](https://docs.rs/kutil/latest/kutil/http/trait.HeaderValues.html)
  * Conditional HTTP and content negotiation
  * [Read body into bytes](https://docs.rs/kutil/latest/kutil/http/trait.ReadBodyIntoBytes.html)
  * [Response caching layer with integrated encoding (compression) for Tower](https://docs.rs/kutil/latest/kutil/http/tower/caching/struct.CachingLayer.html)
* [io](https://docs.rs/kutil/latest/kutil/io/index.html):
  * Adapters and utilities for `Read`, `Stream`, and Tokio's `AsyncRead`
  * [IP address discovery for servers](https://docs.rs/kutil/latest/kutil/io/network/ip/index.html) (dual-stack IPv6 and IPv4)
* [std](https://docs.rs/kutil/latest/kutil/std/index.html):
  * [Fostering](https://docs.rs/kutil/latest/kutil/std/foster/index.html)
  * [Error accumulation](https://docs.rs/kutil/latest/kutil/std/error/index.html)
  * [Fast collections](https://docs.rs/kutil/latest/kutil/std/collections/index.html)
  * Iterators, futures, and more
* [transcoding](https://docs.rs/kutil/latest/kutil/transcoding/index.html):
  * Async encoding/decoding for popular web compression formats

License
-------

Like much of the Rust ecosystem, licensed under your choice of either of

* [Apache License, Version 2.0](https://github.com/tliron/kutil/blob/main/LICENSE-APACHE)
* [MIT license](https://github.com/tliron/kutil/blob/main/LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
