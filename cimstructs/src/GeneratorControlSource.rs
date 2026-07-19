/// The source of controls for a generating unit.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum GeneratorControlSource {
    /// Not available.
    unavailable,
    /// Off of automatic generation control (AGC).
    offAGC,
    /// On automatic generation control (AGC).
    onAGC,
    /// Plant is controlling.
    plantControl,
}

impl GeneratorControlSource {
    pub fn uri(&self) -> &'static str {
        match self {
            GeneratorControlSource::unavailable => "http://iec.ch/TC57/CIM100#GeneratorControlSource.unavailable",
            GeneratorControlSource::offAGC => "http://iec.ch/TC57/CIM100#GeneratorControlSource.offAGC",
            GeneratorControlSource::onAGC => "http://iec.ch/TC57/CIM100#GeneratorControlSource.onAGC",
            GeneratorControlSource::plantControl => "http://iec.ch/TC57/CIM100#GeneratorControlSource.plantControl",
        }
    }
}
