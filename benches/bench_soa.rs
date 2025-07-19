use criterion::{criterion_group, criterion_main, Criterion};
use rand::{seq::SliceRandom, Rng};

use rdag20250730::common::*;

fn bench_one_worker(c: &mut Criterion, mut worker: impl Worker) {
    const N: usize = 10_000;
    let mut rng = rand::rng();
    let mut updates: Vec<_> = (0..N)
        .map(|i| {
            (
                InstrumentId(1 + i as u32),
                DeltaExposure(rng.random_range(-1.0..1.0)),
                GammaExposure(rng.random_range(-1.0..1.0)),
                ThetaExposure(rng.random_range(-1.0..1.0)),
                VegaExposure(rng.random_range(-1.0..1.0)),
            )
        })
        .collect();

    let worker_name = std::any::type_name_of_val(&worker);
    let mut g = c.benchmark_group(worker_name);

    g.bench_function("insert_exposure", |b| {
        updates.shuffle(&mut rng);
        worker = Default::default();
        b.iter(|| {
            let updates = std::hint::black_box(&updates);
            for &update in updates {
                let (id, delta, gamma, theta, vega) = update;
                worker.update_exposure(id, delta, gamma, theta, vega);
            }
        })
    });
    g.bench_function("update_exposure", |b| {
        updates.shuffle(&mut rng);
        b.iter(|| {
            let updates = std::hint::black_box(&updates);
            for &update in updates {
                let (id, delta, gamma, theta, vega) = update;
                worker.update_exposure(id, delta, gamma, theta, vega);
            }
        })
    });
    g.bench_function("total_delta_exposure", |b| {
        b.iter(|| worker.total_delta_exposure())
    });
    g.bench_function("total_exposures", |b| b.iter(|| worker.total_exposures()));
}

fn bench_worker1(c: &mut Criterion) {
    bench_one_worker(c, rdag20250730::worker1::Worker1::default())
}

fn bench_worker2(c: &mut Criterion) {
    bench_one_worker(c, rdag20250730::worker2::Worker2::default())
}

fn bench_worker3(c: &mut Criterion) {
    bench_one_worker(c, rdag20250730::worker3::Worker3::default())
}

fn bench_worker4(c: &mut Criterion) {
    bench_one_worker(c, rdag20250730::worker4::Worker4::default())
}

fn bench_worker5(c: &mut Criterion) {
    bench_one_worker(c, rdag20250730::worker5::Worker5::default())
}

fn bench_worker6(c: &mut Criterion) {
    bench_one_worker(c, rdag20250730::worker6::Worker6::default())
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
