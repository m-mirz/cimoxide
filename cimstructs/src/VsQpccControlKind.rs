/// Kind of reactive power control at point of common coupling for a voltage source converter.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum VsQpccControlKind {
    /// Control is reactive power at point of common coupling. Target is provided by VsConverter.targetQpcc.
    reactivePcc,
    /// Control is voltage at point of common coupling. Target is provided by VsConverter.targetUpcc.
    voltagePcc,
    /// Control is power factor at point of common coupling. Target is provided by VsConverter.targetPowerFactorPcc.
    powerFactorPcc,
    /// No explicit control. Pulse-modulation factor is directly set in magnitude (VsConverter.targetPWMfactor) and phase (VsConverter.targetPhasePcc).
    pulseWidthModulation,
}

impl VsQpccControlKind {
    pub fn uri(&self) -> &'static str {
        match self {
            VsQpccControlKind::reactivePcc => "http://iec.ch/TC57/CIM100#VsQpccControlKind.reactivePcc",
            VsQpccControlKind::voltagePcc => "http://iec.ch/TC57/CIM100#VsQpccControlKind.voltagePcc",
            VsQpccControlKind::powerFactorPcc => "http://iec.ch/TC57/CIM100#VsQpccControlKind.powerFactorPcc",
            VsQpccControlKind::pulseWidthModulation => "http://iec.ch/TC57/CIM100#VsQpccControlKind.pulseWidthModulation",
        }
    }
}
