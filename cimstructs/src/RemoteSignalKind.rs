/// Type of input signal coming from remote bus.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum RemoteSignalKind {
    /// Input is voltage frequency from remote terminal bus.
    remoteBusVoltageFrequency,
    /// Input is voltage frequency deviation from remote terminal bus.
    remoteBusVoltageFrequencyDeviation,
    /// Input is frequency from remote terminal bus.
    remoteBusFrequency,
    /// Input is frequency deviation from remote terminal bus.
    remoteBusFrequencyDeviation,
    /// Input is voltage amplitude from remote terminal bus.
    remoteBusVoltageAmplitude,
    /// Input is voltage from remote terminal bus.
    remoteBusVoltage,
    /// Input is branch current amplitude from remote terminal bus.
    remoteBranchCurrentAmplitude,
    /// Input is branch current amplitude derivative from remote terminal bus.
    remoteBusVoltageAmplitudeDerivative,
    /// Input is PU voltage derivative from remote terminal bus.
    remotePuBusVoltageDerivative,
}

impl RemoteSignalKind {
    pub fn uri(&self) -> &'static str {
        match self {
            RemoteSignalKind::remoteBusVoltageFrequency => "http://iec.ch/TC57/CIM100#RemoteSignalKind.remoteBusVoltageFrequency",
            RemoteSignalKind::remoteBusVoltageFrequencyDeviation => "http://iec.ch/TC57/CIM100#RemoteSignalKind.remoteBusVoltageFrequencyDeviation",
            RemoteSignalKind::remoteBusFrequency => "http://iec.ch/TC57/CIM100#RemoteSignalKind.remoteBusFrequency",
            RemoteSignalKind::remoteBusFrequencyDeviation => "http://iec.ch/TC57/CIM100#RemoteSignalKind.remoteBusFrequencyDeviation",
            RemoteSignalKind::remoteBusVoltageAmplitude => "http://iec.ch/TC57/CIM100#RemoteSignalKind.remoteBusVoltageAmplitude",
            RemoteSignalKind::remoteBusVoltage => "http://iec.ch/TC57/CIM100#RemoteSignalKind.remoteBusVoltage",
            RemoteSignalKind::remoteBranchCurrentAmplitude => "http://iec.ch/TC57/CIM100#RemoteSignalKind.remoteBranchCurrentAmplitude",
            RemoteSignalKind::remoteBusVoltageAmplitudeDerivative => "http://iec.ch/TC57/CIM100#RemoteSignalKind.remoteBusVoltageAmplitudeDerivative",
            RemoteSignalKind::remotePuBusVoltageDerivative => "http://iec.ch/TC57/CIM100#RemoteSignalKind.remotePuBusVoltageDerivative",
        }
    }
}
