use std::collections::hash_map;

use fxhash::FxHashMap;

use crate::common::*;
use crate::fast_worker::fast_sum;

pub struct HotPathWorker {
    instrument_offsets: FxHashMap<Id, usize>,
    deltas: Vec<Delta>,
    gammas: Vec<Gamma>,
    thetas: Vec<Theta>,
    vegas: Vec<Vega>,
    cached_total_delta: Delta,
}

impl Default for HotPathWorker {
    fn default() -> HotPathWorker {
        HotPathWorker {
            instrument_offsets: Default::default(),
            deltas: Default::default(),
            gammas: Default::default(),
            thetas: Default::default(),
            vegas: Default::default(),
            cached_total_delta: Delta(0.0),
        }
    }
}

impl Worker for HotPathWorker {
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
        self.cached_total_delta =
            fast_sum(Delta(0.0), &self.deltas[..]);
    }

    fn total_delta(&self) -> Delta {
        self.cached_total_delta
    }

    fn total_gamma(&self) -> Gamma {
        fast_sum(Gamma(0.0), &self.gammas[..])
    }

    fn total_vega(&self) -> Vega {
        fast_sum(Vega(0.0), &self.vegas[..])
    }

    fn total_theta(&self) -> Theta {
        fast_sum(Theta(0.0), &self.thetas[..])
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
