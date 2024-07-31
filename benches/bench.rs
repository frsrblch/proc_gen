use criterion::{criterion_group, criterion_main, Criterion};
use proc_gen::*;
use rand::{thread_rng, Rng};

#[derive(Copy, Clone)]
pub struct Key(u64);

impl KeyFor<f64> for Key {
    const XOR: u128 = 2319109883130468428;
    const BITSHIFT: u32 = 0;
}

impl PrngKey for Key {
    fn key(&self) -> u64 {
        self.0
    }
}

criterion_main!(bench_group);
criterion_group!(bench_group, benchmark);

fn benchmark(crit: &mut Criterion) {
    let mut rng = thread_rng();
    let seed = rng.gen();

    let keys = (0..128)
        .map(|_| Key(rng.gen_range(0..2u64.pow(12))))
        .collect::<Vec<_>>();

    let mut values = vec![0.0; 128];
    crit.bench_function("generate 128", |b| {
        b.iter(|| {
            for (key, value) in keys.iter().zip(&mut values) {
                let mut prng = f64::prng(seed, *key);
                *value = prng.gen();
            }
        })
    });
}
