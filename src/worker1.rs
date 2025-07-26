use crate::common::*;

pub struct InstrumentData {
    id: Id,
    greeks: Greeks,
}

#[derive(Default)]
pub struct NaiveWorker {
    greeks: Vec<InstrumentData>,
}

impl NaiveWorker {
    pub fn update_separate(
        &mut self,
        id: Id,
        delta: f32,
        gamma: f32,
        theta: f32,
        vega: f32,
    ) {
        let greeks = Greeks {
            delta: Delta(delta),
            gamma: Gamma(gamma),
            theta: Theta(theta),
            vega: Vega(vega),
        };
        if let Some(entry) =
            self.greeks.iter_mut().find(|x| x.id == id)
        {
            entry.greeks = greeks;
        } else {
            self.greeks.push(InstrumentData { id, greeks });
        }
    }
}

impl Worker for NaiveWorker {
    fn update(&mut self, id: Id, greeks: Greeks) {
        if let Some(entry) =
            self.greeks.iter_mut().find(|x| x.id == id)
        {
            entry.greeks = greeks;
        } else {
            self.greeks.push(InstrumentData { id, greeks });
        }
    }

    fn total_delta(&self) -> Delta {
        self.greeks.iter().map(|x| x.greeks.delta).sum()
    }

    fn total_gamma(&self) -> Gamma {
        self.greeks.iter().map(|x| x.greeks.gamma).sum()
    }

    fn total_theta(&self) -> Theta {
        self.greeks.iter().map(|x| x.greeks.theta).sum()
    }

    fn total_vega(&self) -> Vega {
        self.greeks.iter().map(|x| x.greeks.vega).sum()
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
