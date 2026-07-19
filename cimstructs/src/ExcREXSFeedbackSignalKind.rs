/// Types of rate feedback signals.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ExcREXSFeedbackSignalKind {
    /// The voltage regulator output voltage is used. It is the same as exciter field voltage.
    fieldVoltage,
    /// The exciter field current is used.
    fieldCurrent,
    /// The output voltage of the exciter is used.
    outputVoltage,
}

impl ExcREXSFeedbackSignalKind {
    pub fn uri(&self) -> &'static str {
        match self {
            ExcREXSFeedbackSignalKind::fieldVoltage => "http://iec.ch/TC57/CIM100#ExcREXSFeedbackSignalKind.fieldVoltage",
            ExcREXSFeedbackSignalKind::fieldCurrent => "http://iec.ch/TC57/CIM100#ExcREXSFeedbackSignalKind.fieldCurrent",
            ExcREXSFeedbackSignalKind::outputVoltage => "http://iec.ch/TC57/CIM100#ExcREXSFeedbackSignalKind.outputVoltage",
        }
    }
}
