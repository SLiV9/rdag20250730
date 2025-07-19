use criterion::{criterion_group, criterion_main, Criterion};
use rand::{seq::SliceRandom, Rng};

use std::collections::{hash_map, HashMap};

use derive_more::{Add, AddAssign, Sum};
use fxhash::FxHashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InstrumentId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Add, AddAssign, Sum)]
pub struct DeltaExposure(pub f32);

#[derive(Clone, Copy, Debug, PartialEq, Add, AddAssign, Sum)]
pub struct GammaExposure(pub f32);

#[derive(Clone, Copy, Debug, PartialEq, Add, AddAssign, Sum)]
pub struct VegaExposure(pub f32);

#[derive(Clone, Copy, Debug, PartialEq, Add, AddAssign, Sum)]
pub struct ThetaExposure(pub f32);

#[derive(Clone, Copy, Debug, PartialEq, Add, AddAssign, Sum)]
pub struct CharmExposure(pub f32);

pub struct Exposures {
    pub delta_exposure: DeltaExposure,
    pub gamma_exposure: GammaExposure,
    pub vega_exposure: VegaExposure,
    pub theta_exposure: ThetaExposure,
    pub charm_exposure: CharmExposure,
}

pub trait Worker {
    fn update_exposure(
        &mut self,
        instrument_id: InstrumentId,
        delta_exposure: DeltaExposure,
        gamma_exposure: GammaExposure,
        vega_exposure: VegaExposure,
        theta_exposure: ThetaExposure,
        charm_exposure: CharmExposure,
    );

    fn total_delta_exposure(&self) -> DeltaExposure;
    fn total_gamma_exposure(&self) -> GammaExposure;
    fn total_vega_exposure(&self) -> VegaExposure;
    fn total_theta_exposure(&self) -> ThetaExposure;
    fn total_charm_exposure(&self) -> CharmExposure;
    fn total_exposures(&self) -> Exposures;
}
