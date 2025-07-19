use criterion::{criterion_group, criterion_main, Criterion};
use rand::{seq::SliceRandom, Rng};

use std::collections::{hash_map, HashMap};

use derive_more::{Add, AddAssign, Sum};
use fxhash::FxHashMap;

use crate::common::*;

#[derive(Default)]
pub struct Worker2 {
    exposures: HashMap<InstrumentId, Exposures>,
}

impl Worker for Worker2 {
    fn update_exposure(
        &mut self,
        instrument_id: InstrumentId,
        delta_exposure: DeltaExposure,
        gamma_exposure: GammaExposure,
        vega_exposure: VegaExposure,
        theta_exposure: ThetaExposure,
        charm_exposure: CharmExposure,
    ) {
        self.exposures.insert(
            instrument_id,
            Exposures {
                delta_exposure,
                gamma_exposure,
                vega_exposure,
                theta_exposure,
                charm_exposure,
            },
        );
    }

    fn total_delta_exposure(&self) -> DeltaExposure {
        self.exposures.values().map(|x| x.delta_exposure).sum()
    }

    fn total_gamma_exposure(&self) -> GammaExposure {
        self.exposures.values().map(|x| x.gamma_exposure).sum()
    }

    fn total_vega_exposure(&self) -> VegaExposure {
        self.exposures.values().map(|x| x.vega_exposure).sum()
    }

    fn total_theta_exposure(&self) -> ThetaExposure {
        self.exposures.values().map(|x| x.theta_exposure).sum()
    }

    fn total_charm_exposure(&self) -> CharmExposure {
        self.exposures.values().map(|x| x.charm_exposure).sum()
    }

    fn total_exposures(&self) -> Exposures {
        let mut total = Exposures {
            delta_exposure: DeltaExposure(0.0),
            gamma_exposure: GammaExposure(0.0),
            vega_exposure: VegaExposure(0.0),
            theta_exposure: ThetaExposure(0.0),
            charm_exposure: CharmExposure(0.0),
        };
        for entry in self.exposures.values() {
            total.delta_exposure += entry.delta_exposure;
            total.gamma_exposure += entry.gamma_exposure;
            total.vega_exposure += entry.vega_exposure;
            total.theta_exposure += entry.theta_exposure;
            total.charm_exposure += entry.charm_exposure;
        }
        total
    }
}
