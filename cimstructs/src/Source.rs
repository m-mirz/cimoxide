/// Source gives information related to the origin of a value.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Source {
    /// The value is provided by input from the process I/O or being calculated from some function.
    PROCESS,
    /// The value contains a default value.
    DEFAULTED,
    /// The value is provided by input of an operator or by an automatic source.
    SUBSTITUTED,
}

impl Source {
    pub fn uri(&self) -> &'static str {
        match self {
            Source::PROCESS => "http://iec.ch/TC57/CIM100#Source.PROCESS",
            Source::DEFAULTED => "http://iec.ch/TC57/CIM100#Source.DEFAULTED",
            Source::SUBSTITUTED => "http://iec.ch/TC57/CIM100#Source.SUBSTITUTED",
        }
    }
}
