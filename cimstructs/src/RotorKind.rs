/// Type of rotor on physical machine.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum RotorKind {
    /// Round rotor type of synchronous machine.
    roundRotor,
    /// Salient pole type of synchronous machine.
    salientPole,
}

impl RotorKind {
    pub fn uri(&self) -> &'static str {
        match self {
            RotorKind::roundRotor => "http://iec.ch/TC57/CIM100#RotorKind.roundRotor",
            RotorKind::salientPole => "http://iec.ch/TC57/CIM100#RotorKind.salientPole",
        }
    }
}
