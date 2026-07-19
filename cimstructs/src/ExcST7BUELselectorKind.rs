/// Types of connections for the UEL input used for static excitation systems type 7B.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ExcST7BUELselectorKind {
    /// No UEL input is used. Corresponds to UELin not = 1 and not = 2 and not = 3 on the ExcST7B diagram. Original ExcST7B model would have called this UELin = 0.
    noUELinput,
    /// The signal is added to Vref. Corresponds to UELin = 1 on the ExcST7B diagram.
    addVref,
    /// The signal is connected into the input HVGate. Corresponds to UELin = 2 on the ExcST7B diagram.
    inputHVgate,
    /// The signal is connected into the output HVGate. Corresponds to UELin = 3 on the ExcST7B diagram.
    outputHVgate,
}

impl ExcST7BUELselectorKind {
    pub fn uri(&self) -> &'static str {
        match self {
            ExcST7BUELselectorKind::noUELinput => "http://iec.ch/TC57/CIM100#ExcST7BUELselectorKind.noUELinput",
            ExcST7BUELselectorKind::addVref => "http://iec.ch/TC57/CIM100#ExcST7BUELselectorKind.addVref",
            ExcST7BUELselectorKind::inputHVgate => "http://iec.ch/TC57/CIM100#ExcST7BUELselectorKind.inputHVgate",
            ExcST7BUELselectorKind::outputHVgate => "http://iec.ch/TC57/CIM100#ExcST7BUELselectorKind.outputHVgate",
        }
    }
}
