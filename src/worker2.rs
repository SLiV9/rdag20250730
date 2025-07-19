use std::collections::HashMap;

use crate::common::*;

#[derive(Default)]
pub struct Worker2 {
    greeks: HashMap<InstrumentId, Greeks>,
}

impl Worker for Worker2 {
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
        self.greeks.insert(instrument_id, greeks);
    }

    fn total_delta(&self) -> Delta {
        self.greeks.values().map(|x| x.delta).sum()
    }

    fn total_gamma(&self) -> Gamma {
        self.greeks.values().map(|x| x.gamma).sum()
    }

    fn total_vega(&self) -> Vega {
        self.greeks.values().map(|x| x.vega).sum()
    }

    fn total_theta(&self) -> Theta {
        self.greeks.values().map(|x| x.theta).sum()
    }

    fn total_greeks(&self) -> Greeks {
        let mut total = Greeks {
            delta: Delta(0.0),
            gamma: Gamma(0.0),
            theta: Theta(0.0),
            vega: Vega(0.0),
        };
        for entry in self.greeks.values() {
            total.delta += entry.delta;
            total.gamma += entry.gamma;
            total.theta += entry.theta;
            total.vega += entry.vega;
        }
        total
    }
}
