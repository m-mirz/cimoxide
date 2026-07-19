/// Polarity for DC circuits.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DCPolarityKind {
    /// Positive pole. The converter terminal is intended to operate at a positive voltage relative the midpoint or negative terminal.
    positive,
    /// Middle pole. The converter terminal is the midpoint in a bipolar or symmetric monopole configuration. The midpoint can be grounded and/or have a metallic return.
    middle,
    /// Negative pole. The converter terminal is intended to operate at a negative voltage relative the midpoint or positive terminal.
    negative,
}

impl DCPolarityKind {
    pub fn uri(&self) -> &'static str {
        match self {
            DCPolarityKind::positive => "http://iec.ch/TC57/CIM100#DCPolarityKind.positive",
            DCPolarityKind::middle => "http://iec.ch/TC57/CIM100#DCPolarityKind.middle",
            DCPolarityKind::negative => "http://iec.ch/TC57/CIM100#DCPolarityKind.negative",
        }
    }
}
