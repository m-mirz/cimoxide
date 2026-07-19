/// Type of rotor, used by short circuit applications.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ShortCircuitRotorKind {
    /// Salient pole 1 in IEC 60909.
    salientPole1,
    /// Salient pole 2 in IEC 60909.
    salientPole2,
    /// Turbo Series 1 in IEC 60909.
    turboSeries1,
    /// Turbo series 2 in IEC 60909.
    turboSeries2,
}

impl ShortCircuitRotorKind {
    pub fn uri(&self) -> &'static str {
        match self {
            ShortCircuitRotorKind::salientPole1 => "http://iec.ch/TC57/CIM100#ShortCircuitRotorKind.salientPole1",
            ShortCircuitRotorKind::salientPole2 => "http://iec.ch/TC57/CIM100#ShortCircuitRotorKind.salientPole2",
            ShortCircuitRotorKind::turboSeries1 => "http://iec.ch/TC57/CIM100#ShortCircuitRotorKind.turboSeries1",
            ShortCircuitRotorKind::turboSeries2 => "http://iec.ch/TC57/CIM100#ShortCircuitRotorKind.turboSeries2",
        }
    }
}
