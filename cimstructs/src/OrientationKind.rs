/// The orientation of the coordinate system with respect to top, left, and the coordinate number system.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OrientationKind {
    /// For 2D diagrams, a positive orientation will result in X values increasing from left to right and Y values increasing from bottom to top. This is also known as a right hand orientation.
    positive,
    /// For 2D diagrams, a negative orientation gives the left-hand orientation (favoured by computer graphics displays) with X values increasing from left to right and Y values increasing from top to bottom. This is also known as a left hand orientation.
    negative,
}

impl OrientationKind {
    pub fn uri(&self) -> &'static str {
        match self {
            OrientationKind::positive => "http://iec.ch/TC57/CIM100#OrientationKind.positive",
            OrientationKind::negative => "http://iec.ch/TC57/CIM100#OrientationKind.negative",
        }
    }
}
