/// Governor droop signal feedback source.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DroopSignalFeedbackKind {
    /// Electrical power feedback (connection indicated as 1 in the block diagrams of models, e.g. GovCT1, GovCT2).
    electricalPower,
    /// No droop signal feedback, is isochronous governor.
    none,
    /// Fuel valve stroke feedback (true stroke) (connection indicated as 2 in the block diagrams of model, e.g. GovCT1, GovCT2).
    fuelValveStroke,
    /// Governor output feedback (requested stroke) (connection indicated as 3 in the block diagrams of models, e.g. GovCT1, GovCT2).
    governorOutput,
}

impl DroopSignalFeedbackKind {
    pub fn uri(&self) -> &'static str {
        match self {
            DroopSignalFeedbackKind::electricalPower => "http://iec.ch/TC57/CIM100#DroopSignalFeedbackKind.electricalPower",
            DroopSignalFeedbackKind::none => "http://iec.ch/TC57/CIM100#DroopSignalFeedbackKind.none",
            DroopSignalFeedbackKind::fuelValveStroke => "http://iec.ch/TC57/CIM100#DroopSignalFeedbackKind.fuelValveStroke",
            DroopSignalFeedbackKind::governorOutput => "http://iec.ch/TC57/CIM100#DroopSignalFeedbackKind.governorOutput",
        }
    }
}
