/// Winding connection type.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum WindingConnection {
    /// Delta.
    D,
    /// Wye.
    Y,
    /// ZigZag.
    Z,
    /// Wye, with neutral brought out for grounding.
    Yn,
    /// ZigZag, with neutral brought out for grounding.
    Zn,
    /// Autotransformer common winding.
    A,
    /// Independent winding, for single-phase connections.
    I,
}

impl WindingConnection {
    pub fn uri(&self) -> &'static str {
        match self {
            WindingConnection::D => "http://iec.ch/TC57/CIM100#WindingConnection.D",
            WindingConnection::Y => "http://iec.ch/TC57/CIM100#WindingConnection.Y",
            WindingConnection::Z => "http://iec.ch/TC57/CIM100#WindingConnection.Z",
            WindingConnection::Yn => "http://iec.ch/TC57/CIM100#WindingConnection.Yn",
            WindingConnection::Zn => "http://iec.ch/TC57/CIM100#WindingConnection.Zn",
            WindingConnection::A => "http://iec.ch/TC57/CIM100#WindingConnection.A",
            WindingConnection::I => "http://iec.ch/TC57/CIM100#WindingConnection.I",
        }
    }
}
