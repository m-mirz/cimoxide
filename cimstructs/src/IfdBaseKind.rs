/// Excitation base system mode.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum IfdBaseKind {
    /// Air gap line mode.
    ifag,
    /// No load system with saturation mode.
    ifnl,
    /// Full load system mode.
    iffl,
}

impl IfdBaseKind {
    pub fn uri(&self) -> &'static str {
        match self {
            IfdBaseKind::ifag => "http://iec.ch/TC57/CIM100#IfdBaseKind.ifag",
            IfdBaseKind::ifnl => "http://iec.ch/TC57/CIM100#IfdBaseKind.ifnl",
            IfdBaseKind::iffl => "http://iec.ch/TC57/CIM100#IfdBaseKind.iffl",
        }
    }
}
