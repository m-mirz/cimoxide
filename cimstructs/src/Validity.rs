/// Validity for MeasurementValue.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Validity {
    /// The value is marked good if no abnormal condition of the acquisition function or the information source is detected.
    GOOD,
    /// The value is marked questionable if a supervision function detects an abnormal behaviour, however the value could still be valid. The client is responsible for determining whether or not values marked 'questionable' should be used.
    QUESTIONABLE,
    /// The value is marked invalid when a supervision function recognises abnormal conditions of the acquisition function or the information source (missing or non-operating updating devices). The value is not defined under this condition. The mark invalid is used to indicate to the client that the value may be incorrect and shall not be used.
    INVALID,
}

impl Validity {
    pub fn uri(&self) -> &'static str {
        match self {
            Validity::GOOD => "http://iec.ch/TC57/CIM100#Validity.GOOD",
            Validity::QUESTIONABLE => "http://iec.ch/TC57/CIM100#Validity.QUESTIONABLE",
            Validity::INVALID => "http://iec.ch/TC57/CIM100#Validity.INVALID",
        }
    }
}
