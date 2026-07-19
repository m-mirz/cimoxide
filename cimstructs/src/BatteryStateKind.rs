/// The state of the battery unit.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum BatteryStateKind {
    /// Stored energy is decreasing.
    discharging,
    /// Unable to charge, and not discharging.
    full,
    /// Neither charging nor discharging, but able to do so.
    waiting,
    /// Stored energy is increasing.
    charging,
    /// Unable to discharge, and not charging.
    empty,
}

impl BatteryStateKind {
    pub fn uri(&self) -> &'static str {
        match self {
            BatteryStateKind::discharging => "http://iec.ch/TC57/CIM100#BatteryStateKind.discharging",
            BatteryStateKind::full => "http://iec.ch/TC57/CIM100#BatteryStateKind.full",
            BatteryStateKind::waiting => "http://iec.ch/TC57/CIM100#BatteryStateKind.waiting",
            BatteryStateKind::charging => "http://iec.ch/TC57/CIM100#BatteryStateKind.charging",
            BatteryStateKind::empty => "http://iec.ch/TC57/CIM100#BatteryStateKind.empty",
        }
    }
}
