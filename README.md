# AES-PRNG

[![Test Status](https://github.com/tf-encrypted/aes-prng/actions/workflows/ci.yml/badge.svg?event=push)](https://github.com/tf-encrypted/aes-prng/actions)
![Apache2/MIT licensed][license-image]
[![Minimum rustc version][https://img.shields.io/badge/rustc-1.56+-blue.svg]](https://github.com/tf-encrypted/aes-prng#rust-version-requirements)

A Rust library for random number generation using AES as the underlying block-cipher.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
aes-prng = "0.1.0"
```

### Rust version requirements

AES-PRNG requires **Rustc version 1.56 or greater** due to the [RustCrypto](
https://github.com/RustCrypto/block-ciphers/tree/master/aes) dependency.

## Crate Features

AES-PRNG is built with the `-Ctarget-feature=+aes` feature enabled by default
to get the benefit of AES-NI instructions for speeding up the PRNG calls.

# License

AES-PRNG is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.


[//]: # (badges)

[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg