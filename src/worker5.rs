use std::collections::hash_map;

use fxhash::FxHashMap;

use crate::common::*;

#[derive(Default)]
pub struct Worker5 {
    instrument_offsets: FxHashMap<InstrumentId, usize>,
    deltas: Vec<i32>,
    gammas: Vec<i32>,
    thetas: Vec<i32>,
    vegas: Vec<i32>,
}

impl Worker for Worker5 {
    fn update(
        &mut self,
        instrument_id: InstrumentId,
        delta: Delta,
        gamma: Gamma,
        theta: Theta,
        vega: Vega,
    ) {
        match self.instrument_offsets.entry(instrument_id) {
            hash_map::Entry::Occupied(occupied_entry) => {
                let offset = *occupied_entry.get();
                self.deltas[offset] = (delta.0 * 1e6) as i32;
                self.gammas[offset] = (gamma.0 * 1e6) as i32;
                self.thetas[offset] = (theta.0 * 1e6) as i32;
                self.vegas[offset] = (vega.0 * 1e6) as i32;
            }
            hash_map::Entry::Vacant(vacant_entry) => {
                let offset = self.deltas.len();
                vacant_entry.insert(offset);
                self.deltas.push((delta.0 * 1e6) as i32);
                self.gammas.push((gamma.0 * 1e6) as i32);
                self.thetas.push((theta.0 * 1e6) as i32);
                self.vegas.push((vega.0 * 1e6) as i32);
            }
        }
    }

    fn total_delta(&self) -> Delta {
        let sum: i32 = self.deltas.iter().copied().sum();
        Delta(sum as f32 * 1e-6)
    }

    fn total_gamma(&self) -> Gamma {
        let sum: i32 = self.gammas.iter().copied().sum();
        Gamma(sum as f32 * 1e-6)
    }

    fn total_vega(&self) -> Vega {
        let sum: i32 = self.vegas.iter().copied().sum();
        Vega(sum as f32 * 1e-6)
    }

    fn total_theta(&self) -> Theta {
        let sum: i32 = self.thetas.iter().copied().sum();
        Theta(sum as f32 * 1e-6)
    }

    fn total_greeks(&self) -> Greeks {
        Greeks {
            delta: self.total_delta(),
            gamma: self.total_gamma(),
            theta: self.total_theta(),
            vega: self.total_vega(),
        }
    }
}
