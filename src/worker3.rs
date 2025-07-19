use std::collections::hash_map;

use fxhash::FxHashMap;

use crate::common::*;

#[derive(Default)]
pub struct Worker3 {
    instrument_offsets: FxHashMap<InstrumentId, usize>,
    deltas: Vec<Delta>,
    gammas: Vec<Gamma>,
    thetas: Vec<Theta>,
    vegas: Vec<Vega>,
}

impl Worker for Worker3 {
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
                self.deltas[offset] = delta;
                self.gammas[offset] = gamma;
                self.thetas[offset] = theta;
                self.vegas[offset] = vega;
            }
            hash_map::Entry::Vacant(vacant_entry) => {
                let offset = self.deltas.len();
                vacant_entry.insert(offset);
                self.deltas.push(delta);
                self.gammas.push(gamma);
                self.thetas.push(theta);
                self.vegas.push(vega);
            }
        }
    }

    fn total_delta(&self) -> Delta {
        self.deltas.iter().copied().sum()
    }

    fn total_gamma(&self) -> Gamma {
        self.gammas.iter().copied().sum()
    }

    fn total_vega(&self) -> Vega {
        self.vegas.iter().copied().sum()
    }

    fn total_theta(&self) -> Theta {
        self.thetas.iter().copied().sum()
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
