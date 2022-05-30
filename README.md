# AES-PRNG

[![Build Status][build-image]][build-link]
[![Apache2 License 2.0][license-image]][license-link]
[![Minimum rustc version][rustc-image]][rustc-link]

A Rust library for random number generation using AES as the underlying block-cipher.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
aes-prng = "0.1"
```

### Rust version requirements

AES-PRNG requires **Rustc version 1.56 or greater** due to the [RustCrypto](
https://github.com/RustCrypto/block-ciphers/tree/master/aes) dependency.

## Crate Features

AES-PRNG is built with the `-Ctarget-feature=+aes` feature enabled by default
to get the benefit of AES-NI instructions for speeding up the PRNG calls.

## Benchmarks

### AMD Ryzen 9 3900X

```
$ cargo bench -- rng_fill

rng_fill/chacha8/100    time:   [18.266 us 18.269 us 18.271 us]
rng_fill/chacha12/100   time:   [24.603 us 24.607 us 24.610 us]
rng_fill/chacha20/100   time:   [38.965 us 38.970 us 38.974 us]
rng_fill/aes/100        time:   [24.080 us 24.113 us 24.144 us]

rng_fill/chacha8/1000   time:   [176.70 us 176.71 us 176.73 us]
rng_fill/chacha12/1000  time:   [248.39 us 248.41 us 248.44 us]
rng_fill/chacha20/1000  time:   [391.49 us 391.68 us 391.90 us]
rng_fill/aes/1000       time:   [225.52 us 225.53 us 225.54 us]
```

### Apple M1 Max

```
$ cargo bench -- rng_fill

rng_fill/chacha8/100    time:   [82.938 us 83.033 us 83.144 us]
rng_fill/chacha12/100   time:   [120.63 us 120.84 us 121.05 us]
rng_fill/chacha20/100   time:   [195.85 us 196.17 us 196.51 us]
rng_fill/aes/100        time:   [414.90 us 415.26 us 415.71 us]

rng_fill/chacha8/1000   time:   [833.53 us 834.31 us 835.25 us]
rng_fill/chacha12/1000  time:   [1.2083 ms 1.2093 ms 1.2106 ms]
rng_fill/chacha20/1000  time:   [1.9600 ms 1.9638 ms 1.9685 ms]
rng_fill/aes/1000       time:   [4.1675 ms 4.1731 ms 4.1792 ms]
```

```
$ RUSTUP_TOOLCHAIN=nightly \
  RUSTFLAGS="--cfg aes_armv8" \
  cargo bench -- rng_fill

rng_fill/chacha8/100    time:   [74.994 us 75.104 us 75.223 us]
rng_fill/chacha12/100   time:   [109.58 us 109.75 us 109.95 us]
rng_fill/chacha20/100   time:   [179.29 us 179.52 us 179.79 us]
rng_fill/aes/100        time:   [11.019 us 11.064 us 11.113 us]

rng_fill/chacha8/1000   time:   [751.56 us 752.02 us 752.55 us]
rng_fill/chacha12/1000  time:   [1.1022 ms 1.1036 ms 1.1054 ms]
rng_fill/chacha20/1000  time:   [1.8051 ms 1.8100 ms 1.8157 ms]
rng_fill/aes/1000       time:   [112.58 us 113.03 us 113.49 us]
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