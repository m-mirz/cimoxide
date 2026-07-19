/// Governor control flag for Francis hydro model.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum FrancisGovernorControlKind {
    /// Mechanic-hydraulic regulator with tacho-accelerometer (Cflag = 1).
    mechanicHydrolicTachoAccelerator,
    /// Mechanic-hydraulic regulator with transient feedback (Cflag=2).
    mechanicHydraulicTransientFeedback,
    /// Electromechanical and electrohydraulic regulator (Cflag=3).
    electromechanicalElectrohydraulic,
}

impl FrancisGovernorControlKind {
    pub fn uri(&self) -> &'static str {
        match self {
            FrancisGovernorControlKind::mechanicHydrolicTachoAccelerator => "http://iec.ch/TC57/CIM100#FrancisGovernorControlKind.mechanicHydrolicTachoAccelerator",
            FrancisGovernorControlKind::mechanicHydraulicTransientFeedback => "http://iec.ch/TC57/CIM100#FrancisGovernorControlKind.mechanicHydraulicTransientFeedback",
            FrancisGovernorControlKind::electromechanicalElectrohydraulic => "http://iec.ch/TC57/CIM100#FrancisGovernorControlKind.electromechanicalElectrohydraulic",
        }
    }
}
