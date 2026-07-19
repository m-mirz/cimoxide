/// The derived units defined for usage in the CIM. In some cases, the derived unit is equal to an SI unit. Whenever possible, the standard derived symbol is used instead of the formula for the derived unit. For example, the unit symbol Farad is defined as 'F' instead of 'CPerV'. In cases where a standard symbol does not exist for a derived unit, the formula for the unit is used as the unit symbol. For example, density does not have a standard symbol and so it is represented as 'kgPerm3'. With the exception of the 'kg', which is an SI unit, the unit symbols do not contain multipliers and therefore represent the base derived unit to which a multiplier can be applied as a whole. Every unit symbol is treated as an unparseable text as if it were a single-letter symbol. The meaning of each unit symbol is defined by the accompanying descriptive text and not by the text contents of the unit symbol. To allow the widest possible range of serializations without requiring special character handling, several substitutions are made which deviate from the format described in IEC 80000-1. The division symbol '/' is replaced by the letters 'Per'. Exponents are written in plain text after the unit as 'm3' instead of being formatted as 'm' with a superscript of 3 or introducing a symbol as in 'm^3'. The degree symbol '°' is replaced with the letters 'deg'. Any clarification of the meaning for a substitution is included in the description for the unit symbol. Non-SI units are included in list of unit symbols to allow sources of data to be correctly labelled with their non-SI units (for example, a GPS sensor that is reporting numbers that represent feet instead of meters). This allows software to use the unit symbol information correctly convert and scale the raw data of those sources into SI-based units. The integer values are used for harmonization with IEC 61850.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum UnitSymbol {
    /// Dimension less quantity, e.g. count, per unit, etc.
    none,
    /// Length in metres.
    m,
    /// Mass in kilograms. Note: multiplier “k” is included in this unit symbol for compatibility with IEC 61850-7-3.
    kg,
    /// Time in seconds.
    s,
    /// Current in amperes.
    A,
    /// Temperature in kelvins.
    K,
    /// Amount of substance in moles.
    mol,
    /// Luminous intensity in candelas.
    cd,
    /// Plane angle in degrees.
    deg,
    /// Plane angle in radians (m/m).
    rad,
    /// Solid angle in steradians (m2/m2).
    sr,
    /// Absorbed dose in grays (J/kg).
    Gy,
    /// Radioactivity in becquerels (1/s).
    Bq,
    /// Relative temperature in degrees Celsius. In the SI unit system the symbol is °C. Electric charge is measured in coulomb that has the unit symbol C. To distinguish degree Celsius from coulomb the symbol used in the UML is degC. The reason for not using °C is that the special character ° is difficult to manage in software.
    degC,
    /// Dose equivalent in sieverts (J/kg).
    Sv,
    /// Electric capacitance in farads (C/V).
    F,
    /// Electric charge in coulombs (A·s).
    C,
    /// Conductance in siemens.
    S,
    /// Electric inductance in henrys (Wb/A).
    H,
    /// Electric potential in volts (W/A).
    V,
    /// Electric resistance in ohms (V/A).
    ohm,
    /// Energy in joules (N·m = C·V = W·s).
    J,
    /// Force in newtons (kg·m/s²).
    N,
    /// Frequency in hertz (1/s).
    Hz,
    /// Illuminance in lux (lm/m²).
    lx,
    /// Luminous flux in lumens (cd·sr).
    lm,
    /// Magnetic flux in webers (V·s).
    Wb,
    /// Magnetic flux density in teslas (Wb/m2).
    T,
    /// Real power in watts (J/s). Electrical power may have real and reactive components. The real portion of electrical power (I&#178;R or VIcos(phi)), is expressed in Watts. See also apparent power and reactive power.
    W,
    /// Pressure in pascals (N/m²). Note: the absolute or relative measurement of pressure is implied with this entry. See below for more explicit forms.
    Pa,
    /// Area in square metres (m²).
    m2,
    /// Volume in cubic metres (m³).
    m3,
    /// Velocity in metres per second (m/s).
    mPers,
    /// Acceleration in metres per second squared (m/s²).
    mPers2,
    /// Volumetric flow rate in cubic metres per second (m³/s).
    m3Pers,
    /// Fuel efficiency in metres per cubic metres (m/m³).
    mPerm3,
    /// Moment of mass in kilogram metres (kg·m) (first moment of mass). Note: multiplier “k” is included in this unit symbol for compatibility with IEC 61850-7-3.
    kgm,
    /// Density in kilogram/cubic metres (kg/m³). Note: multiplier “k” is included in this unit symbol for compatibility with IEC 61850-7-3.
    kgPerm3,
    /// Viscosity in square metres / second (m²/s).
    m2Pers,
    /// Thermal conductivity in watt/metres kelvin.
    WPermK,
    /// Heat capacity in joules/kelvin.
    JPerK,
    /// Concentration in parts per million.
    ppm,
    /// Rotations per second (1/s). See also Hz (1/s).
    rotPers,
    /// Angular velocity in radians per second (rad/s).
    radPers,
    /// Heat flux density, irradiance, watts per square metre.
    WPerm2,
    /// Insulation energy density, joules per square metre or watt second per square metre.
    JPerm2,
    /// Conductance per length (F/m).
    SPerm,
    /// Temperature change rate in kelvins per second.
    KPers,
    /// Pressure change rate in pascals per second.
    PaPers,
    /// Specific heat capacity, specific entropy, joules per kilogram Kelvin.
    JPerkgK,
    /// Apparent power in volt amperes. See also real power and reactive power.
    VA,
    /// Reactive power in volt amperes reactive. The “reactive” or “imaginary” component of electrical power (VIsin(phi)). (See also real power and apparent power). Note: Different meter designs use different methods to arrive at their results. Some meters may compute reactive power as an arithmetic value, while others compute the value vectorially. The data consumer should determine the method in use and the suitability of the measurement for the intended purpose.
    VAr,
    /// Power factor, dimensionless. Note 1: This definition of power factor only holds for balanced systems. See the alternative definition under code 153. Note 2 : Beware of differing sign conventions in use between the IEC and EEI. It is assumed that the data consumer understands the type of meter in use and the sign convention in use by the utility.
    cosPhi,
    /// Volt seconds (Ws/A).
    Vs,
    /// Volt squared (W²/A²).
    V2,
    /// Ampere seconds (A·s).
    As,
    /// Amperes squared (A²).
    A2,
    /// Ampere squared time in square amperes (A²s).
    A2s,
    /// Apparent energy in volt ampere hours.
    VAh,
    /// Real energy in watt hours.
    Wh,
    /// Reactive energy in volt ampere reactive hours.
    VArh,
    /// Magnetic flux in volt per hertz.
    VPerHz,
    /// Rate of change of frequency in hertz per second.
    HzPers,
    /// Number of characters.
    character,
    /// Data rate (baud) in characters per second.
    charPers,
    /// Moment of mass in kilogram square metres (kg·m²) (Second moment of mass, commonly called the moment of inertia). Note: multiplier “k” is included in this unit symbol for compatibility with IEC 61850-7-3.
    kgm2,
    /// Sound pressure level in decibels. Note: multiplier “d” is included in this unit symbol for compatibility with IEC 61850-7-3.
    dB,
    /// Ramp rate in watts per second.
    WPers,
    /// Volumetric flow rate in litres per second.
    lPers,
    /// Power level (logarithmic ratio of signal strength , Bel-mW), normalized to 1mW. Note: multiplier “d” is included in this unit symbol for compatibility with IEC 61850-7-3.
    dBm,
    /// Time in hours, hour = 60 min = 3600 s.
    h,
    /// Time in minutes, minute = 60 s.
    min,
    /// Quantity power, Q.
    Q,
    /// Quantity energy, Qh.
    Qh,
    /// Resistivity, ohm metres, (rho).
    ohmm,
    /// A/m, magnetic field strength, amperes per metre.
    APerm,
    /// Volt-squared hour, volt-squared-hours.
    V2h,
    /// Ampere-squared hour, ampere-squared hour.
    A2h,
    /// Ampere-hours, ampere-hours.
    Ah,
    /// Amount of substance, Counter value.
    count,
    /// Volume, cubic feet.
    ft3,
    /// Volumetric flow rate, cubic metres per hour.
    m3Perh,
    /// Volume in gallons, US gallon (1 gal = 231 in3 = 128 fl ounce).
    gal,
    /// Energy, British Thermal Units.
    Btu,
    /// Volume in litres, litre = dm3 = m3/1000.
    l,
    /// Volumetric flow rate, litres per hour.
    lPerh,
    /// Concentration, The ratio of the volume of a solute divided by the volume of the solution. Note: Users may need use a prefix such a ‘µ’ to express a quantity such as ‘µL/L’.
    lPerl,
    /// Concentration, The ratio of the mass of a solute divided by the mass of the solution. Note: Users may need use a prefix such a ‘µ’ to express a quantity such as ‘µg/g’.
    gPerg,
    /// Concentration, The amount of substance concentration, (c), the amount of solvent in moles divided by the volume of solution in m³.
    molPerm3,
    /// Concentration, Molar fraction, the ratio of the molar amount of a solute divided by the molar amount of the solution.
    molPermol,
    /// Concentration, Molality, the amount of solute in moles and the amount of solvent in kilograms.
    molPerkg,
    /// Time, Ratio of time. Note: Users may need to supply a prefix such as ‘&#181;’ to show rates such as ‘&#181;s/s’.
    sPers,
    /// Frequency, rate of frequency change. Note: Users may need to supply a prefix such as ‘m’ to show rates such as ‘mHz/Hz’.
    HzPerHz,
    /// Voltage, ratio of voltages. Note: Users may need to supply a prefix such as ‘m’ to show rates such as ‘mV/V’.
    VPerV,
    /// Current, ratio of amperages. Note: Users may need to supply a prefix such as ‘m’ to show rates such as ‘mA/A’.
    APerA,
    /// Power factor, PF, the ratio of the active power to the apparent power. Note: The sign convention used for power factor will differ between IEC meters and EEI (ANSI) meters. It is assumed that the data consumers understand the type of meter being used and agree on the sign convention in use at any given utility.
    VPerVA,
    /// Amount of rotation, revolutions.
    rev,
    /// Catalytic activity, katal = mol / s.
    kat,
    /// Specific energy, Joules / kg.
    JPerkg,
    /// Volume, cubic metres, with the value uncompensated for weather effects.
    m3Uncompensated,
    /// Volume, cubic metres, with the value compensated for weather effects.
    m3Compensated,
    /// Signal Strength, ratio of power. Note: Users may need to supply a prefix such as ‘m’ to show rates such as ‘mW/W’.
    WPerW,
    /// Energy, therms.
    therm,
    /// Wavenumber, reciprocal metres, (1/m).
    onePerm,
    /// Specific volume, cubic metres per kilogram, v.
    m3Perkg,
    /// Dynamic viscosity, pascal seconds.
    Pas,
    /// Moment of force, newton metres.
    Nm,
    /// Surface tension, newton per metre.
    NPerm,
    /// Angular acceleration, radians per second squared.
    radPers2,
    /// Energy density, joules per cubic metre.
    JPerm3,
    /// Electric field strength, volts per metre.
    VPerm,
    /// Electric charge density, coulombs per cubic metre.
    CPerm3,
    /// Surface charge density, coulombs per square metre.
    CPerm2,
    /// Permittivity, farads per metre.
    FPerm,
    /// Permeability, henrys per metre.
    HPerm,
    /// Molar energy, joules per mole.
    JPermol,
    /// Molar entropy, molar heat capacity, joules per mole kelvin.
    JPermolK,
    /// Exposure (x rays), coulombs per kilogram.
    CPerkg,
    /// Absorbed dose rate, grays per second.
    GyPers,
    /// Radiant intensity, watts per steradian.
    WPersr,
    /// Radiance, watts per square metre steradian.
    WPerm2sr,
    /// Catalytic activity concentration, katals per cubic metre.
    katPerm3,
    /// Time in days, day = 24 h = 86400 s.
    d,
    /// Plane angle, minutes.
    anglemin,
    /// Plane angle, seconds.
    anglesec,
    /// Area, hectares.
    ha,
    /// Mass in tons, “tonne” or “metric ton” (1000 kg = 1 Mg).
    tonne,
    /// Pressure in bars, (1 bar = 100 kPa).
    bar,
    /// Pressure, millimetres of mercury (1 mmHg is approximately 133.3 Pa).
    mmHg,
    /// Length, nautical miles (1 M = 1852 m).
    M,
    /// Speed, knots (1 kn = 1852/3600) m/s.
    kn,
    /// Magnetic flux, maxwells (1 Mx = 10-8 Wb).
    Mx,
    /// Magnetic flux density, gausses (1 G = 10-4 T).
    G,
    /// Magnetic field in oersteds, (1 Oe = (103/4p) A/m).
    Oe,
    /// Volt-hour, Volt hours.
    Vh,
    /// Active power per current flow, watts per Ampere.
    WPerA,
    /// Reciprocal of frequency (1/Hz).
    onePerHz,
    /// Power factor, PF, the ratio of the active power to the apparent power. Note: The sign convention used for power factor will differ between IEC meters and EEI (ANSI) meters. It is assumed that the data consumers understand the type of meter being used and agree on the sign convention in use at any given utility.
    VPerVAr,
    /// Electric resistance per length in ohms per metre ((V/A)/m).
    ohmPerm,
    /// Weight per energy in kilograms per joule (kg/J). Note: multiplier “k” is included in this unit symbol for compatibility with IEC 61850-7-3.
    kgPerJ,
    /// Energy rate in joules per second (J/s).
    JPers,
}

