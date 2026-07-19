/// Types of connections for the OEL input used for static excitation systems type 7B.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ExcST7BOELselectorKind {
    /// No OEL input is used. Corresponds to OELin not = 1 and not = 2 and not = 3 on the ExcST7B diagram. Original ExcST7B model would have called this OELin = 0.
    noOELinput,
    /// The signal is added to Vref. Corresponds to OELin = 1 on the ExcST7B diagram.
    addVref,
    /// The signal is connected into the input LVGate. Corresponds to OELin = 2 on the ExcST7B diagram.
    inputLVgate,
    /// The signal is connected into the output LVGate. Corresponds to OELin = 3 on the ExcST7B diagram.
    outputLVgate,
}

impl ExcST7BOELselectorKind {
    pub fn uri(&self) -> &'static str {
        match self {
            ExcST7BOELselectorKind::noOELinput => "http://iec.ch/TC57/CIM100#ExcST7BOELselectorKind.noOELinput",
            ExcST7BOELselectorKind::addVref => "http://iec.ch/TC57/CIM100#ExcST7BOELselectorKind.addVref",
            ExcST7BOELselectorKind::inputLVgate => "http://iec.ch/TC57/CIM100#ExcST7BOELselectorKind.inputLVgate",
            ExcST7BOELselectorKind::outputLVgate => "http://iec.ch/TC57/CIM100#ExcST7BOELselectorKind.outputLVgate",
        }
    }
}
