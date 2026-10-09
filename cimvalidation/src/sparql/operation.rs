use cimmodel::CimDataset;
use super::Fields;
use crate::Violation;

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    check_measurement_terminal_required_cases(dataset)
}

const MEASUREMENT_TYPES: &[&str] = &["Measurement", "Analog", "Discrete", "Accumulator", "StringMeasurement"];

fn check_measurement_terminal_required_cases(dataset: &CimDataset) -> Vec<Violation> {
    // Build index: terminal MRID → conducting equipment MRID (for verifying terminal belongs to PSR)
    let mut v = Vec::new();

    for type_name in MEASUREMENT_TYPES {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            let Some(m) = Fields::of_class(entry, type_name) else { continue };
            let m_type = m.text("Measurement.measurementType");
            let psr_ref = m.reference("Measurement.PowerSystemResource");
            let term_ref = m.reference("Measurement.Terminal");

            if m_type == "TapPosition" || m_type == "SwitchPosition" {
                if term_ref.is_some() {
                    v.push(Violation {
                        object_id:   mrid.clone(),
                        rule_id:     "opn452:Measurement.Terminal-requiredCases".into(),
                        name:        "C:452:OP:Measurement.Terminal:requiredCases".into(),
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
                        name:        "C:452:OP:Measurement.Terminal:requiredCases".into(),
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
            let term_belongs = Fields::get(dataset, term_mrid, "Terminal")
                .is_some_and(|t| t.reference("Terminal.ConductingEquipment")
                    .is_some_and(|ce| ce.trim_start_matches('#') == psr_id));

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
