/// Kind of wind generating unit.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum WindGenUnitKind {
    /// The wind generating unit is located offshore.
    offshore,
    /// The wind generating unit is located onshore.
    onshore,
}

impl WindGenUnitKind {
    pub fn uri(&self) -> &'static str {
        match self {
            WindGenUnitKind::offshore => "http://iec.ch/TC57/CIM100#WindGenUnitKind.offshore",
            WindGenUnitKind::onshore => "http://iec.ch/TC57/CIM100#WindGenUnitKind.onshore",
        }
    }
}
