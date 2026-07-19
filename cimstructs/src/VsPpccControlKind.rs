/// Types applicable to the control of real power and/or DC voltage by voltage source converter.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum VsPpccControlKind {
    /// Control is real power at point of common coupling. The target value is provided by ACDCConverter.targetPpcc.
    pPcc,
    /// Control is DC voltage with target value provided by ACDCConverter.targetUdc.
    udc,
    /// Control is active power at point of common coupling and local DC voltage, with the droop. Target values are provided by ACDCConverter.targetPpcc, ACDCConverter.targetUdc and VsConverter.droop.
    pPccAndUdcDroop,
    /// Control is active power at point of common coupling and compensated DC voltage, with the droop. Compensation factor is the resistance, as an approximation of the DC voltage of a common (real or virtual) node in the DC network. Targets are provided by ACDCConverter.targetPpcc, ACDCConverter.targetUdc, VsConverter.droop and VsConverter.droopCompensation.
    pPccAndUdcDroopWithCompensation,
    /// Control is active power at point of common coupling and the pilot DC voltage, with the droop. The mode is used for Multi Terminal High Voltage DC (MTDC) systems where multiple HVDC Substations are connected to the HVDC transmission lines. The pilot voltage is then used to coordinate the control the DC voltage across the HVDC substations. Targets are provided by ACDCConverter.targetPpcc, ACDCConverter.targetUdc and VsConverter.droop.
    pPccAndUdcDroopPilot,
    /// Control is phase at point of common coupling. Target is provided by VsConverter.targetPhasePcc.
    phasePcc,
}

impl VsPpccControlKind {
    pub fn uri(&self) -> &'static str {
        match self {
            VsPpccControlKind::pPcc => "http://iec.ch/TC57/CIM100#VsPpccControlKind.pPcc",
            VsPpccControlKind::udc => "http://iec.ch/TC57/CIM100#VsPpccControlKind.udc",
            VsPpccControlKind::pPccAndUdcDroop => "http://iec.ch/TC57/CIM100#VsPpccControlKind.pPccAndUdcDroop",
            VsPpccControlKind::pPccAndUdcDroopWithCompensation => "http://iec.ch/TC57/CIM100#VsPpccControlKind.pPccAndUdcDroopWithCompensation",
            VsPpccControlKind::pPccAndUdcDroopPilot => "http://iec.ch/TC57/CIM100#VsPpccControlKind.pPccAndUdcDroopPilot",
            VsPpccControlKind::phasePcc => "http://iec.ch/TC57/CIM100#VsPpccControlKind.phasePcc",
        }
    }
}
