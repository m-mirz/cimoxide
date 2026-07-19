/// Types of connections for the OEL input used for static excitation systems type 6B.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ExcST6BOELselectorKind {
    /// No OEL input is used. Corresponds to OELin not = 1 and not = 2 on the ExcST6B diagram. Original ExcST6B model would have called this OELin = 0.
    noOELinput,
    /// The connection is before UEL. Corresponds to OELin = 1 on the ExcST6B diagram.
    beforeUEL,
    /// The connection is after UEL. Corresponds to OELin = 2 on the ExcST6B diagram.
    afterUEL,
}

impl ExcST6BOELselectorKind {
    pub fn uri(&self) -> &'static str {
        match self {
            ExcST6BOELselectorKind::noOELinput => "http://iec.ch/TC57/CIM100#ExcST6BOELselectorKind.noOELinput",
            ExcST6BOELselectorKind::beforeUEL => "http://iec.ch/TC57/CIM100#ExcST6BOELselectorKind.beforeUEL",
            ExcST6BOELselectorKind::afterUEL => "http://iec.ch/TC57/CIM100#ExcST6BOELselectorKind.afterUEL",
        }
    }
}
