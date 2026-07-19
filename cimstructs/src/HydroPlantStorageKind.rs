/// The type of hydro power plant.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum HydroPlantStorageKind {
    /// Run of river.
    runOfRiver,
    /// Pumped storage.
    pumpedStorage,
    /// Storage.
    storage,
}

impl HydroPlantStorageKind {
    pub fn uri(&self) -> &'static str {
        match self {
            HydroPlantStorageKind::runOfRiver => "http://iec.ch/TC57/CIM100#HydroPlantStorageKind.runOfRiver",
            HydroPlantStorageKind::pumpedStorage => "http://iec.ch/TC57/CIM100#HydroPlantStorageKind.pumpedStorage",
            HydroPlantStorageKind::storage => "http://iec.ch/TC57/CIM100#HydroPlantStorageKind.storage",
        }
    }
}
