/// The kind of regulation model. For example regulating voltage, reactive power, active power, etc.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum RegulatingControlModeKind {
    /// Voltage is specified.
    voltage,
    /// Active power is specified.
    activePower,
    /// Reactive power is specified.
    reactivePower,
    /// Current flow is specified.
    currentFlow,
    /// Admittance is specified.
    admittance,
    /// Control switches on/off by time of day. The times may change on the weekend, or in different seasons.
    timeScheduled,
    /// Control switches on/off based on the local temperature (i.e., a thermostat).
    temperature,
    /// Power factor is specified.
    powerFactor,
}

impl RegulatingControlModeKind {
    pub fn uri(&self) -> &'static str {
        match self {
            RegulatingControlModeKind::voltage => "http://iec.ch/TC57/CIM100#RegulatingControlModeKind.voltage",
            RegulatingControlModeKind::activePower => "http://iec.ch/TC57/CIM100#RegulatingControlModeKind.activePower",
            RegulatingControlModeKind::reactivePower => "http://iec.ch/TC57/CIM100#RegulatingControlModeKind.reactivePower",
            RegulatingControlModeKind::currentFlow => "http://iec.ch/TC57/CIM100#RegulatingControlModeKind.currentFlow",
            RegulatingControlModeKind::admittance => "http://iec.ch/TC57/CIM100#RegulatingControlModeKind.admittance",
            RegulatingControlModeKind::timeScheduled => "http://iec.ch/TC57/CIM100#RegulatingControlModeKind.timeScheduled",
            RegulatingControlModeKind::temperature => "http://iec.ch/TC57/CIM100#RegulatingControlModeKind.temperature",
            RegulatingControlModeKind::powerFactor => "http://iec.ch/TC57/CIM100#RegulatingControlModeKind.powerFactor",
        }
    }
}
