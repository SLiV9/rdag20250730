
use std::collections::hash_map;

use fxhash::FxHashMap;

use crate::common::*;

#[derive(Default)]
pub struct Worker5 {
    instrument_offsets: FxHashMap<InstrumentId, usize>,
    delta_exposures: Vec<i32>,
    gamma_exposures: Vec<i32>,
    vega_exposures: Vec<i32>,
    theta_exposures: Vec<i32>,
    charm_exposures: Vec<i32>,
}

impl Worker for Worker5 {
    fn update_exposure(
        &mut self,
        instrument_id: InstrumentId,
        delta_exposure: DeltaExposure,
        gamma_exposure: GammaExposure,
        vega_exposure: VegaExposure,
        theta_exposure: ThetaExposure,
        charm_exposure: CharmExposure,
    ) {
        match self.instrument_offsets.entry(instrument_id) {
            hash_map::Entry::Occupied(occupied_entry) => {
                let offset = *occupied_entry.get();
                self.delta_exposures[offset] = (delta_exposure.0 * 1e6) as i32;
                self.gamma_exposures[offset] = (gamma_exposure.0 * 1e6) as i32;
                self.vega_exposures[offset] = (vega_exposure.0 * 1e6) as i32;
                self.theta_exposures[offset] = (theta_exposure.0 * 1e6) as i32;
                self.charm_exposures[offset] = (charm_exposure.0 * 1e6) as i32;
            }
            hash_map::Entry::Vacant(vacant_entry) => {
                let offset = self.delta_exposures.len();
                vacant_entry.insert(offset);
                self.delta_exposures.push((delta_exposure.0 * 1e6) as i32);
                self.gamma_exposures.push((gamma_exposure.0 * 1e6) as i32);
                self.vega_exposures.push((vega_exposure.0 * 1e6) as i32);
                self.theta_exposures.push((theta_exposure.0 * 1e6) as i32);
                self.charm_exposures.push((charm_exposure.0 * 1e6) as i32);
            }
        }
    }

    fn total_delta_exposure(&self) -> DeltaExposure {
        let sum: i32 = self.delta_exposures.iter().copied().sum();
        DeltaExposure(sum as f32 * 1e-6)
    }

    fn total_gamma_exposure(&self) -> GammaExposure {
        let sum: i32 = self.gamma_exposures.iter().copied().sum();
        GammaExposure(sum as f32 * 1e-6)
    }

    fn total_vega_exposure(&self) -> VegaExposure {
        let sum: i32 = self.vega_exposures.iter().copied().sum();
        VegaExposure(sum as f32 * 1e-6)
    }

    fn total_theta_exposure(&self) -> ThetaExposure {
        let sum: i32 = self.theta_exposures.iter().copied().sum();
        ThetaExposure(sum as f32 * 1e-6)
    }

    fn total_charm_exposure(&self) -> CharmExposure {
        let sum: i32 = self.charm_exposures.iter().copied().sum();
        CharmExposure(sum as f32 * 1e-6)
    }

    fn total_exposures(&self) -> Exposures {
        Exposures {
            delta_exposure: self.total_delta_exposure(),
            gamma_exposure: self.total_gamma_exposure(),
            vega_exposure: self.total_vega_exposure(),
            theta_exposure: self.total_theta_exposure(),
            charm_exposure: self.total_charm_exposure(),
        }
    }
}
