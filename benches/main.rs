use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use simrs_benchmark::simulation;

fn bench(c: &mut Criterion) {
        let mut group = c.benchmark_group("simrs_spsc");
        for limit in [10000, 20000, 30000, 40000, 50000] {
                group.bench_with_input(BenchmarkId::from_parameter(limit), &limit, |b, &limit| {
                        b.iter(|| simulation(black_box(limit)));
                    });
            }
        group.finish();
        // c.bexnch_function("spsc unbound", |b| b.iter(|| simulation()));
    }

criterion_group!(benches, bench);
criterion_main!(benches);
