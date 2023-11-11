use criterion::{criterion_group, criterion_main, Criterion};
use procedural_generation::*;

pub struct Key(u64);

impl Generate<f64> for Key {
    const XOR: u128 = 2319109883130468428;
    fn distribution() -> Self::Distribution {
        rand::distributions::Standard
    }
}

impl PrngKey for Key {
    fn key(&self) -> u64 {
        self.0
    }
}

criterion_main!(bench_group);
criterion_group!(bench_group, benchmark);

fn benchmark(crit: &mut Criterion) {
    let seed = Seed::from(rand::random::<u128>());
    let keys = (0..128).map(|_| Key(rand::random())).collect::<Vec<_>>();
    let mut values = vec![0.0; 128];
    crit.bench_function("generate 128", |b| {
        b.iter(|| {
            for (key, value) in keys.iter().zip(&mut values) {
                *value = seed.generate(key);
            }
        })
    });
}
