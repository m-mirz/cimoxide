/// Function of the lookup table.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum WindLookupTableFunctionKind {
    /// Power versus speed change (negative slip) lookup table (prr(deltaomega)). It is used for the rotor resistance control model, IEC 61400-27-1:2015, 5.6.5.3.
    prr,
    /// Power vs. speed lookup table (omega(p)). It is used for the P control model type 3, IEC 61400-27-1:2015, 5.6.5.4.
    omegap,
    /// Lookup table for voltage dependency of active current limits (ipmax(uWT)). It is used for the current limitation model, IEC 61400-27-1:2015, 5.6.5.8.
    ipmax,
    /// Lookup table for voltage dependency of reactive current limits (iqmax(uWT)). It is used for the current limitation model, IEC 61400-27-1:2015, 5.6.5.8.
    iqmax,
    /// Power vs. frequency lookup table (pWPbias(f)). It is used for the wind power plant frequency and active power control model, IEC 61400-27-1:2015, Annex D.
    pwp,
    /// Crowbar duration versus voltage variation look-up table (TCW(du)). It is a case-dependent parameter. It is used for the type 3B generator set model, IEC 61400-27-1:2015, 5.6.3.3.
    tcwdu,
    /// Lookup table to determine the duration of the power reduction after a voltage dip, depending on the size of the voltage dip (Td(uWT)). It is a type-dependent parameter. It is used for the pitch control power model, IEC 61400-27-1:2015, 5.6.5.1.
    tduwt,
    /// Lookup table for active power dependency of reactive power maximum limit (qmaxp(p)). It is used for the QP and QU limitation model, IEC 61400-27-1:2015, 5.6.5.10.
    qmaxp,
    /// Lookup table for active power dependency of reactive power minimum limit (qminp(p)). It is used for the QP and QU limitation model, IEC 61400-27-1:2015, 5.6.5.10.
    qminp,
    /// Lookup table for voltage dependency of reactive power maximum limit (qmaxu(p)). It is used for the QP and QU limitation model, IEC 61400-27-1:2015, 5.6.5.10.
    qmaxu,
    /// Lookup table for voltage dependency of reactive power minimum limit (qminu(p)). It is used for the QP and QU limitation model, IEC 61400-27-1:2015, 5.6.5.10.
    qminu,
    /// Disconnection time versus over-voltage lookup table (Tuover(uWT)). It is used for the grid protection model, IEC 61400-27-1:2015, 5.6.6.
    tuover,
    /// Disconnection time versus under-voltage lookup table (Tuunder(uWT)). It is used for the grid protection model, IEC 61400-27-1:2015, 5.6.6.
    tuunder,
    /// Disconnection time versus over-frequency lookup table (Tfover(fWT)). It is used for the grid protection model, IEC 61400-27-1:2015, 5.6.6.
    tfover,
    /// Disconnection time versus under-frequency lookup table (Tfunder(fWT)). It is used for the grid protection model, IEC 61400-27-1:2015, 5.6.6.
    tfunder,
    /// Look up table for the UQ static mode (qWP(uerr)). It is used for the voltage and reactive power control model, IEC 61400-27-1:2015, Annex D.
    qwp,
}

impl WindLookupTableFunctionKind {
    pub fn uri(&self) -> &'static str {
        match self {
            WindLookupTableFunctionKind::prr => "http://iec.ch/TC57/CIM100#WindLookupTableFunctionKind.prr",
            WindLookupTableFunctionKind::omegap => "http://iec.ch/TC57/CIM100#WindLookupTableFunctionKind.omegap",
            WindLookupTableFunctionKind::ipmax => "http://iec.ch/TC57/CIM100#WindLookupTableFunctionKind.ipmax",
            WindLookupTableFunctionKind::iqmax => "http://iec.ch/TC57/CIM100#WindLookupTableFunctionKind.iqmax",
            WindLookupTableFunctionKind::pwp => "http://iec.ch/TC57/CIM100#WindLookupTableFunctionKind.pwp",
            WindLookupTableFunctionKind::tcwdu => "http://iec.ch/TC57/CIM100#WindLookupTableFunctionKind.tcwdu",
            WindLookupTableFunctionKind::tduwt => "http://iec.ch/TC57/CIM100#WindLookupTableFunctionKind.tduwt",
            WindLookupTableFunctionKind::qmaxp => "http://iec.ch/TC57/CIM100#WindLookupTableFunctionKind.qmaxp",
            WindLookupTableFunctionKind::qminp => "http://iec.ch/TC57/CIM100#WindLookupTableFunctionKind.qminp",
            WindLookupTableFunctionKind::qmaxu => "http://iec.ch/TC57/CIM100#WindLookupTableFunctionKind.qmaxu",
            WindLookupTableFunctionKind::qminu => "http://iec.ch/TC57/CIM100#WindLookupTableFunctionKind.qminu",
            WindLookupTableFunctionKind::tuover => "http://iec.ch/TC57/CIM100#WindLookupTableFunctionKind.tuover",
            WindLookupTableFunctionKind::tuunder => "http://iec.ch/TC57/CIM100#WindLookupTableFunctionKind.tuunder",
            WindLookupTableFunctionKind::tfover => "http://iec.ch/TC57/CIM100#WindLookupTableFunctionKind.tfover",
            WindLookupTableFunctionKind::tfunder => "http://iec.ch/TC57/CIM100#WindLookupTableFunctionKind.tfunder",
            WindLookupTableFunctionKind::qwp => "http://iec.ch/TC57/CIM100#WindLookupTableFunctionKind.qwp",
        }
    }
}
