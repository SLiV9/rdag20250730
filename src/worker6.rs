use std::collections::hash_map;
use std::iter::Sum;
use std::ops::Add;
use std::ops::AddAssign;

use fxhash::FxHashMap;

use crate::common::*;

#[derive(Default)]
pub struct Worker6 {
    instrument_offsets: FxHashMap<InstrumentId, usize>,
    deltas: Vec<Delta>,
    gammas: Vec<Gamma>,
    thetas: Vec<Theta>,
    vegas: Vec<Vega>,
}

impl Worker for Worker6 {
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
        fast_sum(Delta(0.0), &self.deltas[..])
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

fn fast_sum<T>(zero_value: T, mut values: &[T]) -> T
where
    T: Copy + Add<Output = T> + AddAssign + Sum,
{
    const CHUNK_SIZE: usize = 128;
    let mut sums = [zero_value; CHUNK_SIZE];
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
