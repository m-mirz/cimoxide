/// Synchronous machine type.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SynchronousMachineKind {
    /// Indicates the synchronous machine can operate as a generator.
    generator,
    /// Indicates the synchronous machine can operate as a condenser.
    condenser,
    /// Indicates the synchronous machine can operate as a generator or as a condenser.
    generatorOrCondenser,
    /// Indicates the synchronous machine can operate as a motor.
    motor,
    /// Indicates the synchronous machine can operate as a generator or as a motor.
    generatorOrMotor,
    /// Indicates the synchronous machine can operate as a motor or as a condenser.
    motorOrCondenser,
    /// Indicates the synchronous machine can operate as a generator or as a condenser or as a motor.
    generatorOrCondenserOrMotor,
}

impl SynchronousMachineKind {
    pub fn uri(&self) -> &'static str {
        match self {
            SynchronousMachineKind::generator => "http://iec.ch/TC57/CIM100#SynchronousMachineKind.generator",
            SynchronousMachineKind::condenser => "http://iec.ch/TC57/CIM100#SynchronousMachineKind.condenser",
            SynchronousMachineKind::generatorOrCondenser => "http://iec.ch/TC57/CIM100#SynchronousMachineKind.generatorOrCondenser",
            SynchronousMachineKind::motor => "http://iec.ch/TC57/CIM100#SynchronousMachineKind.motor",
            SynchronousMachineKind::generatorOrMotor => "http://iec.ch/TC57/CIM100#SynchronousMachineKind.generatorOrMotor",
            SynchronousMachineKind::motorOrCondenser => "http://iec.ch/TC57/CIM100#SynchronousMachineKind.motorOrCondenser",
            SynchronousMachineKind::generatorOrCondenserOrMotor => "http://iec.ch/TC57/CIM100#SynchronousMachineKind.generatorOrCondenserOrMotor",
        }
    }
}
