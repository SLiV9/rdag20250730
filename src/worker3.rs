use std::collections::hash_map;

use fxhash::FxHashMap;

use crate::common::*;

#[derive(Default)]
pub struct Worker3 {
    instrument_offsets: FxHashMap<Id, usize>,
    deltas: Vec<Delta>,
    gammas: Vec<Gamma>,
    thetas: Vec<Theta>,
    vegas: Vec<Vega>,
}

impl Worker for Worker3 {
    fn update(&mut self, id: Id, greeks: Greeks) {
        match self.instrument_offsets.entry(id) {
            hash_map::Entry::Occupied(occupied_entry) => {
                let offset = *occupied_entry.get();
                self.deltas[offset] = greeks.delta;
                self.gammas[offset] = greeks.gamma;
                self.thetas[offset] = greeks.theta;
                self.vegas[offset] = greeks.vega;
            }
            hash_map::Entry::Vacant(vacant_entry) => {
                let offset = self.deltas.len();
                vacant_entry.insert(offset);
                self.deltas.push(greeks.delta);
                self.gammas.push(greeks.gamma);
                self.thetas.push(greeks.theta);
                self.vegas.push(greeks.vega);
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
