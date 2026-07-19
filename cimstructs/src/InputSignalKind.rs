/// Types of input signals. In dynamics modelling, commonly represented by the j parameter.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum InputSignalKind {
    /// Input signal is rotor or shaft speed (angular frequency).
    rotorSpeed,
    /// Input signal is rotor or shaft angular frequency deviation.
    rotorAngularFrequencyDeviation,
    /// Input signal is bus voltage frequency. This could be a terminal frequency or remote frequency.
    busFrequency,
    /// Input signal is deviation of bus voltage frequency. This could be a terminal frequency deviation or remote frequency deviation.
    busFrequencyDeviation,
    /// Input signal is generator electrical power on rated S.
    generatorElectricalPower,
    /// Input signal is generator accelerating power.
    generatorAcceleratingPower,
    /// Input signal is bus voltage. This could be a terminal voltage or remote voltage.
    busVoltage,
    /// Input signal is derivative of bus voltage. This could be a terminal voltage derivative or remote voltage derivative.
    busVoltageDerivative,
    /// Input signal is amplitude of remote branch current.
    branchCurrent,
    /// Input signal is generator field current.
    fieldCurrent,
    /// Input signal is generator mechanical power.
    generatorMechanicalPower,
}

impl InputSignalKind {
    pub fn uri(&self) -> &'static str {
        match self {
            InputSignalKind::rotorSpeed => "http://iec.ch/TC57/CIM100#InputSignalKind.rotorSpeed",
            InputSignalKind::rotorAngularFrequencyDeviation => "http://iec.ch/TC57/CIM100#InputSignalKind.rotorAngularFrequencyDeviation",
            InputSignalKind::busFrequency => "http://iec.ch/TC57/CIM100#InputSignalKind.busFrequency",
            InputSignalKind::busFrequencyDeviation => "http://iec.ch/TC57/CIM100#InputSignalKind.busFrequencyDeviation",
            InputSignalKind::generatorElectricalPower => "http://iec.ch/TC57/CIM100#InputSignalKind.generatorElectricalPower",
            InputSignalKind::generatorAcceleratingPower => "http://iec.ch/TC57/CIM100#InputSignalKind.generatorAcceleratingPower",
            InputSignalKind::busVoltage => "http://iec.ch/TC57/CIM100#InputSignalKind.busVoltage",
            InputSignalKind::busVoltageDerivative => "http://iec.ch/TC57/CIM100#InputSignalKind.busVoltageDerivative",
            InputSignalKind::branchCurrent => "http://iec.ch/TC57/CIM100#InputSignalKind.branchCurrent",
            InputSignalKind::fieldCurrent => "http://iec.ch/TC57/CIM100#InputSignalKind.fieldCurrent",
            InputSignalKind::generatorMechanicalPower => "http://iec.ch/TC57/CIM100#InputSignalKind.generatorMechanicalPower",
        }
    }
}
