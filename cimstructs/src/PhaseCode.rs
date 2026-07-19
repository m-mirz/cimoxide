/// An unordered enumeration of phase identifiers. Allows designation of phases for both transmission and distribution equipment, circuits and loads. The enumeration, by itself, does not describe how the phases are connected together or connected to ground. Ground is not explicitly denoted as a phase. Residential and small commercial loads are often served from single-phase, or split-phase, secondary circuits. For the example of s12N, phases 1 and 2 refer to hot wires that are 180 degrees out of phase, while N refers to the neutral wire. Through single-phase transformer connections, these secondary circuits may be served from one or two of the primary phases A, B, and C. For three-phase loads, use the A, B, C phase codes instead of s12N. The integer values are from IEC 61968-9 to support revenue metering applications.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PhaseCode {
    /// Phases A, B, C, and N.
    ABCN,
    /// Phases A, B, and C.
    ABC,
    /// Phases A, B, and neutral.
    ABN,
    /// Phases A, C and neutral.
    ACN,
    /// Phases B, C, and neutral.
    BCN,
    /// Phases A and B.
    AB,
    /// Phases A and C.
    AC,
    /// Phases B and C.
    BC,
    /// Phases A and neutral.
    AN,
    /// Phases B and neutral.
    BN,
    /// Phases C and neutral.
    CN,
    /// Phase A.
    A,
    /// Phase B.
    B,
    /// Phase C.
    C,
    /// Neutral phase.
    N,
    /// Secondary phase 1 and neutral.
    s1N,
    /// Secondary phase 2 and neutral.
    s2N,
    /// Secondary phases 1, 2, and neutral.
    s12N,
    /// Secondary phase 1.
    s1,
    /// Secondary phase 2.
    s2,
    /// Secondary phase 1 and 2.
    s12,
    /// No phases specified.
    none,
    /// Unknown non-neutral phase.
    X,
    /// Two unknown non-neutral phases.
    XY,
    /// Unknown non-neutral phase plus neutral.
    XN,
    /// Two unknown non-neutral phases plus neutral.
    XYN,
}

impl PhaseCode {
    pub fn uri(&self) -> &'static str {
        match self {
            PhaseCode::ABCN => "http://iec.ch/TC57/CIM100#PhaseCode.ABCN",
            PhaseCode::ABC => "http://iec.ch/TC57/CIM100#PhaseCode.ABC",
            PhaseCode::ABN => "http://iec.ch/TC57/CIM100#PhaseCode.ABN",
            PhaseCode::ACN => "http://iec.ch/TC57/CIM100#PhaseCode.ACN",
            PhaseCode::BCN => "http://iec.ch/TC57/CIM100#PhaseCode.BCN",
            PhaseCode::AB => "http://iec.ch/TC57/CIM100#PhaseCode.AB",
            PhaseCode::AC => "http://iec.ch/TC57/CIM100#PhaseCode.AC",
            PhaseCode::BC => "http://iec.ch/TC57/CIM100#PhaseCode.BC",
            PhaseCode::AN => "http://iec.ch/TC57/CIM100#PhaseCode.AN",
            PhaseCode::BN => "http://iec.ch/TC57/CIM100#PhaseCode.BN",
            PhaseCode::CN => "http://iec.ch/TC57/CIM100#PhaseCode.CN",
            PhaseCode::A => "http://iec.ch/TC57/CIM100#PhaseCode.A",
            PhaseCode::B => "http://iec.ch/TC57/CIM100#PhaseCode.B",
            PhaseCode::C => "http://iec.ch/TC57/CIM100#PhaseCode.C",
            PhaseCode::N => "http://iec.ch/TC57/CIM100#PhaseCode.N",
            PhaseCode::s1N => "http://iec.ch/TC57/CIM100#PhaseCode.s1N",
            PhaseCode::s2N => "http://iec.ch/TC57/CIM100#PhaseCode.s2N",
            PhaseCode::s12N => "http://iec.ch/TC57/CIM100#PhaseCode.s12N",
            PhaseCode::s1 => "http://iec.ch/TC57/CIM100#PhaseCode.s1",
            PhaseCode::s2 => "http://iec.ch/TC57/CIM100#PhaseCode.s2",
            PhaseCode::s12 => "http://iec.ch/TC57/CIM100#PhaseCode.s12",
            PhaseCode::none => "http://iec.ch/TC57/CIM100#PhaseCode.none",
            PhaseCode::X => "http://iec.ch/TC57/CIM100#PhaseCode.X",
            PhaseCode::XY => "http://iec.ch/TC57/CIM100#PhaseCode.XY",
            PhaseCode::XN => "http://iec.ch/TC57/CIM100#PhaseCode.XN",
            PhaseCode::XYN => "http://iec.ch/TC57/CIM100#PhaseCode.XYN",
        }
    }
}
