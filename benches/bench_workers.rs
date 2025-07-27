use criterion::{
    criterion_group, criterion_main, Criterion,
};
use rand::{seq::SliceRandom, Rng};

use rdag20250730::aos_worker::AosWorker;
use rdag20250730::common::*;
use rdag20250730::fast_worker::FastWorker;
use rdag20250730::fxhash_worker::FxHashWorker;
use rdag20250730::hashmap_worker::HashMapWorker;
use rdag20250730::hotpath_worker::HotPathWorker;
use rdag20250730::int_worker::IntWorker;
use rdag20250730::naive_worker::NaiveWorker;
use rdag20250730::soa_worker::SoaWorker;

fn bench_worker(
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
            for &update in std::hint::black_box(&updates) {
                let (id, greeks) = update;
                worker.update(id, greeks);
            }
        })
    });
    g.bench_function("update", |b| {
        updates.shuffle(&mut rng);
        b.iter(|| {
            for &update in std::hint::black_box(&updates) {
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

fn bench_naive_worker(c: &mut Criterion) {
    bench_worker(c, NaiveWorker::default())
}

fn bench_hashmap_worker(c: &mut Criterion) {
    bench_worker(c, HashMapWorker::default())
}

fn bench_fxhash_worker(c: &mut Criterion) {
    bench_worker(c, FxHashWorker::default())
}

fn bench_soa_worker(c: &mut Criterion) {
    bench_worker(c, SoaWorker::default())
}

fn bench_aos_worker(c: &mut Criterion) {
    bench_worker(c, AosWorker::default())
}

fn bench_int_worker(c: &mut Criterion) {
    bench_worker(c, IntWorker::default())
}

fn bench_fast_worker(c: &mut Criterion) {
    bench_worker(c, FastWorker::default())
}

fn bench_hotpath_worker(c: &mut Criterion) {
    bench_worker(c, HotPathWorker::default())
}

criterion_group!(
    benches,
    bench_naive_worker,
    bench_hashmap_worker,
    bench_fxhash_worker,
    bench_soa_worker,
    bench_aos_worker,
    bench_int_worker,
    bench_fast_worker,
    bench_hotpath_worker,
);
criterion_main!(benches);
