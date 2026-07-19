/// Kind of Asynchronous Machine.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AsynchronousMachineKind {
    /// The Asynchronous Machine is a generator.
    generator,
    /// The Asynchronous Machine is a motor.
    motor,
}

impl AsynchronousMachineKind {
    pub fn uri(&self) -> &'static str {
        match self {
            AsynchronousMachineKind::generator => "http://iec.ch/TC57/CIM100#AsynchronousMachineKind.generator",
            AsynchronousMachineKind::motor => "http://iec.ch/TC57/CIM100#AsynchronousMachineKind.motor",
        }
    }
}
