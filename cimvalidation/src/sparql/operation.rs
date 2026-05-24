use cimdecoder::CimDataset;
use crate::Violation;

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    check_measurement_terminal_required_cases(dataset)
}

const MEASUREMENT_TYPES: &[&str] = &["Measurement", "Analog", "Discrete", "Accumulator", "StringMeasurement"];

fn get_measurement_fields(entry: &cimdecoder::CimEntry) -> Option<(&str, Option<&str>, Option<&str>)> {
    fn from_m(m: &cimstructs::Measurement) -> Option<(&str, Option<&str>, Option<&str>)> {
        Some((
            m.measurement_type.as_str(),
            m.power_system_resource.as_ref().map(|r| r.mrid.as_str()),
            m.terminal.as_ref().map(|r| r.mrid.as_str()),
        ))
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::Measurement>() {
        return from_m(o);
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::Analog>() {
        return from_m(&o.base);
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::Discrete>() {
        return from_m(&o.base);
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::Accumulator>() {
        return from_m(&o.base);
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::StringMeasurement>() {
        return from_m(&o.base);
    }
    None
}

fn check_measurement_terminal_required_cases(dataset: &CimDataset) -> Vec<Violation> {
    // Build index: terminal MRID → conducting equipment MRID (for verifying terminal belongs to PSR)
    let mut v = Vec::new();

    for type_name in MEASUREMENT_TYPES {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            let (m_type, psr_ref, term_ref) = match get_measurement_fields(entry) {
                Some(f) => f, None => continue,
            };

            if m_type == "TapPosition" || m_type == "SwitchPosition" {
                if term_ref.is_some() {
                    v.push(Violation {
                        object_id:   mrid.clone(),
                        rule_id:     "opn452:Measurement.Terminal-requiredCases".into(),
                        name:        "Measurement.Terminal-requiredCases".into(),
                        class:       (*type_name).to_string(),
                        property:    "Terminal".into(),
                        message:     format!("Measurement.Terminal should not be exchanged for measurementType '{m_type}'."),
                        severity:    "sh:Violation".into(),
                        description: String::new(),
                    });
                }
                continue;
            }

            let term_mrid = match term_ref {
                Some(r) => r.trim_start_matches('#'),
                None => {
                    v.push(Violation {
                        object_id:   mrid.clone(),
                        rule_id:     "opn452:Measurement.Terminal-requiredCases".into(),
                        name:        "Measurement.Terminal-requiredCases".into(),
                        class:       (*type_name).to_string(),
                        property:    "Terminal".into(),
                        message:     format!("Measurement.Terminal is required for measurementType '{m_type}'."),
                        severity:    "sh:Violation".into(),
                        description: String::new(),
                    });
                    continue;
                }
            };

            let psr_id = match psr_ref { Some(r) => r.trim_start_matches('#'), None => continue };

            // Verify terminal belongs to the PSR
            let term_belongs = dataset.entries.get(term_mrid)
                .and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>())
                .and_then(|t| t.conducting_equipment.as_ref())
                .map_or(false, |ce| ce.mrid.trim_start_matches('#') == psr_id);

            if !term_belongs {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "opn452:Measurement.Terminal-requiredCases".into(),
                    name:        "Measurement.Terminal-requiredCases".into(),
                    class:       (*type_name).to_string(),
                    property:    "Terminal".into(),
                    message:     format!("Terminal {term_mrid} is not a terminal of PowerSystemResource {psr_id}."),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
        }
    }
    v
}
