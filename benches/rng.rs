use aes_prng::{AesRng, SEED_SIZE};
use criterion::{criterion_group, criterion_main, Criterion};
use rand::{RngCore, SeedableRng};
use rand_chacha::{ChaCha12Rng, ChaCha20Rng, ChaCha8Rng};

fn aes_rng(c: &mut Criterion) {
    c.bench_function("aes_rng_fill_2MB", |b| {
        let mut rng = AesRng::from_seed([0u8; SEED_SIZE]);
        let mut output = vec![0u8; 2 * 1000 * 1000];
        b.iter(|| {
            rng.try_fill_bytes(&mut output).unwrap();
        })
    });

    c.bench_function("aes_rng_next_u64", |b| {
        let mut rng = AesRng::from_seed([0u8; SEED_SIZE]);
        let n: u64 = 1000;
        b.iter(|| {
            for _ in 0..n {
                let _ = rng.next_u64();
            }
        })
    });
}

fn chacha_rng(c: &mut Criterion) {
    c.bench_function("chacha8_rng_fill_2MB", |b| {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let mut output = vec![0u8; 2 * 1024 * 1024];
        b.iter(|| {
            rng.try_fill_bytes(&mut output).unwrap();
        })
    });

    c.bench_function("chacha12_rng_fill_2MB", |b| {
        let mut rng = ChaCha12Rng::seed_from_u64(42);
        let mut output = vec![0u8; 2 * 1000 * 1000];
        b.iter(|| {
            rng.try_fill_bytes(&mut output).unwrap();
        })
    });

    c.bench_function("chacha20_rng_fill_2MB", |b| {
        let mut rng = ChaCha20Rng::seed_from_u64(42);
        let mut output = vec![0u8; 2 * 1000 * 1000];
        b.iter(|| {
            rng.try_fill_bytes(&mut output).unwrap();
        })
    });

    c.bench_function("chacha8_rng_next_u64", |b| {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let n: u64 = 1000;
        b.iter(|| {
            for _ in 0..n {
                let _ = rng.next_u64();
            }
        })
    });

    c.bench_function("chacha12_rng_next_u64", |b| {
        let mut rng = ChaCha12Rng::seed_from_u64(42);
        let n: u64 = 1000;
        b.iter(|| {
            for _ in 0..n {
                let _ = rng.next_u64();
            }
        })
    });

    c.bench_function("chacha20_rng_next_u64", |b| {
        let mut rng = ChaCha20Rng::seed_from_u64(42);
        let n: u64 = 1000;
        b.iter(|| {
            for _ in 0..n {
                let _ = rng.next_u64();
            }
        })
    });
}

fn thread_rng(c: &mut Criterion) {
    c.bench_function("thread_rng_16B_with_setup", |b| {
        let mut rng = rand::thread_rng();
        let mut output = vec![0u8; 16];
        b.iter(|| {
            rng.fill_bytes(&mut output);
        })
    });
    c.bench_function("thread_rng_16B_no_setup", |b| {
        b.iter(|| {
            let mut rng = rand::thread_rng();
            let mut output = vec![0u8; 16];
            rng.fill_bytes(&mut output);
        })
    });

    c.bench_function("thread_rng_16B_10000", |b| {
        let n: u64 = 10000;
        let mut rng = rand::thread_rng();
        let mut output = vec![0u8; 16];
        b.iter(|| {
            for _ in 0..n {
                rng.fill_bytes(&mut output);
            }
        })
    });
}

criterion_group!(benches, thread_rng, chacha_rng, aes_rng);
criterion_main!(benches);
