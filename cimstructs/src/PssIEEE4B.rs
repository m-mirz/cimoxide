/// IEEE 421.5-2005 type PSS4B power system stabilizer. The PSS4B model represents a structure based on multiple working frequency bands. Three separate bands, respectively dedicated to the low-, intermediate- and high-frequency modes of oscillations, are used in this delta omega (speed input) PSS. There is an error in the in IEEE 421.5-2005 PSS4B model: the Pe input should read -Pe. This implies that the input Pe needs to be multiplied by -1. Reference: IEEE 4B 421.5-2005, 8.4. Parameter details: This model has 2 input signals. They have the following fixed types (expressed in terms of InputSignalKind values): the first one is of rotorAngleFrequencyDeviation type and the second one is of generatorElectricalPower type.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PssIEEE4B {
    #[serde(flatten)]
    pub base: super::PowerSystemStabilizerDynamics,
    /// Notch filter 1 (high-frequency band): three dB bandwidth (Bwi).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bwh1: Option<f64>,
    /// Notch filter 2 (high-frequency band): three dB bandwidth (Bwi).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bwh2: Option<f64>,
    /// Notch filter 1 (low-frequency band): three dB bandwidth (Bwi).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bwl1: Option<f64>,
    /// Notch filter 2 (low-frequency band): three dB bandwidth (Bwi).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bwl2: Option<f64>,
    /// High band gain (KH). Typical value = 120.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kh: Option<f64>,
    /// High band differential filter gain (KH1). Typical value = 66.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kh1: Option<f64>,
    /// High band first lead-lag blocks coefficient (KH11). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kh11: Option<f64>,
    /// High band first lead-lag blocks coefficient (KH17). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kh17: Option<f64>,
    /// High band differential filter gain (KH2). Typical value = 66.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kh2: Option<f64>,
    /// Intermediate band gain (KI). Typical value = 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ki: Option<f64>,
    /// Intermediate band differential filter gain (KI1). Typical value = 66.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ki1: Option<f64>,
    /// Intermediate band first lead-lag blocks coefficient (KI11). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ki11: Option<f64>,
    /// Intermediate band first lead-lag blocks coefficient (KI17). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ki17: Option<f64>,
    /// Intermediate band differential filter gain (KI2). Typical value = 66.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ki2: Option<f64>,
    /// Low band gain (KL). Typical value = 7.5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kl: Option<f64>,
    /// Low band differential filter gain (KL1). Typical value = 66.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kl1: Option<f64>,
    /// Low band first lead-lag blocks coefficient (KL11). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kl11: Option<f64>,
    /// Low band first lead-lag blocks coefficient (KL17). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kl17: Option<f64>,
    /// Low band differential filter gain (KL2). Typical value = 66.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kl2: Option<f64>,
    /// Notch filter 1 (high-frequency band): filter frequency (omegani).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub omeganh1: Option<f64>,
    /// Notch filter 2 (high-frequency band): filter frequency (omegani).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub omeganh2: Option<f64>,
    /// Notch filter 1 (low-frequency band): filter frequency (omegani).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub omeganl1: Option<f64>,
    /// Notch filter 2 (low-frequency band): filter frequency (omegani).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub omeganl2: Option<f64>,
    /// High band time constant (TH1) (>= 0). Typical value = 0,01513.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub th1: Option<f64>,
    /// High band time constant (TH10) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub th10: Option<f64>,
    /// High band time constant (TH11) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub th11: Option<f64>,
    /// High band time constant (TH12) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub th12: Option<f64>,
    /// High band time constant (TH2) (>= 0). Typical value = 0,01816.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub th2: Option<f64>,
    /// High band time constant (TH3) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub th3: Option<f64>,
    /// High band time constant (TH4) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub th4: Option<f64>,
    /// High band time constant (TH5) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub th5: Option<f64>,
    /// High band time constant (TH6) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub th6: Option<f64>,
    /// High band time constant (TH7) (>= 0). Typical value = 0,01816.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub th7: Option<f64>,
    /// High band time constant (TH8) (>= 0). Typical value = 0,02179.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub th8: Option<f64>,
    /// High band time constant (TH9) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub th9: Option<f64>,
    /// Intermediate band time constant (TI1) (>= 0). Typical value = 0,173.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ti1: Option<f64>,
    /// Intermediate band time constant (TI10) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ti10: Option<f64>,
    /// Intermediate band time constant (TI11) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ti11: Option<f64>,
    /// Intermediate band time constant (TI12) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ti12: Option<f64>,
    /// Intermediate band time constant (TI2) (>= 0). Typical value = 0,2075.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ti2: Option<f64>,
    /// Intermediate band time constant (TI3) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ti3: Option<f64>,
    /// Intermediate band time constant (TI4) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ti4: Option<f64>,
    /// Intermediate band time constant (TI5) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ti5: Option<f64>,
    /// Intermediate band time constant (TI6) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ti6: Option<f64>,
    /// Intermediate band time constant (TI7) (>= 0). Typical value = 0,2075.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ti7: Option<f64>,
    /// Intermediate band time constant (TI8) (>= 0). Typical value = 0,2491.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ti8: Option<f64>,
    /// Intermediate band time constant (TI9) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ti9: Option<f64>,
    /// Low band time constant (TL1) (>= 0). Typical value = 1,73.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tl1: Option<f64>,
    /// Low band time constant (TL10) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tl10: Option<f64>,
    /// Low band time constant (TL11) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tl11: Option<f64>,
    /// Low band time constant (TL12) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tl12: Option<f64>,
    /// Low band time constant (TL2) (>= 0). Typical value = 2,075.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tl2: Option<f64>,
    /// Low band time constant (TL3) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tl3: Option<f64>,
    /// Low band time constant (TL4) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tl4: Option<f64>,
    /// Low band time constant (TL5) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tl5: Option<f64>,
    /// Low band time constant (TL6) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tl6: Option<f64>,
    /// Low band time constant (TL7) (>= 0). Typical value = 2,075.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tl7: Option<f64>,
    /// Low band time constant (TL8) (>= 0). Typical value = 2,491.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tl8: Option<f64>,
    /// Low band time constant (TL9) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tl9: Option<f64>,
    /// High band output maximum limit (VHmax) (> PssIEEE4B.vhmin). Typical value = 0,6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vhmax: Option<f64>,
    /// High band output minimum limit (VHmin) (< PssIEEE4V.vhmax). Typical value = -0,6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vhmin: Option<f64>,
    /// Intermediate band output maximum limit (VImax) (> PssIEEE4B.vimin). Typical value = 0,6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vimax: Option<f64>,
    /// Intermediate band output minimum limit (VImin) (< PssIEEE4B.vimax). Typical value = -0,6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vimin: Option<f64>,
    /// Low band output maximum limit (VLmax) (> PssIEEE4B.vlmin). Typical value = 0,075.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlmax: Option<f64>,
    /// Low band output minimum limit (VLmin) (< PssIEEE4B.vlmax). Typical value = -0,075.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlmin: Option<f64>,
    /// PSS output maximum limit (VSTmax) (> PssIEEE4B.vstmin). Typical value = 0,15.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vstmax: Option<f64>,
    /// PSS output minimum limit (VSTmin) (< PssIEEE4B.vstmax). Typical value = -0,15.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vstmin: Option<f64>,
}
impl crate::base::CimElement for PssIEEE4B {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "PssIEEE4B" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "PssIEEE4B".to_string();
        if let Some(v) = self.bwh1 {
            block.fields.insert("PssIEEE4B.bwh1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.bwh2 {
            block.fields.insert("PssIEEE4B.bwh2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.bwl1 {
            block.fields.insert("PssIEEE4B.bwl1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.bwl2 {
            block.fields.insert("PssIEEE4B.bwl2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kh {
            block.fields.insert("PssIEEE4B.kh".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kh1 {
            block.fields.insert("PssIEEE4B.kh1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kh11 {
            block.fields.insert("PssIEEE4B.kh11".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kh17 {
            block.fields.insert("PssIEEE4B.kh17".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kh2 {
            block.fields.insert("PssIEEE4B.kh2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ki {
            block.fields.insert("PssIEEE4B.ki".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ki1 {
            block.fields.insert("PssIEEE4B.ki1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ki11 {
            block.fields.insert("PssIEEE4B.ki11".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ki17 {
            block.fields.insert("PssIEEE4B.ki17".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ki2 {
            block.fields.insert("PssIEEE4B.ki2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kl {
            block.fields.insert("PssIEEE4B.kl".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kl1 {
            block.fields.insert("PssIEEE4B.kl1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kl11 {
            block.fields.insert("PssIEEE4B.kl11".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kl17 {
            block.fields.insert("PssIEEE4B.kl17".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kl2 {
            block.fields.insert("PssIEEE4B.kl2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.omeganh1 {
            block.fields.insert("PssIEEE4B.omeganh1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.omeganh2 {
            block.fields.insert("PssIEEE4B.omeganh2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.omeganl1 {
            block.fields.insert("PssIEEE4B.omeganl1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.omeganl2 {
            block.fields.insert("PssIEEE4B.omeganl2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.th1 {
            block.fields.insert("PssIEEE4B.th1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.th10 {
            block.fields.insert("PssIEEE4B.th10".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.th11 {
            block.fields.insert("PssIEEE4B.th11".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.th12 {
            block.fields.insert("PssIEEE4B.th12".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.th2 {
            block.fields.insert("PssIEEE4B.th2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.th3 {
            block.fields.insert("PssIEEE4B.th3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.th4 {
            block.fields.insert("PssIEEE4B.th4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.th5 {
            block.fields.insert("PssIEEE4B.th5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.th6 {
            block.fields.insert("PssIEEE4B.th6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.th7 {
            block.fields.insert("PssIEEE4B.th7".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.th8 {
            block.fields.insert("PssIEEE4B.th8".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.th9 {
            block.fields.insert("PssIEEE4B.th9".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ti1 {
            block.fields.insert("PssIEEE4B.ti1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ti10 {
            block.fields.insert("PssIEEE4B.ti10".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ti11 {
            block.fields.insert("PssIEEE4B.ti11".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ti12 {
            block.fields.insert("PssIEEE4B.ti12".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ti2 {
            block.fields.insert("PssIEEE4B.ti2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ti3 {
            block.fields.insert("PssIEEE4B.ti3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ti4 {
            block.fields.insert("PssIEEE4B.ti4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ti5 {
            block.fields.insert("PssIEEE4B.ti5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ti6 {
            block.fields.insert("PssIEEE4B.ti6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ti7 {
            block.fields.insert("PssIEEE4B.ti7".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ti8 {
            block.fields.insert("PssIEEE4B.ti8".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ti9 {
            block.fields.insert("PssIEEE4B.ti9".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tl1 {
            block.fields.insert("PssIEEE4B.tl1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tl10 {
            block.fields.insert("PssIEEE4B.tl10".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tl11 {
            block.fields.insert("PssIEEE4B.tl11".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tl12 {
            block.fields.insert("PssIEEE4B.tl12".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tl2 {
            block.fields.insert("PssIEEE4B.tl2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tl3 {
            block.fields.insert("PssIEEE4B.tl3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tl4 {
            block.fields.insert("PssIEEE4B.tl4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tl5 {
            block.fields.insert("PssIEEE4B.tl5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tl6 {
            block.fields.insert("PssIEEE4B.tl6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tl7 {
            block.fields.insert("PssIEEE4B.tl7".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tl8 {
            block.fields.insert("PssIEEE4B.tl8".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tl9 {
            block.fields.insert("PssIEEE4B.tl9".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vhmax {
            block.fields.insert("PssIEEE4B.vhmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vhmin {
            block.fields.insert("PssIEEE4B.vhmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vimax {
            block.fields.insert("PssIEEE4B.vimax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vimin {
            block.fields.insert("PssIEEE4B.vimin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vlmax {
            block.fields.insert("PssIEEE4B.vlmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vlmin {
            block.fields.insert("PssIEEE4B.vlmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vstmax {
            block.fields.insert("PssIEEE4B.vstmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vstmin {
            block.fields.insert("PssIEEE4B.vstmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl PssIEEE4B {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "PssIEEE4B.bwh1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bwh1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bwh1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.bwh2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bwh2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bwh2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.bwl1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bwl1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bwl1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.bwl2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bwl2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bwl2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.kh" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kh = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kh = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.kh1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kh1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kh1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.kh11" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kh11 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kh11 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.kh17" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kh17 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kh17 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.kh2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kh2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kh2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.ki" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.ki1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ki1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ki1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.ki11" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ki11 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ki11 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.ki17" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ki17 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ki17 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.ki2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ki2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ki2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.kl" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kl = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kl = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.kl1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kl1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kl1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.kl11" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kl11 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kl11 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.kl17" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kl17 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kl17 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.kl2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kl2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kl2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.omeganh1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.omeganh1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.omeganh1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.omeganh2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.omeganh2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.omeganh2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.omeganl1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.omeganl1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.omeganl1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.omeganl2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.omeganl2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.omeganl2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.th1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.th1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.th1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.th10" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.th10 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.th10 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.th11" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.th11 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.th11 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.th12" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.th12 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.th12 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.th2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.th2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.th2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.th3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.th3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.th3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.th4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.th4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.th4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.th5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.th5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.th5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.th6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.th6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.th6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.th7" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.th7 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.th7 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.th8" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.th8 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.th8 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.th9" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.th9 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.th9 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.ti1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ti1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ti1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.ti10" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ti10 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ti10 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.ti11" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ti11 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ti11 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.ti12" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ti12 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ti12 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.ti2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ti2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ti2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.ti3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ti3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ti3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.ti4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ti4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ti4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.ti5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ti5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ti5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.ti6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ti6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ti6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.ti7" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ti7 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ti7 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.ti8" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ti8 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ti8 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.ti9" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ti9 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ti9 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.tl1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tl1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tl1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.tl10" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tl10 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tl10 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.tl11" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tl11 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tl11 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.tl12" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tl12 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tl12 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.tl2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tl2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tl2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.tl3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tl3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tl3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.tl4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tl4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tl4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.tl5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tl5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tl5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.tl6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tl6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tl6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.tl7" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tl7 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tl7 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.tl8" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tl8 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tl8 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.tl9" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tl9 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tl9 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.vhmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vhmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vhmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.vhmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vhmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vhmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.vimax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vimax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vimax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.vimin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vimin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vimin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.vlmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vlmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vlmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.vlmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vlmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vlmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.vstmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vstmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vstmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE4B.vstmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vstmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vstmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PowerSystemStabilizerDynamics.ExcitationSystemDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.excitation_system_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "DynamicsFunctionBlock.enabled" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.enabled = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.enabled = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.short_name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        obj
    }
}
