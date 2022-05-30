# AES-PRNG

[![Build Status][build-image]][build-link]
[![Apache2 License 2.0][license-image]][license-link]
[![Minimum rustc version][rustc-image]][rustc-link]

A Rust library for random number generation using AES as the underlying block-cipher.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
aes-prng = "0.1.0"
```

## Example

```rust
use rand::{RngCore, SeedableRng};
use aes_prng::AesRng;

let mut rng = AesRng::from_random_seed();

// sample random bytes
let mut bytes = [0; 1024];
rng.fill_bytes(&mut bytes);

// sample random u32
let r_u32 = rng.next_u32();

// sample random u64
let r_u64 = rng.next_u64();
```

`AesRng` can also be created from an existing seed:

```rust
let seed = AesRng::generate_random_key();
let mut rng = AesRng::from_seed(seed);
```

### Rust version requirements

AES-PRNG requires **Rustc version 1.56 or greater** due to the [RustCrypto](
https://github.com/RustCrypto/block-ciphers/tree/master/aes) dependency.

## Crate Features

AES-PRNG is built with the `-Ctarget-feature=+aes` feature enabled by default
to get the benefit of AES-NI instructions for speeding up the PRNG calls.

## Releasing

We release manually using [`carge-release`](https://github.com/crate-ci/cargo-release):

```
$ cargo release minor
```

## License

AES-PRNG is distributed under the terms of Apache License (Version 2.0).


[//]: # (badges)


[build-image]: https://github.com/tf-encrypted/aes-prng/workflows/CI/badge.svg
[build-link]: https://github.com/tf-encrypted/aes-prng/actions
[license-image]: https://img.shields.io/badge/license-Apache%20License%202.0-blue.svg?style=flat
[license-link]: https://www.apache.org/licenses/LICENSE-2.0
[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg
[rustc-link]: https://github.com/tf-encrypted/aes-prng#rust-version-requirements
