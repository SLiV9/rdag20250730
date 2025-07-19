use derive_more::{Add, AddAssign, Sum};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InstrumentId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Add, AddAssign, Sum)]
pub struct DeltaExposure(pub f32);

#[derive(Clone, Copy, Debug, PartialEq, Add, AddAssign, Sum)]
pub struct GammaExposure(pub f32);

#[derive(Clone, Copy, Debug, PartialEq, Add, AddAssign, Sum)]
pub struct ThetaExposure(pub f32);

#[derive(Clone, Copy, Debug, PartialEq, Add, AddAssign, Sum)]
pub struct VegaExposure(pub f32);

pub struct Exposures {
    pub delta_exposure: DeltaExposure,
    pub gamma_exposure: GammaExposure,
    pub theta_exposure: ThetaExposure,
    pub vega_exposure: VegaExposure,
}

pub trait Worker: Default {
    fn update_exposure(
        &mut self,
        instrument_id: InstrumentId,
        delta_exposure: DeltaExposure,
        gamma_exposure: GammaExposure,
        theta_exposure: ThetaExposure,
        vega_exposure: VegaExposure,
    );

    fn total_delta_exposure(&self) -> DeltaExposure;
    fn total_gamma_exposure(&self) -> GammaExposure;
    fn total_theta_exposure(&self) -> ThetaExposure;
    fn total_vega_exposure(&self) -> VegaExposure;
    fn total_exposures(&self) -> Exposures;
}
