use std::collections::hash_map;

use fxhash::FxHashMap;

use crate::common::*;

#[derive(Default)]
pub struct IntWorker {
    instrument_offsets: FxHashMap<Id, usize>,
    deltas: Vec<i32>,
    gammas: Vec<i32>,
    thetas: Vec<i32>,
    vegas: Vec<i32>,
}

impl Worker for IntWorker {
    fn update(&mut self, id: Id, greeks: Greeks) {
        let delta = int_from_greek(greeks.delta.0);
        let gamma = int_from_greek(greeks.gamma.0);
        let theta = int_from_greek(greeks.theta.0);
        let vega = int_from_greek(greeks.vega.0);
        match self.instrument_offsets.entry(id) {
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
        };
    }

    fn total_delta(&self) -> Delta {
        let sum: i32 = self.deltas.iter().copied().sum();
        Delta(greek_from_int(sum))
    }

    fn total_gamma(&self) -> Gamma {
        let sum: i32 = self.gammas.iter().copied().sum();
        Gamma(greek_from_int(sum))
    }

    fn total_vega(&self) -> Vega {
        let sum: i32 = self.vegas.iter().copied().sum();
        Vega(greek_from_int(sum))
    }

    fn total_theta(&self) -> Theta {
        let sum: i32 = self.thetas.iter().copied().sum();
        Theta(greek_from_int(sum))
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

fn int_from_greek(greek: f32) -> i32 {
    (greek * 1e6) as i32
}

fn greek_from_int(value: i32) -> f32 {
    value as f32 * 1e-6
}
