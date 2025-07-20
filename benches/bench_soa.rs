use criterion::{
    criterion_group, criterion_main, Criterion,
};
use rand::{seq::SliceRandom, Rng};

use rdag20250730 as w;
use rdag20250730::common::*;

fn bench_one_worker(
    c: &mut Criterion,
    mut worker: impl Worker + Default,
) {
    const N: usize = 10_000;
    let mut rng = rand::rng();
    let mut updates: Vec<_> = (0..N)
        .map(|i| {
            let id = Id(1 + i as u32);
            let greeks = Greeks {
                delta: Delta(rng.random_range(-1.0..1.0)),
                gamma: Gamma(rng.random_range(-1.0..1.0)),
                theta: Theta(rng.random_range(-1.0..1.0)),
                vega: Vega(rng.random_range(-1.0..1.0)),
            };
            (id, greeks)
        })
        .collect();

    let worker_name = std::any::type_name_of_val(&worker);
    let mut g = c.benchmark_group(worker_name);

    g.bench_function("insert", |b| {
        updates.shuffle(&mut rng);
        worker = Default::default();
        b.iter(|| {
            let updates = std::hint::black_box(&updates);
            for &update in updates {
                let (id, greeks) = update;
                worker.update(id, greeks);
            }
        })
    });
    g.bench_function("update", |b| {
        updates.shuffle(&mut rng);
        b.iter(|| {
            let updates = std::hint::black_box(&updates);
            for &update in updates {
                let (id, greeks) = update;
                worker.update(id, greeks);
            }
        })
    });
    g.bench_function("total_delta", |b| {
        b.iter(|| worker.total_delta())
    });
    g.bench_function("total_greeks", |b| {
        b.iter(|| worker.total_greeks())
    });
}

fn bench_worker1(c: &mut Criterion) {
    bench_one_worker(c, w::worker1::NaiveWorker::default())
}

fn bench_worker2(c: &mut Criterion) {
    bench_one_worker(
        c,
        w::worker2::HashMapWorker::default(),
    )
}

fn bench_worker3(c: &mut Criterion) {
    bench_one_worker(c, w::worker3::Worker3::default())
}

fn bench_worker4(c: &mut Criterion) {
    bench_one_worker(c, w::worker4::Worker4::default())
}

fn bench_worker5(c: &mut Criterion) {
    bench_one_worker(c, w::worker5::Worker5::default())
}

fn bench_worker6(c: &mut Criterion) {
    bench_one_worker(c, w::worker6::Worker6::default())
}

criterion_group!(
    benches,
    bench_worker1,
    bench_worker2,
    bench_worker3,
    bench_worker4,
    bench_worker5,
    bench_worker6,
);
criterion_main!(benches);
