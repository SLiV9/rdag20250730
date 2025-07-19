use criterion::{criterion_group, criterion_main, Criterion};
use rand::{seq::SliceRandom, Rng};

use std::collections::{hash_map, HashMap};

use derive_more::{Add, AddAssign, Sum};
use fxhash::FxHashMap;

use rdag20250730::common::*;

fn bench_one_worker(c: &mut Criterion, mut worker: impl Worker) {
    const N: usize = 500_000;
    let mut rng = rand::rng();
    let mut instruments: [InstrumentId; N] = std::array::from_fn(|i| InstrumentId(1 + i as u32));

    instruments.shuffle(&mut rng);
    for &instrument_id in &instruments {
        worker.update_exposure(
            instrument_id,
            DeltaExposure(rng.random_range(-1.0..1.0)),
            GammaExposure(rng.random_range(-1.0..1.0)),
            VegaExposure(rng.random_range(-1.0..1.0)),
            ThetaExposure(rng.random_range(-1.0..1.0)),
            CharmExposure(rng.random_range(-1.0..1.0)),
        );
    }

    let worker_name = std::any::type_name_of_val(&worker);
    let mut g = c.benchmark_group(worker_name);

    // g.bench_function("update_exposure", |b| {
    //     instruments.shuffle(&mut rng);
    //     b.iter(|| {
    //         for &instrument_id in instruments.iter().take(N / 2) {
    //             worker.update_exposure(
    //                 instrument_id,
    //                 DeltaExposure(rng.random_range(0.01..9.99)),
    //                 GammaExposure(rng.random_range(0.001..0.999)),
    //                 VegaExposure(rng.random_range(0.1..99.9)),
    //                 ThetaExposure(rng.random_range(0.0001..0.0999)),
    //                 CharmExposure(rng.random_range(1.0..999.0)),
    //             );
    //         }
    //     })
    // });

    g.bench_function("total_delta_exposure", |b| {
        b.iter(|| worker.total_delta_exposure())
    });
    // g.bench_function("total_gamma_exposure", |b| {
    //     b.iter(|| worker.total_gamma_exposure())
    // });
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
