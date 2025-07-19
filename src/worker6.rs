use std::collections::hash_map;

use fxhash::FxHashMap;

use crate::common::*;

#[derive(Default)]
pub struct Worker6 {
    instrument_offsets: FxHashMap<InstrumentId, usize>,
    delta_exposures: Vec<DeltaExposure>,
    gamma_exposures: Vec<GammaExposure>,
    theta_exposures: Vec<ThetaExposure>,
    vega_exposures: Vec<VegaExposure>,
}

impl Worker for Worker6 {
    fn update_exposure(
        &mut self,
        instrument_id: InstrumentId,
        delta_exposure: DeltaExposure,
        gamma_exposure: GammaExposure,
        theta_exposure: ThetaExposure,
        vega_exposure: VegaExposure,
    ) {
        match self.instrument_offsets.entry(instrument_id) {
            hash_map::Entry::Occupied(occupied_entry) => {
                let offset = *occupied_entry.get();
                self.delta_exposures[offset] = delta_exposure;
                self.gamma_exposures[offset] = gamma_exposure;
                self.theta_exposures[offset] = theta_exposure;
                self.vega_exposures[offset] = vega_exposure;
            }
            hash_map::Entry::Vacant(vacant_entry) => {
                let offset = self.delta_exposures.len();
                vacant_entry.insert(offset);
                self.delta_exposures.push(delta_exposure);
                self.gamma_exposures.push(gamma_exposure);
                self.theta_exposures.push(theta_exposure);
                self.vega_exposures.push(vega_exposure);
            }
        }
    }

    fn total_delta_exposure(&self) -> DeltaExposure {
        add_exposures(DeltaExposure(0.0), &self.delta_exposures[..])
    }

    fn total_gamma_exposure(&self) -> GammaExposure {
        add_exposures(GammaExposure(0.0), &self.gamma_exposures[..])
    }

    fn total_vega_exposure(&self) -> VegaExposure {
        add_exposures(VegaExposure(0.0), &self.vega_exposures[..])
    }

    fn total_theta_exposure(&self) -> ThetaExposure {
        add_exposures(ThetaExposure(0.0), &self.theta_exposures[..])
    }

    fn total_exposures(&self) -> Exposures {
        Exposures {
            delta_exposure: self.total_delta_exposure(),
            gamma_exposure: self.total_gamma_exposure(),
            theta_exposure: self.total_theta_exposure(),
            vega_exposure: self.total_vega_exposure(),
        }
    }
}

fn add_exposures<T>(zero: T, mut values: &[T]) -> T
where
    T: Copy + std::ops::Add<Output = T> + std::ops::AddAssign + std::iter::Sum,
{
    const CHUNK_SIZE: usize = 128;
    let mut sums = [zero; CHUNK_SIZE];
    while let Some(chunk) = values.first_chunk() {
        let chunk: &[T; CHUNK_SIZE] = chunk;
        for i in 0..CHUNK_SIZE {
            sums[i] += chunk[i];
        }
        values = &values[CHUNK_SIZE..];
    }
    for (i, v) in values.iter().enumerate() {
        sums[i] += *v;
    }
    sums.iter().copied().sum()
}
