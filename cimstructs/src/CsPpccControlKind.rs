/// Active power control modes for HVDC line operating as Current Source Converter.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CsPpccControlKind {
    /// Control is active power control at AC side, at point of common coupling. Target is provided by ACDCConverter.targetPpcc.
    activePower,
    /// Control is DC voltage with target value provided by ACDCConverter.targetUdc.
    dcVoltage,
    /// Control is DC current with target value provided by CsConverter.targetIdc.
    dcCurrent,
}

impl CsPpccControlKind {
    pub fn uri(&self) -> &'static str {
        match self {
            CsPpccControlKind::activePower => "http://iec.ch/TC57/CIM100#CsPpccControlKind.activePower",
            CsPpccControlKind::dcVoltage => "http://iec.ch/TC57/CIM100#CsPpccControlKind.dcVoltage",
            CsPpccControlKind::dcCurrent => "http://iec.ch/TC57/CIM100#CsPpccControlKind.dcCurrent",
        }
    }
}
