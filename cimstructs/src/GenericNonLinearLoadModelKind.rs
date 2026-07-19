/// Type of generic non-linear load model.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum GenericNonLinearLoadModelKind {
    /// Exponential recovery model.
    exponentialRecovery,
    /// Load adaptive model.
    loadAdaptive,
}

impl GenericNonLinearLoadModelKind {
    pub fn uri(&self) -> &'static str {
        match self {
            GenericNonLinearLoadModelKind::exponentialRecovery => "http://iec.ch/TC57/CIM100#GenericNonLinearLoadModelKind.exponentialRecovery",
            GenericNonLinearLoadModelKind::loadAdaptive => "http://iec.ch/TC57/CIM100#GenericNonLinearLoadModelKind.loadAdaptive",
        }
    }
}
