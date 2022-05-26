# AES-PRNG

[![Build Status][build-image]][build-link]
![Apache2 License 2.0][license-image]
[![Minimum rustc version][rustc-image]][rustc-link]

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

AES-PRNG is distributed under the terms of Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) for details.


[//]: # (badges)


[build-image]: https://github.com/tf-encrypted/aes-prng/workflows/CI/badge.svg
[build-link]: https://github.com/tf-encrypted/aes-prng/actions
[license-image]: https://img.shields.io/badge/license-Apache%20License%202.0-blue.svg?style=flat
[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg
[rustc-link]: https://github.com/tf-encrypted/aes-prng#rust-version-requirements