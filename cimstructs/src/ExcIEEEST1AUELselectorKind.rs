/// Types of connections for the UEL input used in ExcIEEEST1A.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ExcIEEEST1AUELselectorKind {
    /// Ignore UEL signal.
    ignoreUELsignal,
    /// UEL input HV gate with voltage regulator output.
    inputHVgateVoltageOutput,
    /// UEL input HV gate with error signal.
    inputHVgateErrorSignal,
    /// UEL input added to error signal.
    inputAddedToErrorSignal,
}

impl ExcIEEEST1AUELselectorKind {
    pub fn uri(&self) -> &'static str {
        match self {
            ExcIEEEST1AUELselectorKind::ignoreUELsignal => "http://iec.ch/TC57/CIM100#ExcIEEEST1AUELselectorKind.ignoreUELsignal",
            ExcIEEEST1AUELselectorKind::inputHVgateVoltageOutput => "http://iec.ch/TC57/CIM100#ExcIEEEST1AUELselectorKind.inputHVgateVoltageOutput",
            ExcIEEEST1AUELselectorKind::inputHVgateErrorSignal => "http://iec.ch/TC57/CIM100#ExcIEEEST1AUELselectorKind.inputHVgateErrorSignal",
            ExcIEEEST1AUELselectorKind::inputAddedToErrorSignal => "http://iec.ch/TC57/CIM100#ExcIEEEST1AUELselectorKind.inputAddedToErrorSignal",
        }
    }
}
