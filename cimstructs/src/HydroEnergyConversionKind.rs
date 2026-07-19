/// Specifies the capability of the hydro generating unit to convert energy as a generator or pump.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum HydroEnergyConversionKind {
    /// Able to generate power, but not able to pump water for energy storage.
    generator,
    /// Able to both generate power and pump water for energy storage.
    pumpAndGenerator,
}

impl HydroEnergyConversionKind {
    pub fn uri(&self) -> &'static str {
        match self {
            HydroEnergyConversionKind::generator => "http://iec.ch/TC57/CIM100#HydroEnergyConversionKind.generator",
            HydroEnergyConversionKind::pumpAndGenerator => "http://iec.ch/TC57/CIM100#HydroEnergyConversionKind.pumpAndGenerator",
        }
    }
}
