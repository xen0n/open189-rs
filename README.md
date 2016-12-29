# open189-rs  [![Documentation](https://docs.rs/open189/badge.svg)][docs-rs] [![Crates.io version](https://img.shields.io/crates/v/open189.svg)][cratesio] [![Crates.io downloads](https://img.shields.io/crates/dv/open189.svg)][cratesio] [![Crates.io license](https://img.shields.io/crates/l/open189.svg)](#License)

[![Build Status](https://img.shields.io/travis/xen0n/open189-rs/develop.svg)](https://travis-ci.org/xen0n/open189-rs)

A Rust client for the [open.189.cn] (天翼开放平台) API.

Documentation is [available at docs.rs][docs-rs]. I *might* translate all of
the docs to Chinese, as the service is largely useless for anyone outside of
Mainland China, but since the i18n story of `rustdoc` isn't really there yet,
I decided to postpone this.

[open.189.cn]: http://open.189.cn
[cratesio]: https://crates.io/crates/open189
[docs-rs]: https://docs.rs/open189


## License

`open189-rs` is licensed the same as Rust: dual Apache 2.0 and MIT. See
[LICENSE-Apache](./LICENSE-Apache) and [LICENSE-MIT](./LICENSE-MIT) for details.


## Usage

Check out [the docs][docs-rs] or [the `examples/` directory](./examples) to
learn how to use the library. You can send verification codes with SMS to
mobile phones in Mainland China with this library, actually it's almost what the
whole API service is all about. (The API has more features beyond SMS sending,
but people are hardly making use of any of those. That's another ~~rant~~ story,
on how the Chinese IT industry got so fragmented, with so many vendors trying
to build an omni-platform once and for all, instead of focusing on where each
vendor excels.)


## Changelog

See [CHANGELOG.md](./CHANGELOG.md) for details.


<!-- vim:set ai et ts=4 sw=4 sts=4 fenc=utf-8: -->
