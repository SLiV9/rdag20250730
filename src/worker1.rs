use crate::common::*;

pub struct InstrumentExposure {
    instrument_id: InstrumentId,
    delta: Delta,
    gamma: Gamma,
    theta: Theta,
    vega: Vega,
}

#[derive(Default)]
pub struct Worker1 {
    greeks: Vec<InstrumentExposure>,
}

impl Worker for Worker1 {
    fn update(
        &mut self,
        instrument_id: InstrumentId,
        delta: Delta,
        gamma: Gamma,
        theta: Theta,
        vega: Vega,
    ) {
        let mut entries = self.greeks.iter_mut();
        let entry = entries.find(|x| x.instrument_id == instrument_id);
        if let Some(entry) = entry {
            entry.delta = delta;
            entry.gamma = gamma;
            entry.theta = theta;
            entry.vega = vega;
        } else {
            self.greeks.push(InstrumentExposure {
                instrument_id,
                delta,
                gamma,
                theta,
                vega,
            })
        }
    }

    fn total_delta(&self) -> Delta {
        self.greeks.iter().map(|x| x.delta).sum()
    }

    fn total_gamma(&self) -> Gamma {
        self.greeks.iter().map(|x| x.gamma).sum()
    }

    fn total_theta(&self) -> Theta {
        self.greeks.iter().map(|x| x.theta).sum()
    }

    fn total_vega(&self) -> Vega {
        self.greeks.iter().map(|x| x.vega).sum()
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
