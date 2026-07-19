/// Type of synchronous machine model used in dynamic simulation applications.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SynchronousMachineModelKind {
    /// Subtransient synchronous machine model.
    subtransient,
    /// WECC type F variant of subtransient synchronous machine model.
    subtransientTypeF,
    /// WECC type J variant of subtransient synchronous machine model.
    subtransientTypeJ,
    /// Simplified version of subtransient synchronous machine model where magnetic coupling between the direct- and quadrature- axes is ignored.
    subtransientSimplified,
    /// Simplified version of a subtransient synchronous machine model with no damper circuit on the direct-axis.
    subtransientSimplifiedDirectAxis,
}

impl SynchronousMachineModelKind {
    pub fn uri(&self) -> &'static str {
        match self {
            SynchronousMachineModelKind::subtransient => "http://iec.ch/TC57/CIM100#SynchronousMachineModelKind.subtransient",
            SynchronousMachineModelKind::subtransientTypeF => "http://iec.ch/TC57/CIM100#SynchronousMachineModelKind.subtransientTypeF",
            SynchronousMachineModelKind::subtransientTypeJ => "http://iec.ch/TC57/CIM100#SynchronousMachineModelKind.subtransientTypeJ",
            SynchronousMachineModelKind::subtransientSimplified => "http://iec.ch/TC57/CIM100#SynchronousMachineModelKind.subtransientSimplified",
            SynchronousMachineModelKind::subtransientSimplifiedDirectAxis => "http://iec.ch/TC57/CIM100#SynchronousMachineModelKind.subtransientSimplifiedDirectAxis",
        }
    }
}
