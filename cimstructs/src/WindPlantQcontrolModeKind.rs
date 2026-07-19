/// Reactive power/voltage controller mode.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum WindPlantQcontrolModeKind {
    /// Reactive power reference.
    reactivePower,
    /// Power factor reference.
    powerFactor,
    /// UQ static.
    uqStatic,
    /// Voltage control.
    voltageControl,
}

impl WindPlantQcontrolModeKind {
    pub fn uri(&self) -> &'static str {
        match self {
            WindPlantQcontrolModeKind::reactivePower => "http://iec.ch/TC57/CIM100#WindPlantQcontrolModeKind.reactivePower",
            WindPlantQcontrolModeKind::powerFactor => "http://iec.ch/TC57/CIM100#WindPlantQcontrolModeKind.powerFactor",
            WindPlantQcontrolModeKind::uqStatic => "http://iec.ch/TC57/CIM100#WindPlantQcontrolModeKind.uqStatic",
            WindPlantQcontrolModeKind::voltageControl => "http://iec.ch/TC57/CIM100#WindPlantQcontrolModeKind.voltageControl",
        }
    }
}
