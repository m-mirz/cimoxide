/// UVRT Q control modes MqUVRT.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum WindUVRTQcontrolModeKind {
    /// Voltage-dependent reactive current injection (MqUVRT equals 0).
    mode0,
    /// Reactive current injection controlled as the pre-fault value plus an additional voltage dependent reactive current injection (MqUVRT equals 1).
    mode1,
    /// Reactive current injection controlled as the pre-fault value plus an additional voltage-dependent reactive current injection during fault, and as the pre-fault value plus an additional constant reactive current injection post fault (MqUVRT equals 2).
    mode2,
}

impl WindUVRTQcontrolModeKind {
    pub fn uri(&self) -> &'static str {
        match self {
            WindUVRTQcontrolModeKind::mode0 => "http://iec.ch/TC57/CIM100#WindUVRTQcontrolModeKind.mode0",
            WindUVRTQcontrolModeKind::mode1 => "http://iec.ch/TC57/CIM100#WindUVRTQcontrolModeKind.mode1",
            WindUVRTQcontrolModeKind::mode2 => "http://iec.ch/TC57/CIM100#WindUVRTQcontrolModeKind.mode2",
        }
    }
}
