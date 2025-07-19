use derive_more::{Add, AddAssign, Sum};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InstrumentId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Add, AddAssign, Sum)]
pub struct Delta(pub f32);

#[derive(Clone, Copy, Debug, PartialEq, Add, AddAssign, Sum)]
pub struct Gamma(pub f32);

#[derive(Clone, Copy, Debug, PartialEq, Add, AddAssign, Sum)]
pub struct Theta(pub f32);

#[derive(Clone, Copy, Debug, PartialEq, Add, AddAssign, Sum)]
pub struct Vega(pub f32);

pub struct Greeks {
    pub delta: Delta,
    pub gamma: Gamma,
    pub theta: Theta,
    pub vega: Vega,
}

pub trait Worker: Default {
    fn update(
        &mut self,
        instrument_id: InstrumentId,
        delta: Delta,
        gamma: Gamma,
        theta: Theta,
        vega: Vega,
    );

    fn total_delta(&self) -> Delta;
    fn total_gamma(&self) -> Gamma;
    fn total_theta(&self) -> Theta;
    fn total_vega(&self) -> Vega;
    fn total_greeks(&self) -> Greeks;
}
