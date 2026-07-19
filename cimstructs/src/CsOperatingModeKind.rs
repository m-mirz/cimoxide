/// Operating mode for HVDC line operating as Current Source Converter.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CsOperatingModeKind {
    /// Operating as inverter, which is the power receiving end.
    inverter,
    /// Operating as rectifier, which is the power sending end.
    rectifier,
}

impl CsOperatingModeKind {
    pub fn uri(&self) -> &'static str {
        match self {
            CsOperatingModeKind::inverter => "http://iec.ch/TC57/CIM100#CsOperatingModeKind.inverter",
            CsOperatingModeKind::rectifier => "http://iec.ch/TC57/CIM100#CsOperatingModeKind.rectifier",
        }
    }
}
