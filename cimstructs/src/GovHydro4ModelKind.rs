/// Possible types of GovHydro4 models.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum GovHydro4ModelKind {
    /// Simple model.
    simple,
    /// Francis or Pelton model.
    francisPelton,
    /// Kaplan model.
    kaplan,
}

impl GovHydro4ModelKind {
    pub fn uri(&self) -> &'static str {
        match self {
            GovHydro4ModelKind::simple => "http://iec.ch/TC57/CIM100#GovHydro4ModelKind.simple",
            GovHydro4ModelKind::francisPelton => "http://iec.ch/TC57/CIM100#GovHydro4ModelKind.francisPelton",
            GovHydro4ModelKind::kaplan => "http://iec.ch/TC57/CIM100#GovHydro4ModelKind.kaplan",
        }
    }
}
