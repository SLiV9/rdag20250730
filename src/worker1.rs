


use crate::common::*;

pub struct InstrumentExposure {
    instrument_id: InstrumentId,
    delta_exposure: DeltaExposure,
    gamma_exposure: GammaExposure,
    vega_exposure: VegaExposure,
    theta_exposure: ThetaExposure,
    charm_exposure: CharmExposure,
}

#[derive(Default)]
pub struct Worker1 {
    exposures: Vec<InstrumentExposure>,
}

impl Worker for Worker1 {
    fn update_exposure(
        &mut self,
        instrument_id: InstrumentId,
        delta_exposure: DeltaExposure,
        gamma_exposure: GammaExposure,
        vega_exposure: VegaExposure,
        theta_exposure: ThetaExposure,
        charm_exposure: CharmExposure,
    ) {
        let entry = self
            .exposures
            .iter_mut()
            .find(|x| x.instrument_id == instrument_id);
        if let Some(entry) = entry {
            entry.delta_exposure = delta_exposure;
            entry.gamma_exposure = gamma_exposure;
            entry.vega_exposure = vega_exposure;
            entry.theta_exposure = theta_exposure;
            entry.charm_exposure = charm_exposure;
        } else {
            self.exposures.push(InstrumentExposure {
                instrument_id,
                delta_exposure,
                gamma_exposure,
                vega_exposure,
                theta_exposure,
                charm_exposure,
            })
        }
    }

    fn total_delta_exposure(&self) -> DeltaExposure {
        self.exposures.iter().map(|x| x.delta_exposure).sum()
    }

    fn total_gamma_exposure(&self) -> GammaExposure {
        self.exposures.iter().map(|x| x.gamma_exposure).sum()
    }

    fn total_vega_exposure(&self) -> VegaExposure {
        self.exposures.iter().map(|x| x.vega_exposure).sum()
    }

    fn total_theta_exposure(&self) -> ThetaExposure {
        self.exposures.iter().map(|x| x.theta_exposure).sum()
    }

    fn total_charm_exposure(&self) -> CharmExposure {
        self.exposures.iter().map(|x| x.charm_exposure).sum()
    }

    fn total_exposures(&self) -> Exposures {
        Exposures {
            delta_exposure: self.total_delta_exposure(),
            gamma_exposure: self.total_gamma_exposure(),
            vega_exposure: self.total_vega_exposure(),
            theta_exposure: self.total_theta_exposure(),
            charm_exposure: self.total_charm_exposure(),
        }
    }
}
