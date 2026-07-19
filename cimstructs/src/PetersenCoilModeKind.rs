/// The mode of operation for a Petersen coil.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PetersenCoilModeKind {
    /// Fixed position.
    fixed,
    /// Manual positioning.
    manual,
    /// Automatic positioning.
    automaticPositioning,
}

impl PetersenCoilModeKind {
    pub fn uri(&self) -> &'static str {
        match self {
            PetersenCoilModeKind::fixed => "http://iec.ch/TC57/CIM100#PetersenCoilModeKind.fixed",
            PetersenCoilModeKind::manual => "http://iec.ch/TC57/CIM100#PetersenCoilModeKind.manual",
            PetersenCoilModeKind::automaticPositioning => "http://iec.ch/TC57/CIM100#PetersenCoilModeKind.automaticPositioning",
        }
    }
}
