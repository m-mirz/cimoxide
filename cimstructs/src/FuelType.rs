/// Type of fuel.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum FuelType {
    /// Generic coal, not including lignite type.
    coal,
    /// Oil.
    oil,
    /// Natural gas.
    gas,
    /// The fuel is lignite coal. Note that this is a special type of coal, so the other enum of coal is reserved for hard coal types or if the exact type of coal is not known.
    lignite,
    /// Hard coal.
    hardCoal,
    /// Oil Shale.
    oilShale,
    /// Brown coal lignite.
    brownCoalLignite,
    /// Coal derived gas.
    coalDerivedGas,
    /// Peat.
    peat,
    /// Any fuel type not included in the rest of the enumerated value.
    other,
}

impl FuelType {
    pub fn uri(&self) -> &'static str {
        match self {
            FuelType::coal => "http://iec.ch/TC57/CIM100#FuelType.coal",
            FuelType::oil => "http://iec.ch/TC57/CIM100#FuelType.oil",
            FuelType::gas => "http://iec.ch/TC57/CIM100#FuelType.gas",
            FuelType::lignite => "http://iec.ch/TC57/CIM100#FuelType.lignite",
            FuelType::hardCoal => "http://iec.ch/TC57/CIM100#FuelType.hardCoal",
            FuelType::oilShale => "http://iec.ch/TC57/CIM100#FuelType.oilShale",
            FuelType::brownCoalLignite => "http://iec.ch/TC57/CIM100#FuelType.brownCoalLignite",
            FuelType::coalDerivedGas => "http://iec.ch/TC57/CIM100#FuelType.coalDerivedGas",
            FuelType::peat => "http://iec.ch/TC57/CIM100#FuelType.peat",
            FuelType::other => "http://iec.ch/TC57/CIM100#FuelType.other",
        }
    }
}
