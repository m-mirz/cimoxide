/// The operating mode of an HVDC bipole.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DCConverterOperatingModeKind {
    /// Bipolar operation.
    bipolar,
    /// Monopolar operation with metallic return.
    monopolarMetallicReturn,
    /// Monopolar operation with ground return.
    monopolarGroundReturn,
}

impl DCConverterOperatingModeKind {
    pub fn uri(&self) -> &'static str {
        match self {
            DCConverterOperatingModeKind::bipolar => "http://iec.ch/TC57/CIM100#DCConverterOperatingModeKind.bipolar",
            DCConverterOperatingModeKind::monopolarMetallicReturn => "http://iec.ch/TC57/CIM100#DCConverterOperatingModeKind.monopolarMetallicReturn",
            DCConverterOperatingModeKind::monopolarGroundReturn => "http://iec.ch/TC57/CIM100#DCConverterOperatingModeKind.monopolarGroundReturn",
        }
    }
}
