Kutil for Rust
==============

Various Rust utilities.

The word "kutil" means "do-it-yourselfer" in Czech.

Crates:

* [kutil-cli](https://docs.rs/kutil-cli):
  * [Handle exit codes in `main()`](https://docs.rs/kutil-cli/latest/kutil_cli/run/index.html)
  * [Clap](https://github.com/clap-rs/clap)
    [helpers](https://docs.rs/kutil-cli/latest/kutil_cli/clap/index.html)
  * [Initialize logging](https://docs.rs/kutil-cli/latest/kutil_cli/debug/index.html)
    (via [tracing](https://github.com/tokio-rs/tracing))
  * The [Depict trait](https://docs.rs/kutil-cli/latest/kutil_cli/depict/index.html) is a supercharged version of `Debug`
* [kutil-http](https://docs.rs/kutil-http):
  * [Easy access to headers](https://docs.rs/kutil-http/latest/kutil_http/trait.HeaderValues.html)
  * Conditional HTTP and content negotiation
  * [Read body into bytes](https://docs.rs/kutil-http/latest/kutil_http/trait.ReadBodyIntoBytes.html)
  * [Response caching layer with integrated encoding (compression) for Tower](https://docs.rs/kutil-http/latest/kutil_http/tower/caching/struct.CachingLayer.html)
* [kutil-io](https://docs.rs/kutil-io):
  * Adapters and utilities for `Read`, `Stream`, and Tokio's `AsyncRead`
  * [IP address discovery for servers](https://docs.rs/kutil-io/latest/kutil_io/network/ip/index.html) (dual-stack IPv6 and IPv4)
* [kutil-std](https://docs.rs/kutil-std):
  * [Fostering](https://docs.rs/kutil-std/latest/kutil_std/foster/index.html)
  * [Error accumulation](https://docs.rs/kutil-std/latest/kutil_std/errors/index.html)
  * [Fast collections](https://docs.rs/kutil-std/latest/kutil_std/collections/index.html)
  * Iterators, futures, and more
* [kutil-transcoding](https://docs.rs/kutil-transcoding):
  * Async encoding/decoding for popular web compression formats

License
-------

Like much of the Rust ecosystem, licensed under your choice of either of

* [Apache License, Version 2.0](https://github.com/tliron/kutil/blob/main/LICENSE-APACHE)
* [MIT license](https://github.com/tliron/kutil/blob/main/LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
