/// Style or shape of curve.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CurveStyle {
    /// The Y-axis values are assumed constant until the next curve point and prior to the first curve point.
    constantYValue,
    /// The Y-axis values are assumed to be a straight line between values. Also known as linear interpolation.
    straightLineYValues,
}

impl CurveStyle {
    pub fn uri(&self) -> &'static str {
        match self {
            CurveStyle::constantYValue => "http://iec.ch/TC57/CIM100#CurveStyle.constantYValue",
            CurveStyle::straightLineYValues => "http://iec.ch/TC57/CIM100#CurveStyle.straightLineYValues",
        }
    }
}