impl UnitSymbol {
    pub fn uri(&self) -> &'static str {
        match self {
            UnitSymbol::none => "http://iec.ch/TC57/CIM100#UnitSymbol.none",
            UnitSymbol::m => "http://iec.ch/TC57/CIM100#UnitSymbol.m",
            UnitSymbol::kg => "http://iec.ch/TC57/CIM100#UnitSymbol.kg",
            UnitSymbol::s => "http://iec.ch/TC57/CIM100#UnitSymbol.s",
            UnitSymbol::A => "http://iec.ch/TC57/CIM100#UnitSymbol.A",
            UnitSymbol::K => "http://iec.ch/TC57/CIM100#UnitSymbol.K",
            UnitSymbol::mol => "http://iec.ch/TC57/CIM100#UnitSymbol.mol",
            UnitSymbol::cd => "http://iec.ch/TC57/CIM100#UnitSymbol.cd",
            UnitSymbol::deg => "http://iec.ch/TC57/CIM100#UnitSymbol.deg",
            UnitSymbol::rad => "http://iec.ch/TC57/CIM100#UnitSymbol.rad",
            UnitSymbol::sr => "http://iec.ch/TC57/CIM100#UnitSymbol.sr",
            UnitSymbol::Gy => "http://iec.ch/TC57/CIM100#UnitSymbol.Gy",
            UnitSymbol::Bq => "http://iec.ch/TC57/CIM100#UnitSymbol.Bq",
            UnitSymbol::degC => "http://iec.ch/TC57/CIM100#UnitSymbol.degC",
            UnitSymbol::Sv => "http://iec.ch/TC57/CIM100#UnitSymbol.Sv",
            UnitSymbol::F => "http://iec.ch/TC57/CIM100#UnitSymbol.F",
            UnitSymbol::C => "http://iec.ch/TC57/CIM100#UnitSymbol.C",
            UnitSymbol::S => "http://iec.ch/TC57/CIM100#UnitSymbol.S",
            UnitSymbol::H => "http://iec.ch/TC57/CIM100#UnitSymbol.H",
            UnitSymbol::V => "http://iec.ch/TC57/CIM100#UnitSymbol.V",
            UnitSymbol::ohm => "http://iec.ch/TC57/CIM100#UnitSymbol.ohm",
            UnitSymbol::J => "http://iec.ch/TC57/CIM100#UnitSymbol.J",
            UnitSymbol::N => "http://iec.ch/TC57/CIM100#UnitSymbol.N",
            UnitSymbol::Hz => "http://iec.ch/TC57/CIM100#UnitSymbol.Hz",
            UnitSymbol::lx => "http://iec.ch/TC57/CIM100#UnitSymbol.lx",
            UnitSymbol::lm => "http://iec.ch/TC57/CIM100#UnitSymbol.lm",
            UnitSymbol::Wb => "http://iec.ch/TC57/CIM100#UnitSymbol.Wb",
            UnitSymbol::T => "http://iec.ch/TC57/CIM100#UnitSymbol.T",
            UnitSymbol::W => "http://iec.ch/TC57/CIM100#UnitSymbol.W",
            UnitSymbol::Pa => "http://iec.ch/TC57/CIM100#UnitSymbol.Pa",
            UnitSymbol::m2 => "http://iec.ch/TC57/CIM100#UnitSymbol.m2",
            UnitSymbol::m3 => "http://iec.ch/TC57/CIM100#UnitSymbol.m3",
            UnitSymbol::mPers => "http://iec.ch/TC57/CIM100#UnitSymbol.mPers",
            UnitSymbol::mPers2 => "http://iec.ch/TC57/CIM100#UnitSymbol.mPers2",
            UnitSymbol::m3Pers => "http://iec.ch/TC57/CIM100#UnitSymbol.m3Pers",
            UnitSymbol::mPerm3 => "http://iec.ch/TC57/CIM100#UnitSymbol.mPerm3",
            UnitSymbol::kgm => "http://iec.ch/TC57/CIM100#UnitSymbol.kgm",
            UnitSymbol::kgPerm3 => "http://iec.ch/TC57/CIM100#UnitSymbol.kgPerm3",
            UnitSymbol::m2Pers => "http://iec.ch/TC57/CIM100#UnitSymbol.m2Pers",
            UnitSymbol::WPermK => "http://iec.ch/TC57/CIM100#UnitSymbol.WPermK",
            UnitSymbol::JPerK => "http://iec.ch/TC57/CIM100#UnitSymbol.JPerK",
            UnitSymbol::ppm => "http://iec.ch/TC57/CIM100#UnitSymbol.ppm",
            UnitSymbol::rotPers => "http://iec.ch/TC57/CIM100#UnitSymbol.rotPers",
            UnitSymbol::radPers => "http://iec.ch/TC57/CIM100#UnitSymbol.radPers",
            UnitSymbol::WPerm2 => "http://iec.ch/TC57/CIM100#UnitSymbol.WPerm2",
            UnitSymbol::JPerm2 => "http://iec.ch/TC57/CIM100#UnitSymbol.JPerm2",
            UnitSymbol::SPerm => "http://iec.ch/TC57/CIM100#UnitSymbol.SPerm",
            UnitSymbol::KPers => "http://iec.ch/TC57/CIM100#UnitSymbol.KPers",
            UnitSymbol::PaPers => "http://iec.ch/TC57/CIM100#UnitSymbol.PaPers",
            UnitSymbol::JPerkgK => "http://iec.ch/TC57/CIM100#UnitSymbol.JPerkgK",
            UnitSymbol::VA => "http://iec.ch/TC57/CIM100#UnitSymbol.VA",
            UnitSymbol::VAr => "http://iec.ch/TC57/CIM100#UnitSymbol.VAr",
            UnitSymbol::cosPhi => "http://iec.ch/TC57/CIM100#UnitSymbol.cosPhi",
            UnitSymbol::Vs => "http://iec.ch/TC57/CIM100#UnitSymbol.Vs",
            UnitSymbol::V2 => "http://iec.ch/TC57/CIM100#UnitSymbol.V2",
            UnitSymbol::As => "http://iec.ch/TC57/CIM100#UnitSymbol.As",
            UnitSymbol::A2 => "http://iec.ch/TC57/CIM100#UnitSymbol.A2",
            UnitSymbol::A2s => "http://iec.ch/TC57/CIM100#UnitSymbol.A2s",
            UnitSymbol::VAh => "http://iec.ch/TC57/CIM100#UnitSymbol.VAh",
            UnitSymbol::Wh => "http://iec.ch/TC57/CIM100#UnitSymbol.Wh",
            UnitSymbol::VArh => "http://iec.ch/TC57/CIM100#UnitSymbol.VArh",
            UnitSymbol::VPerHz => "http://iec.ch/TC57/CIM100#UnitSymbol.VPerHz",
            UnitSymbol::HzPers => "http://iec.ch/TC57/CIM100#UnitSymbol.HzPers",
            UnitSymbol::character => "http://iec.ch/TC57/CIM100#UnitSymbol.character",
            UnitSymbol::charPers => "http://iec.ch/TC57/CIM100#UnitSymbol.charPers",
            UnitSymbol::kgm2 => "http://iec.ch/TC57/CIM100#UnitSymbol.kgm2",
            UnitSymbol::dB => "http://iec.ch/TC57/CIM100#UnitSymbol.dB",
            UnitSymbol::WPers => "http://iec.ch/TC57/CIM100#UnitSymbol.WPers",
            UnitSymbol::lPers => "http://iec.ch/TC57/CIM100#UnitSymbol.lPers",
            UnitSymbol::dBm => "http://iec.ch/TC57/CIM100#UnitSymbol.dBm",
            UnitSymbol::h => "http://iec.ch/TC57/CIM100#UnitSymbol.h",
            UnitSymbol::min => "http://iec.ch/TC57/CIM100#UnitSymbol.min",
            UnitSymbol::Q => "http://iec.ch/TC57/CIM100#UnitSymbol.Q",
            UnitSymbol::Qh => "http://iec.ch/TC57/CIM100#UnitSymbol.Qh",
            UnitSymbol::ohmm => "http://iec.ch/TC57/CIM100#UnitSymbol.ohmm",
            UnitSymbol::APerm => "http://iec.ch/TC57/CIM100#UnitSymbol.APerm",
            UnitSymbol::V2h => "http://iec.ch/TC57/CIM100#UnitSymbol.V2h",
            UnitSymbol::A2h => "http://iec.ch/TC57/CIM100#UnitSymbol.A2h",
            UnitSymbol::Ah => "http://iec.ch/TC57/CIM100#UnitSymbol.Ah",
            UnitSymbol::count => "http://iec.ch/TC57/CIM100#UnitSymbol.count",
            UnitSymbol::ft3 => "http://iec.ch/TC57/CIM100#UnitSymbol.ft3",
            UnitSymbol::m3Perh => "http://iec.ch/TC57/CIM100#UnitSymbol.m3Perh",
            UnitSymbol::gal => "http://iec.ch/TC57/CIM100#UnitSymbol.gal",
            UnitSymbol::Btu => "http://iec.ch/TC57/CIM100#UnitSymbol.Btu",
            UnitSymbol::l => "http://iec.ch/TC57/CIM100#UnitSymbol.l",
            UnitSymbol::lPerh => "http://iec.ch/TC57/CIM100#UnitSymbol.lPerh",
            UnitSymbol::lPerl => "http://iec.ch/TC57/CIM100#UnitSymbol.lPerl",
            UnitSymbol::gPerg => "http://iec.ch/TC57/CIM100#UnitSymbol.gPerg",
            UnitSymbol::molPerm3 => "http://iec.ch/TC57/CIM100#UnitSymbol.molPerm3",
            UnitSymbol::molPermol => "http://iec.ch/TC57/CIM100#UnitSymbol.molPermol",
            UnitSymbol::molPerkg => "http://iec.ch/TC57/CIM100#UnitSymbol.molPerkg",
            UnitSymbol::sPers => "http://iec.ch/TC57/CIM100#UnitSymbol.sPers",
            UnitSymbol::HzPerHz => "http://iec.ch/TC57/CIM100#UnitSymbol.HzPerHz",
            UnitSymbol::VPerV => "http://iec.ch/TC57/CIM100#UnitSymbol.VPerV",
            UnitSymbol::APerA => "http://iec.ch/TC57/CIM100#UnitSymbol.APerA",
            UnitSymbol::VPerVA => "http://iec.ch/TC57/CIM100#UnitSymbol.VPerVA",
            UnitSymbol::rev => "http://iec.ch/TC57/CIM100#UnitSymbol.rev",
            UnitSymbol::kat => "http://iec.ch/TC57/CIM100#UnitSymbol.kat",
            UnitSymbol::JPerkg => "http://iec.ch/TC57/CIM100#UnitSymbol.JPerkg",
            UnitSymbol::m3Uncompensated => "http://iec.ch/TC57/CIM100#UnitSymbol.m3Uncompensated",
            UnitSymbol::m3Compensated => "http://iec.ch/TC57/CIM100#UnitSymbol.m3Compensated",
            UnitSymbol::WPerW => "http://iec.ch/TC57/CIM100#UnitSymbol.WPerW",
            UnitSymbol::therm => "http://iec.ch/TC57/CIM100#UnitSymbol.therm",
            UnitSymbol::onePerm => "http://iec.ch/TC57/CIM100#UnitSymbol.onePerm",
            UnitSymbol::m3Perkg => "http://iec.ch/TC57/CIM100#UnitSymbol.m3Perkg",
            UnitSymbol::Pas => "http://iec.ch/TC57/CIM100#UnitSymbol.Pas",
            UnitSymbol::Nm => "http://iec.ch/TC57/CIM100#UnitSymbol.Nm",
            UnitSymbol::NPerm => "http://iec.ch/TC57/CIM100#UnitSymbol.NPerm",
            UnitSymbol::radPers2 => "http://iec.ch/TC57/CIM100#UnitSymbol.radPers2",
            UnitSymbol::JPerm3 => "http://iec.ch/TC57/CIM100#UnitSymbol.JPerm3",
            UnitSymbol::VPerm => "http://iec.ch/TC57/CIM100#UnitSymbol.VPerm",
            UnitSymbol::CPerm3 => "http://iec.ch/TC57/CIM100#UnitSymbol.CPerm3",
            UnitSymbol::CPerm2 => "http://iec.ch/TC57/CIM100#UnitSymbol.CPerm2",
            UnitSymbol::FPerm => "http://iec.ch/TC57/CIM100#UnitSymbol.FPerm",
            UnitSymbol::HPerm => "http://iec.ch/TC57/CIM100#UnitSymbol.HPerm",
            UnitSymbol::JPermol => "http://iec.ch/TC57/CIM100#UnitSymbol.JPermol",
            UnitSymbol::JPermolK => "http://iec.ch/TC57/CIM100#UnitSymbol.JPermolK",
            UnitSymbol::CPerkg => "http://iec.ch/TC57/CIM100#UnitSymbol.CPerkg",
            UnitSymbol::GyPers => "http://iec.ch/TC57/CIM100#UnitSymbol.GyPers",
            UnitSymbol::WPersr => "http://iec.ch/TC57/CIM100#UnitSymbol.WPersr",
            UnitSymbol::WPerm2sr => "http://iec.ch/TC57/CIM100#UnitSymbol.WPerm2sr",
            UnitSymbol::katPerm3 => "http://iec.ch/TC57/CIM100#UnitSymbol.katPerm3",
            UnitSymbol::d => "http://iec.ch/TC57/CIM100#UnitSymbol.d",
            UnitSymbol::anglemin => "http://iec.ch/TC57/CIM100#UnitSymbol.anglemin",
            UnitSymbol::anglesec => "http://iec.ch/TC57/CIM100#UnitSymbol.anglesec",
            UnitSymbol::ha => "http://iec.ch/TC57/CIM100#UnitSymbol.ha",
            UnitSymbol::tonne => "http://iec.ch/TC57/CIM100#UnitSymbol.tonne",
            UnitSymbol::bar => "http://iec.ch/TC57/CIM100#UnitSymbol.bar",
            UnitSymbol::mmHg => "http://iec.ch/TC57/CIM100#UnitSymbol.mmHg",
            UnitSymbol::M => "http://iec.ch/TC57/CIM100#UnitSymbol.M",
            UnitSymbol::kn => "http://iec.ch/TC57/CIM100#UnitSymbol.kn",
            UnitSymbol::Mx => "http://iec.ch/TC57/CIM100#UnitSymbol.Mx",
            UnitSymbol::G => "http://iec.ch/TC57/CIM100#UnitSymbol.G",
            UnitSymbol::Oe => "http://iec.ch/TC57/CIM100#UnitSymbol.Oe",
            UnitSymbol::Vh => "http://iec.ch/TC57/CIM100#UnitSymbol.Vh",
            UnitSymbol::WPerA => "http://iec.ch/TC57/CIM100#UnitSymbol.WPerA",
            UnitSymbol::onePerHz => "http://iec.ch/TC57/CIM100#UnitSymbol.onePerHz",
            UnitSymbol::VPerVAr => "http://iec.ch/TC57/CIM100#UnitSymbol.VPerVAr",
            UnitSymbol::ohmPerm => "http://iec.ch/TC57/CIM100#UnitSymbol.ohmPerm",
            UnitSymbol::kgPerJ => "http://iec.ch/TC57/CIM100#UnitSymbol.kgPerJ",
            UnitSymbol::JPers => "http://iec.ch/TC57/CIM100#UnitSymbol.JPers",
        }
    }
}
