use criterion::{criterion_group, criterion_main, Criterion};
use rand::{seq::SliceRandom, Rng};

use std::collections::{hash_map, HashMap};

use derive_more::{Add, AddAssign, Sum};
use fxhash::FxHashMap;

use crate::common::*;

#[derive(Default)]
pub struct Worker4 {
    instrument_offsets: FxHashMap<InstrumentId, usize>,
    exposures: Vec<Exposures>,
}

impl Worker for Worker4 {
    fn update_exposure(
        &mut self,
        instrument_id: InstrumentId,
        delta_exposure: DeltaExposure,
        gamma_exposure: GammaExposure,
        vega_exposure: VegaExposure,
        theta_exposure: ThetaExposure,
        charm_exposure: CharmExposure,
    ) {
        let exposures = Exposures {
            delta_exposure,
            gamma_exposure,
            vega_exposure,
            theta_exposure,
            charm_exposure,
        };
        match self.instrument_offsets.entry(instrument_id) {
            hash_map::Entry::Occupied(occupied_entry) => {
                let offset = *occupied_entry.get();
                self.exposures[offset] = exposures;
            }
            hash_map::Entry::Vacant(vacant_entry) => {
                let offset = self.exposures.len();
                vacant_entry.insert(offset);
                self.exposures.push(exposures);
            }
        }
    }

    fn total_delta_exposure(&self) -> DeltaExposure {
        self.exposures.iter().map(|x| x.delta_exposure).sum()
    }

    fn total_gamma_exposure(&self) -> GammaExposure {
        self.exposures.iter().map(|x| x.gamma_exposure).sum()
    }

    fn total_vega_exposure(&self) -> VegaExposure {
        self.exposures.iter().map(|x| x.vega_exposure).sum()
    }

    fn total_theta_exposure(&self) -> ThetaExposure {
        self.exposures.iter().map(|x| x.theta_exposure).sum()
    }

    fn total_charm_exposure(&self) -> CharmExposure {
        self.exposures.iter().map(|x| x.charm_exposure).sum()
    }

    fn total_exposures(&self) -> Exposures {
        let mut total = Exposures {
            delta_exposure: DeltaExposure(0.0),
            gamma_exposure: GammaExposure(0.0),
            vega_exposure: VegaExposure(0.0),
            theta_exposure: ThetaExposure(0.0),
            charm_exposure: CharmExposure(0.0),
        };
        for entry in &self.exposures {
            total.delta_exposure += entry.delta_exposure;
            total.gamma_exposure += entry.gamma_exposure;
            total.vega_exposure += entry.vega_exposure;
            total.theta_exposure += entry.theta_exposure;
            total.charm_exposure += entry.charm_exposure;
        }
        total
    }
}
