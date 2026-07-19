/// Monetary currencies. ISO 4217 standard including 3-character currency code.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Currency {
    /// United Arab Emirates dirham.
    AED,
    /// Afghan afghani.
    AFN,
    /// Albanian lek.
    ALL,
    /// Armenian dram.
    AMD,
    /// Netherlands Antillean guilder.
    ANG,
    /// Angolan kwanza.
    AOA,
    /// Argentine peso.
    ARS,
    /// Australian dollar.
    AUD,
    /// Aruban florin.
    AWG,
    /// Azerbaijani manat.
    AZN,
    /// Bosnia and Herzegovina convertible mark.
    BAM,
    /// Barbados dollar.
    BBD,
    /// Bangladeshi taka.
    BDT,
    /// Bulgarian lev.
    BGN,
    /// Bahraini dinar.
    BHD,
    /// Burundian franc.
    BIF,
    /// Bermudian dollar (customarily known as Bermuda dollar).
    BMD,
    /// Brunei dollar.
    BND,
    /// Boliviano.
    BOB,
    /// Bolivian Mvdol (funds code).
    BOV,
    /// Brazilian real.
    BRL,
    /// Bahamian dollar.
    BSD,
    /// Bhutanese ngultrum.
    BTN,
    /// Botswana pula.
    BWP,
    /// Belarusian ruble.
    BYR,
    /// Belize dollar.
    BZD,
    /// Canadian dollar.
    CAD,
    /// Congolese franc.
    CDF,
    /// Swiss franc.
    CHF,
    /// Unidad de Fomento (funds code), Chile.
    CLF,
    /// Chilean peso.
    CLP,
    /// Chinese yuan.
    CNY,
    /// Colombian peso.
    COP,
    /// Unidad de Valor Real.
    COU,
    /// Costa Rican colon.
    CRC,
    /// Cuban convertible peso.
    CUC,
    /// Cuban peso.
    CUP,
    /// Cape Verde escudo.
    CVE,
    /// Czech koruna.
    CZK,
    /// Djiboutian franc.
    DJF,
    /// Danish krone.
    DKK,
    /// Dominican peso.
    DOP,
    /// Algerian dinar.
    DZD,
    /// Estonian kroon.
    EEK,
    /// Egyptian pound.
    EGP,
    /// Eritrean nakfa.
    ERN,
    /// Ethiopian birr.
    ETB,
    /// Euro.
    EUR,
    /// Fiji dollar.
    FJD,
    /// Falkland Islands pound.
    FKP,
    /// Pound sterling.
    GBP,
    /// Georgian lari.
    GEL,
    /// Ghanaian cedi.
    GHS,
    /// Gibraltar pound.
    GIP,
    /// Gambian dalasi.
    GMD,
    /// Guinean franc.
    GNF,
    /// Guatemalan quetzal.
    GTQ,
    /// Guyanese dollar.
    GYD,
    /// Hong Kong dollar.
    HKD,
    /// Honduran lempira.
    HNL,
    /// Croatian kuna.
    HRK,
    /// Haitian gourde.
    HTG,
    /// Hungarian forint.
    HUF,
    /// Indonesian rupiah.
    IDR,
    /// Israeli new sheqel.
    ILS,
    /// Indian rupee.
    INR,
    /// Iraqi dinar.
    IQD,
    /// Iranian rial.
    IRR,
    /// Icelandic króna.
    ISK,
    /// Jamaican dollar.
    JMD,
    /// Jordanian dinar.
    JOD,
    /// Japanese yen.
    JPY,
    /// Kenyan shilling.
    KES,
    /// Kyrgyzstani som.
    KGS,
    /// Cambodian riel.
    KHR,
    /// Comoro franc.
    KMF,
    /// North Korean won.
    KPW,
    /// South Korean won.
    KRW,
    /// Kuwaiti dinar.
    KWD,
    /// Cayman Islands dollar.
    KYD,
    /// Kazakhstani tenge.
    KZT,
    /// Lao kip.
    LAK,
    /// Lebanese pound.
    LBP,
    /// Sri Lanka rupee.
    LKR,
    /// Liberian dollar.
    LRD,
    /// Lesotho loti.
    LSL,
    /// Lithuanian litas.
    LTL,
    /// Latvian lats.
    LVL,
    /// Libyan dinar.
    LYD,
    /// Moroccan dirham.
    MAD,
    /// Moldovan leu.
    MDL,
    /// Malagasy ariary.
    MGA,
    /// Macedonian denar.
    MKD,
    /// Myanma kyat.
    MMK,
    /// Mongolian tugrik.
    MNT,
    /// Macanese pataca.
    MOP,
    /// Mauritanian ouguiya.
    MRO,
    /// Mauritian rupee.
    MUR,
    /// Maldivian rufiyaa.
    MVR,
    /// Malawian kwacha.
    MWK,
    /// Mexican peso.
    MXN,
    /// Malaysian ringgit.
    MYR,
    /// Mozambican metical.
    MZN,
    /// Namibian dollar.
    NAD,
    /// Nigerian naira.
    NGN,
    /// Cordoba oro.
    NIO,
    /// Norwegian krone.
    NOK,
    /// Nepalese rupee.
    NPR,
    /// New Zealand dollar.
    NZD,
    /// Omani rial.
    OMR,
    /// Panamanian balboa.
    PAB,
    /// Peruvian nuevo sol.
    PEN,
    /// Papua New Guinean kina.
    PGK,
    /// Philippine peso.
    PHP,
    /// Pakistani rupee.
    PKR,
    /// Polish zloty.
    PLN,
    /// Paraguayan guaraní.
    PYG,
    /// Qatari rial.
    QAR,
    /// Romanian new leu.
    RON,
    /// Serbian dinar.
    RSD,
    /// Russian rouble.
    RUB,
    /// Rwandan franc.
    RWF,
    /// Saudi riyal.
    SAR,
    /// Solomon Islands dollar.
    SBD,
    /// Seychelles rupee.
    SCR,
    /// Sudanese pound.
    SDG,
    /// Swedish krona/kronor.
    SEK,
    /// Singapore dollar.
    SGD,
    /// Saint Helena pound.
    SHP,
    /// Sierra Leonean leone.
    SLL,
    /// Somali shilling.
    SOS,
    /// Surinamese dollar.
    SRD,
    /// São Tomé and Príncipe dobra.
    STD,
    /// Syrian pound.
    SYP,
    /// Lilangeni.
    SZL,
    /// Thai baht.
    THB,
    /// Tajikistani somoni.
    TJS,
    /// Turkmenistani manat.
    TMT,
    /// Tunisian dinar.
    TND,
    /// Tongan pa'anga.
    TOP,
    /// Turkish lira.
    TRY,
    /// Trinidad and Tobago dollar.
    TTD,
    /// New Taiwan dollar.
    TWD,
    /// Tanzanian shilling.
    TZS,
    /// Ukrainian hryvnia.
    UAH,
    /// Ugandan shilling.
    UGX,
    /// United States dollar.
    USD,
    /// Uruguayan peso.
    UYU,
    /// Uzbekistan som.
    UZS,
    /// Venezuelan bolívar fuerte.
    VEF,
    /// Vietnamese Dong.
    VND,
    /// Vanuatu vatu.
    VUV,
    /// Samoan tala.
    WST,
    /// CFA franc BEAC.
    XAF,
    /// East Caribbean dollar.
    XCD,
    /// CFA Franc BCEAO.
    XOF,
    /// CFP franc.
    XPF,
    /// Yemeni rial.
    YER,
    /// South African rand.
    ZAR,
    /// Zambian kwacha.
    ZMK,
    /// Zimbabwe dollar.
    ZWL,
}

