use aes_prng::{AesRng, SEED_SIZE};
use criterion::{criterion_group, criterion_main, Bencher, BenchmarkId, Criterion};
use rand::{RngCore, SeedableRng};
use rand_chacha::{ChaCha12Rng, ChaCha20Rng, ChaCha8Rng};

fn rng_fill(c: &mut Criterion) {
    fn rng_bencher<T: RngCore>(b: &mut Bencher, rng: &mut T, buffer_size_kb: usize) {
        let buffer_size = buffer_size_kb * 1024;
        let mut output = vec![0u8; buffer_size];
        b.iter(|| {
            rng.fill_bytes(&mut output);
        })
    }

    let mut group = c.benchmark_group("rng_fill");
    for buffer_size_kb in [10, 100, 1000] {
        group.bench_function(BenchmarkId::new("chacha8", buffer_size_kb), |b| {
            let mut rng = ChaCha8Rng::seed_from_u64(42);
            rng_bencher(b, &mut rng, buffer_size_kb)
        });

        group.bench_function(BenchmarkId::new("chacha12", buffer_size_kb), |b| {
            let mut rng = ChaCha12Rng::seed_from_u64(42);
            rng_bencher(b, &mut rng, buffer_size_kb)
        });

        group.bench_function(BenchmarkId::new("chacha20", buffer_size_kb), |b| {
            let mut rng = ChaCha20Rng::seed_from_u64(42);
            rng_bencher(b, &mut rng, buffer_size_kb)
        });

        group.bench_function(BenchmarkId::new("aes", buffer_size_kb), |b| {
            let mut rng = AesRng::from_seed([0u8; SEED_SIZE]);
            rng_bencher(b, &mut rng, buffer_size_kb)
        });
    }
}

fn rng_next64(c: &mut Criterion) {
    fn rng_bencher<T: RngCore>(b: &mut Bencher, rng: &mut T, n: usize) {
        b.iter(|| {
            for _ in 0..n {
                let _ = rng.next_u64();
            }
        })
    }

    let mut group = c.benchmark_group("rng_next_u64");
    for n in [1000] {
        group.bench_function("chacha8", |b| {
            let mut rng = ChaCha8Rng::seed_from_u64(42);
            rng_bencher(b, &mut rng, n)
        });

        group.bench_function("chacha12", |b| {
            let mut rng = ChaCha12Rng::seed_from_u64(42);
            rng_bencher(b, &mut rng, n)
        });

        group.bench_function("chacha20", |b| {
            let mut rng = ChaCha20Rng::seed_from_u64(42);
            rng_bencher(b, &mut rng, n)
        });

        group.bench_function("aes", |b| {
            let mut rng = AesRng::from_seed([0u8; SEED_SIZE]);
            rng_bencher(b, &mut rng, n)
        });
    }
}

criterion_group!(benches, rng_fill, rng_next64);
criterion_main!(benches);
