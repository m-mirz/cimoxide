/// Type of turbine.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum HydroTurbineKind {
    /// Francis.
    francis,
    /// Pelton.
    pelton,
    /// Kaplan.
    kaplan,
}

impl HydroTurbineKind {
    pub fn uri(&self) -> &'static str {
        match self {
            HydroTurbineKind::francis => "http://iec.ch/TC57/CIM100#HydroTurbineKind.francis",
            HydroTurbineKind::pelton => "http://iec.ch/TC57/CIM100#HydroTurbineKind.pelton",
            HydroTurbineKind::kaplan => "http://iec.ch/TC57/CIM100#HydroTurbineKind.kaplan",
        }
    }
}
