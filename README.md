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

### Rust version requirements

AES-PRNG requires **Rustc version 1.56 or greater** due to the [RustCrypto](
https://github.com/RustCrypto/block-ciphers/tree/master/aes) dependency.

## Crate Features

AES-PRNG is built with the `-Ctarget-feature=+aes` feature enabled by default
to get the benefit of AES-NI instructions for speeding up the PRNG calls.

## Benchmarks

On AMD Ryzen 9 3900X:

```
$ cargo bench -- rng_fill

rng_fill/chacha8/10     time:   [1.7716 us 1.7729 us 1.7742 us]
rng_fill/chacha12/10    time:   [2.4794 us 2.4810 us 2.4831 us]
rng_fill/chacha20/10    time:   [3.9333 us 3.9391 us 3.9444 us]
rng_fill/aes/10         time:   [2.2231 us 2.2232 us 2.2234 us]

rng_fill/chacha8/100    time:   [18.266 us 18.269 us 18.271 us]
rng_fill/chacha12/100   time:   [24.603 us 24.607 us 24.610 us]
rng_fill/chacha20/100   time:   [38.965 us 38.970 us 38.974 us]
rng_fill/aes/100        time:   [24.080 us 24.113 us 24.144 us]

rng_fill/chacha8/1000   time:   [176.70 us 176.71 us 176.73 us]
rng_fill/chacha12/1000  time:   [248.39 us 248.41 us 248.44 us]
rng_fill/chacha20/1000  time:   [391.49 us 391.68 us 391.90 us]
rng_fill/aes/1000       time:   [225.52 us 225.53 us 225.54 us]
```

On Apple M1 Max:

```
$ 
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