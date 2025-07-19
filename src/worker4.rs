use std::collections::hash_map;

use fxhash::FxHashMap;

use crate::common::*;

#[derive(Default)]
pub struct Worker4 {
    instrument_offsets: FxHashMap<InstrumentId, usize>,
    greeks: Vec<Greeks>,
}

impl Worker for Worker4 {
    fn update(
        &mut self,
        instrument_id: InstrumentId,
        delta: Delta,
        gamma: Gamma,
        theta: Theta,
        vega: Vega,
    ) {
        let greeks = Greeks {
            delta,
            gamma,
            theta,
            vega,
        };
        match self.instrument_offsets.entry(instrument_id) {
            hash_map::Entry::Occupied(occupied_entry) => {
                let offset = *occupied_entry.get();
                self.greeks[offset] = greeks;
            }
            hash_map::Entry::Vacant(vacant_entry) => {
                let offset = self.greeks.len();
                vacant_entry.insert(offset);
                self.greeks.push(greeks);
            }
        }
    }

    fn total_delta(&self) -> Delta {
        self.greeks.iter().map(|x| x.delta).sum()
    }

    fn total_gamma(&self) -> Gamma {
        self.greeks.iter().map(|x| x.gamma).sum()
    }

    fn total_vega(&self) -> Vega {
        self.greeks.iter().map(|x| x.vega).sum()
    }

    fn total_theta(&self) -> Theta {
        self.greeks.iter().map(|x| x.theta).sum()
    }

    fn total_greeks(&self) -> Greeks {
        let mut total = Greeks {
            delta: Delta(0.0),
            gamma: Gamma(0.0),
            theta: Theta(0.0),
            vega: Vega(0.0),
        };
        for entry in &self.greeks {
            total.delta += entry.delta;
            total.gamma += entry.gamma;
            total.theta += entry.theta;
            total.vega += entry.vega;
        }
        total
    }
}
