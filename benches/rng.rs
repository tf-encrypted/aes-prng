use aes_prng::{AesRng, SEED_SIZE};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{RngCore, SeedableRng};
use rand_chacha::{ChaCha12Rng, ChaCha20Rng, ChaCha8Rng};

fn rng_fill(c: &mut Criterion) {
    let mut group = c.benchmark_group("rng_fill");
    for buffer_size in [2 * 1000 * 1000] {
        group.bench_function(BenchmarkId::new("chacha8", buffer_size), |b| {
            let mut rng = ChaCha8Rng::seed_from_u64(42);
            let mut output = vec![0u8; buffer_size];
            b.iter(|| {
                rng.try_fill_bytes(&mut output).unwrap();
            })
        });

        group.bench_function(BenchmarkId::new("chacha12", buffer_size), |b| {
            let mut rng = ChaCha12Rng::seed_from_u64(42);
            let mut output = vec![0u8; buffer_size];
            b.iter(|| {
                rng.try_fill_bytes(&mut output).unwrap();
            })
        });

        group.bench_function(BenchmarkId::new("chacha20", buffer_size), |b| {
            let mut rng = ChaCha20Rng::seed_from_u64(42);
            let mut output = vec![0u8; buffer_size];
            b.iter(|| {
                rng.try_fill_bytes(&mut output).unwrap();
            })
        });

        group.bench_function(BenchmarkId::new("aes", buffer_size), |b| {
            let mut rng = AesRng::from_seed([0u8; SEED_SIZE]);
            let mut output = vec![0u8; buffer_size];
            b.iter(|| {
                rng.try_fill_bytes(&mut output).unwrap();
            })
        });
    }
}

fn rng_next64(c: &mut Criterion) {
    let mut group = c.benchmark_group("rng_next_u64");
    group.bench_function("chacha8", |b| {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let n: u64 = 1000;
        b.iter(|| {
            for _ in 0..n {
                let _ = rng.next_u64();
            }
        })
    });

    group.bench_function("chacha12", |b| {
        let mut rng = ChaCha12Rng::seed_from_u64(42);
        let n: u64 = 1000;
        b.iter(|| {
            for _ in 0..n {
                let _ = rng.next_u64();
            }
        })
    });

    group.bench_function("chacha20", |b| {
        let mut rng = ChaCha20Rng::seed_from_u64(42);
        let n: u64 = 1000;
        b.iter(|| {
            for _ in 0..n {
                let _ = rng.next_u64();
            }
        })
    });

    group.bench_function("aes", |b| {
        let mut rng = AesRng::from_seed([0u8; SEED_SIZE]);
        let n: u64 = 1000;
        b.iter(|| {
            for _ in 0..n {
                let _ = rng.next_u64();
            }
        })
    });
}

criterion_group!(benches, rng_fill, rng_next64);
criterion_main!(benches);
