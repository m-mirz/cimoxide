/// General wind turbine Q control modes MqG.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum WindQcontrolModeKind {
    /// Voltage control (MqG equals 0).
    voltage,
    /// Reactive power control (MqG equals 1).
    reactivePower,
    /// Open loop reactive power control (only used with closed loop at plant level) (MqG equals 2).
    openLoopReactivePower,
    /// Power factor control (MqG equals 3).
    powerFactor,
    /// Open loop power factor control (MqG equals 4).
    openLooppowerFactor,
}

impl WindQcontrolModeKind {
    pub fn uri(&self) -> &'static str {
        match self {
            WindQcontrolModeKind::voltage => "http://iec.ch/TC57/CIM100#WindQcontrolModeKind.voltage",
            WindQcontrolModeKind::reactivePower => "http://iec.ch/TC57/CIM100#WindQcontrolModeKind.reactivePower",
            WindQcontrolModeKind::openLoopReactivePower => "http://iec.ch/TC57/CIM100#WindQcontrolModeKind.openLoopReactivePower",
            WindQcontrolModeKind::powerFactor => "http://iec.ch/TC57/CIM100#WindQcontrolModeKind.powerFactor",
            WindQcontrolModeKind::openLooppowerFactor => "http://iec.ch/TC57/CIM100#WindQcontrolModeKind.openLooppowerFactor",
        }
    }
}
