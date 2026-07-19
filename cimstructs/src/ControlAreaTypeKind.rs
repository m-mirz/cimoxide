/// The type of control area.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ControlAreaTypeKind {
    /// Used for automatic generation control.
    AGC,
    /// Used for load forecast.
    Forecast,
    /// Used for interchange specification or control.
    Interchange,
}

impl ControlAreaTypeKind {
    pub fn uri(&self) -> &'static str {
        match self {
            ControlAreaTypeKind::AGC => "http://iec.ch/TC57/CIM100#ControlAreaTypeKind.AGC",
            ControlAreaTypeKind::Forecast => "http://iec.ch/TC57/CIM100#ControlAreaTypeKind.Forecast",
            ControlAreaTypeKind::Interchange => "http://iec.ch/TC57/CIM100#ControlAreaTypeKind.Interchange",
        }
    }
}