impl Currency {
    pub fn uri(&self) -> &'static str {
        match self {
            Currency::AED => "http://iec.ch/TC57/CIM100#Currency.AED",
            Currency::AFN => "http://iec.ch/TC57/CIM100#Currency.AFN",
            Currency::ALL => "http://iec.ch/TC57/CIM100#Currency.ALL",
            Currency::AMD => "http://iec.ch/TC57/CIM100#Currency.AMD",
            Currency::ANG => "http://iec.ch/TC57/CIM100#Currency.ANG",
            Currency::AOA => "http://iec.ch/TC57/CIM100#Currency.AOA",
            Currency::ARS => "http://iec.ch/TC57/CIM100#Currency.ARS",
            Currency::AUD => "http://iec.ch/TC57/CIM100#Currency.AUD",
            Currency::AWG => "http://iec.ch/TC57/CIM100#Currency.AWG",
            Currency::AZN => "http://iec.ch/TC57/CIM100#Currency.AZN",
            Currency::BAM => "http://iec.ch/TC57/CIM100#Currency.BAM",
            Currency::BBD => "http://iec.ch/TC57/CIM100#Currency.BBD",
            Currency::BDT => "http://iec.ch/TC57/CIM100#Currency.BDT",
            Currency::BGN => "http://iec.ch/TC57/CIM100#Currency.BGN",
            Currency::BHD => "http://iec.ch/TC57/CIM100#Currency.BHD",
            Currency::BIF => "http://iec.ch/TC57/CIM100#Currency.BIF",
            Currency::BMD => "http://iec.ch/TC57/CIM100#Currency.BMD",
            Currency::BND => "http://iec.ch/TC57/CIM100#Currency.BND",
            Currency::BOB => "http://iec.ch/TC57/CIM100#Currency.BOB",
            Currency::BOV => "http://iec.ch/TC57/CIM100#Currency.BOV",
            Currency::BRL => "http://iec.ch/TC57/CIM100#Currency.BRL",
            Currency::BSD => "http://iec.ch/TC57/CIM100#Currency.BSD",
            Currency::BTN => "http://iec.ch/TC57/CIM100#Currency.BTN",
            Currency::BWP => "http://iec.ch/TC57/CIM100#Currency.BWP",
            Currency::BYR => "http://iec.ch/TC57/CIM100#Currency.BYR",
            Currency::BZD => "http://iec.ch/TC57/CIM100#Currency.BZD",
            Currency::CAD => "http://iec.ch/TC57/CIM100#Currency.CAD",
            Currency::CDF => "http://iec.ch/TC57/CIM100#Currency.CDF",
            Currency::CHF => "http://iec.ch/TC57/CIM100#Currency.CHF",
            Currency::CLF => "http://iec.ch/TC57/CIM100#Currency.CLF",
            Currency::CLP => "http://iec.ch/TC57/CIM100#Currency.CLP",
            Currency::CNY => "http://iec.ch/TC57/CIM100#Currency.CNY",
            Currency::COP => "http://iec.ch/TC57/CIM100#Currency.COP",
            Currency::COU => "http://iec.ch/TC57/CIM100#Currency.COU",
            Currency::CRC => "http://iec.ch/TC57/CIM100#Currency.CRC",
            Currency::CUC => "http://iec.ch/TC57/CIM100#Currency.CUC",
            Currency::CUP => "http://iec.ch/TC57/CIM100#Currency.CUP",
            Currency::CVE => "http://iec.ch/TC57/CIM100#Currency.CVE",
            Currency::CZK => "http://iec.ch/TC57/CIM100#Currency.CZK",
            Currency::DJF => "http://iec.ch/TC57/CIM100#Currency.DJF",
            Currency::DKK => "http://iec.ch/TC57/CIM100#Currency.DKK",
            Currency::DOP => "http://iec.ch/TC57/CIM100#Currency.DOP",
            Currency::DZD => "http://iec.ch/TC57/CIM100#Currency.DZD",
            Currency::EEK => "http://iec.ch/TC57/CIM100#Currency.EEK",
            Currency::EGP => "http://iec.ch/TC57/CIM100#Currency.EGP",
            Currency::ERN => "http://iec.ch/TC57/CIM100#Currency.ERN",
            Currency::ETB => "http://iec.ch/TC57/CIM100#Currency.ETB",
            Currency::EUR => "http://iec.ch/TC57/CIM100#Currency.EUR",
            Currency::FJD => "http://iec.ch/TC57/CIM100#Currency.FJD",
            Currency::FKP => "http://iec.ch/TC57/CIM100#Currency.FKP",
            Currency::GBP => "http://iec.ch/TC57/CIM100#Currency.GBP",
            Currency::GEL => "http://iec.ch/TC57/CIM100#Currency.GEL",
            Currency::GHS => "http://iec.ch/TC57/CIM100#Currency.GHS",
            Currency::GIP => "http://iec.ch/TC57/CIM100#Currency.GIP",
            Currency::GMD => "http://iec.ch/TC57/CIM100#Currency.GMD",
            Currency::GNF => "http://iec.ch/TC57/CIM100#Currency.GNF",
            Currency::GTQ => "http://iec.ch/TC57/CIM100#Currency.GTQ",
            Currency::GYD => "http://iec.ch/TC57/CIM100#Currency.GYD",
            Currency::HKD => "http://iec.ch/TC57/CIM100#Currency.HKD",
            Currency::HNL => "http://iec.ch/TC57/CIM100#Currency.HNL",
            Currency::HRK => "http://iec.ch/TC57/CIM100#Currency.HRK",
            Currency::HTG => "http://iec.ch/TC57/CIM100#Currency.HTG",
            Currency::HUF => "http://iec.ch/TC57/CIM100#Currency.HUF",
            Currency::IDR => "http://iec.ch/TC57/CIM100#Currency.IDR",
            Currency::ILS => "http://iec.ch/TC57/CIM100#Currency.ILS",
            Currency::INR => "http://iec.ch/TC57/CIM100#Currency.INR",
            Currency::IQD => "http://iec.ch/TC57/CIM100#Currency.IQD",
            Currency::IRR => "http://iec.ch/TC57/CIM100#Currency.IRR",
            Currency::ISK => "http://iec.ch/TC57/CIM100#Currency.ISK",
            Currency::JMD => "http://iec.ch/TC57/CIM100#Currency.JMD",
            Currency::JOD => "http://iec.ch/TC57/CIM100#Currency.JOD",
            Currency::JPY => "http://iec.ch/TC57/CIM100#Currency.JPY",
            Currency::KES => "http://iec.ch/TC57/CIM100#Currency.KES",
            Currency::KGS => "http://iec.ch/TC57/CIM100#Currency.KGS",
            Currency::KHR => "http://iec.ch/TC57/CIM100#Currency.KHR",
            Currency::KMF => "http://iec.ch/TC57/CIM100#Currency.KMF",
            Currency::KPW => "http://iec.ch/TC57/CIM100#Currency.KPW",
            Currency::KRW => "http://iec.ch/TC57/CIM100#Currency.KRW",
            Currency::KWD => "http://iec.ch/TC57/CIM100#Currency.KWD",
            Currency::KYD => "http://iec.ch/TC57/CIM100#Currency.KYD",
            Currency::KZT => "http://iec.ch/TC57/CIM100#Currency.KZT",
            Currency::LAK => "http://iec.ch/TC57/CIM100#Currency.LAK",
            Currency::LBP => "http://iec.ch/TC57/CIM100#Currency.LBP",
            Currency::LKR => "http://iec.ch/TC57/CIM100#Currency.LKR",
            Currency::LRD => "http://iec.ch/TC57/CIM100#Currency.LRD",
            Currency::LSL => "http://iec.ch/TC57/CIM100#Currency.LSL",
            Currency::LTL => "http://iec.ch/TC57/CIM100#Currency.LTL",
            Currency::LVL => "http://iec.ch/TC57/CIM100#Currency.LVL",
            Currency::LYD => "http://iec.ch/TC57/CIM100#Currency.LYD",
            Currency::MAD => "http://iec.ch/TC57/CIM100#Currency.MAD",
            Currency::MDL => "http://iec.ch/TC57/CIM100#Currency.MDL",
            Currency::MGA => "http://iec.ch/TC57/CIM100#Currency.MGA",
            Currency::MKD => "http://iec.ch/TC57/CIM100#Currency.MKD",
            Currency::MMK => "http://iec.ch/TC57/CIM100#Currency.MMK",
            Currency::MNT => "http://iec.ch/TC57/CIM100#Currency.MNT",
            Currency::MOP => "http://iec.ch/TC57/CIM100#Currency.MOP",
            Currency::MRO => "http://iec.ch/TC57/CIM100#Currency.MRO",
            Currency::MUR => "http://iec.ch/TC57/CIM100#Currency.MUR",
            Currency::MVR => "http://iec.ch/TC57/CIM100#Currency.MVR",
            Currency::MWK => "http://iec.ch/TC57/CIM100#Currency.MWK",
            Currency::MXN => "http://iec.ch/TC57/CIM100#Currency.MXN",
            Currency::MYR => "http://iec.ch/TC57/CIM100#Currency.MYR",
            Currency::MZN => "http://iec.ch/TC57/CIM100#Currency.MZN",
            Currency::NAD => "http://iec.ch/TC57/CIM100#Currency.NAD",
            Currency::NGN => "http://iec.ch/TC57/CIM100#Currency.NGN",
            Currency::NIO => "http://iec.ch/TC57/CIM100#Currency.NIO",
            Currency::NOK => "http://iec.ch/TC57/CIM100#Currency.NOK",
            Currency::NPR => "http://iec.ch/TC57/CIM100#Currency.NPR",
            Currency::NZD => "http://iec.ch/TC57/CIM100#Currency.NZD",
            Currency::OMR => "http://iec.ch/TC57/CIM100#Currency.OMR",
            Currency::PAB => "http://iec.ch/TC57/CIM100#Currency.PAB",
            Currency::PEN => "http://iec.ch/TC57/CIM100#Currency.PEN",
            Currency::PGK => "http://iec.ch/TC57/CIM100#Currency.PGK",
            Currency::PHP => "http://iec.ch/TC57/CIM100#Currency.PHP",
            Currency::PKR => "http://iec.ch/TC57/CIM100#Currency.PKR",
            Currency::PLN => "http://iec.ch/TC57/CIM100#Currency.PLN",
            Currency::PYG => "http://iec.ch/TC57/CIM100#Currency.PYG",
            Currency::QAR => "http://iec.ch/TC57/CIM100#Currency.QAR",
            Currency::RON => "http://iec.ch/TC57/CIM100#Currency.RON",
            Currency::RSD => "http://iec.ch/TC57/CIM100#Currency.RSD",
            Currency::RUB => "http://iec.ch/TC57/CIM100#Currency.RUB",
            Currency::RWF => "http://iec.ch/TC57/CIM100#Currency.RWF",
            Currency::SAR => "http://iec.ch/TC57/CIM100#Currency.SAR",
            Currency::SBD => "http://iec.ch/TC57/CIM100#Currency.SBD",
            Currency::SCR => "http://iec.ch/TC57/CIM100#Currency.SCR",
            Currency::SDG => "http://iec.ch/TC57/CIM100#Currency.SDG",
            Currency::SEK => "http://iec.ch/TC57/CIM100#Currency.SEK",
            Currency::SGD => "http://iec.ch/TC57/CIM100#Currency.SGD",
            Currency::SHP => "http://iec.ch/TC57/CIM100#Currency.SHP",
            Currency::SLL => "http://iec.ch/TC57/CIM100#Currency.SLL",
            Currency::SOS => "http://iec.ch/TC57/CIM100#Currency.SOS",
            Currency::SRD => "http://iec.ch/TC57/CIM100#Currency.SRD",
            Currency::STD => "http://iec.ch/TC57/CIM100#Currency.STD",
            Currency::SYP => "http://iec.ch/TC57/CIM100#Currency.SYP",
            Currency::SZL => "http://iec.ch/TC57/CIM100#Currency.SZL",
            Currency::THB => "http://iec.ch/TC57/CIM100#Currency.THB",
            Currency::TJS => "http://iec.ch/TC57/CIM100#Currency.TJS",
            Currency::TMT => "http://iec.ch/TC57/CIM100#Currency.TMT",
            Currency::TND => "http://iec.ch/TC57/CIM100#Currency.TND",
            Currency::TOP => "http://iec.ch/TC57/CIM100#Currency.TOP",
            Currency::TRY => "http://iec.ch/TC57/CIM100#Currency.TRY",
            Currency::TTD => "http://iec.ch/TC57/CIM100#Currency.TTD",
            Currency::TWD => "http://iec.ch/TC57/CIM100#Currency.TWD",
            Currency::TZS => "http://iec.ch/TC57/CIM100#Currency.TZS",
            Currency::UAH => "http://iec.ch/TC57/CIM100#Currency.UAH",
            Currency::UGX => "http://iec.ch/TC57/CIM100#Currency.UGX",
            Currency::USD => "http://iec.ch/TC57/CIM100#Currency.USD",
            Currency::UYU => "http://iec.ch/TC57/CIM100#Currency.UYU",
            Currency::UZS => "http://iec.ch/TC57/CIM100#Currency.UZS",
            Currency::VEF => "http://iec.ch/TC57/CIM100#Currency.VEF",
            Currency::VND => "http://iec.ch/TC57/CIM100#Currency.VND",
            Currency::VUV => "http://iec.ch/TC57/CIM100#Currency.VUV",
            Currency::WST => "http://iec.ch/TC57/CIM100#Currency.WST",
            Currency::XAF => "http://iec.ch/TC57/CIM100#Currency.XAF",
            Currency::XCD => "http://iec.ch/TC57/CIM100#Currency.XCD",
            Currency::XOF => "http://iec.ch/TC57/CIM100#Currency.XOF",
            Currency::XPF => "http://iec.ch/TC57/CIM100#Currency.XPF",
            Currency::YER => "http://iec.ch/TC57/CIM100#Currency.YER",
            Currency::ZAR => "http://iec.ch/TC57/CIM100#Currency.ZAR",
            Currency::ZMK => "http://iec.ch/TC57/CIM100#Currency.ZMK",
            Currency::ZWL => "http://iec.ch/TC57/CIM100#Currency.ZWL",
        }
    }
}
