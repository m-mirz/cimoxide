/// The direction attribute describes the side of a limit that is a violation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OperationalLimitDirectionKind {
    /// High means that a monitored value above the limit value is a violation. If applied to a terminal flow, the positive direction is into the terminal.
    high,
    /// Low means a monitored value below the limit is a violation. If applied to a terminal flow, the positive direction is into the terminal.
    low,
    /// An absoluteValue limit means that a monitored absolute value above the limit value is a violation.
    absoluteValue,
}

impl OperationalLimitDirectionKind {
    pub fn uri(&self) -> &'static str {
        match self {
            OperationalLimitDirectionKind::high => "http://iec.ch/TC57/CIM100#OperationalLimitDirectionKind.high",
            OperationalLimitDirectionKind::low => "http://iec.ch/TC57/CIM100#OperationalLimitDirectionKind.low",
            OperationalLimitDirectionKind::absoluteValue => "http://iec.ch/TC57/CIM100#OperationalLimitDirectionKind.absoluteValue",
        }
    }
}
