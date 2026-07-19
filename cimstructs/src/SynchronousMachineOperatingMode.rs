/// Synchronous machine operating mode.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SynchronousMachineOperatingMode {
    /// Operating as generator.
    generator,
    /// Operating as condenser.
    condenser,
    /// Operating as motor.
    motor,
}

impl SynchronousMachineOperatingMode {
    pub fn uri(&self) -> &'static str {
        match self {
            SynchronousMachineOperatingMode::generator => "http://iec.ch/TC57/CIM100#SynchronousMachineOperatingMode.generator",
            SynchronousMachineOperatingMode::condenser => "http://iec.ch/TC57/CIM100#SynchronousMachineOperatingMode.condenser",
            SynchronousMachineOperatingMode::motor => "http://iec.ch/TC57/CIM100#SynchronousMachineOperatingMode.motor",
        }
    }
}
