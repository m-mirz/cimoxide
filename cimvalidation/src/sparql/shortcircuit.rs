use cimmodel::CimDataset;
use super::Fields;
use crate::Violation;

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(check_series_compensator_varistor_usage(dataset));
    v.extend(check_transformer_end_grounding(dataset));
    v.extend(check_synchronous_machine_earthing(dataset));
    v.extend(check_series_compensator_varistor_required(dataset));
    v
}

fn check_series_compensator_varistor_usage(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SeriesCompensator").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let obj = match Fields::of_class(entry, "SeriesCompensator") {
            Some(o) => o, None => continue,
        };
        if !obj.bool("SeriesCompensator.varistorPresent").unwrap_or(false) {
            if obj.f64("SeriesCompensator.varistorRatedCurrent").unwrap_or(0.0) != 0.0 {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "scu:SeriesCompensator.varistorRatedCurrent-usage".into(),
                    name:        "C:301:SC:SeriesCompensator.varistorRatedCurrent:usage".into(),
                    class:       "SeriesCompensator".into(),
                    property:    "SeriesCompensator.varistorRatedCurrent".into(),
                    message:     "The attribute is present and SeriesCompensator.varistorPresent is false.".into(),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
            if obj.f64("SeriesCompensator.varistorVoltageThreshold").unwrap_or(0.0) != 0.0 {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "scu:SeriesCompensator.varistorVoltageThreshold-usage".into(),
                    name:        "C:301:SC:SeriesCompensator.varistorVoltageThreshold:usage".into(),
                    class:       "SeriesCompensator".into(),
                    property:    "SeriesCompensator.varistorVoltageThreshold".into(),
                    message:     "The attribute is present and SeriesCompensator.varistorPresent is false.".into(),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
        }
    }
    v
}

fn check_transformer_end_grounding(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("PowerTransformerEnd").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let obj = match Fields::of_class(entry, "PowerTransformerEnd") {
            Some(o) => o, None => continue,
        };
        if obj.bool("TransformerEnd.grounded").unwrap_or(false)
            && obj.f64("TransformerEnd.rground").unwrap_or(0.0) == 0.0 && obj.f64("TransformerEnd.xground").unwrap_or(0.0) == 0.0 {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "sc452:TransformerEnd-grounding".into(),
                    name:        "C:452:SC:PowerTransformerEnd.grounded:grounding".into(),
                    class:       "PowerTransformerEnd".into(),
                    property:    "grounded".into(),
                    message:     "Missing required properties .rground or .xground when grounded=true.".into(),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
    }
    v
}

fn check_synchronous_machine_earthing(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let obj = match Fields::of_class(entry, "SynchronousMachine") {
            Some(o) => o, None => continue,
        };
        if obj.bool("SynchronousMachine.earthing").unwrap_or(false)
            && obj.f64("SynchronousMachine.earthingStarPointR").unwrap_or(0.0) == 0.0 && obj.f64("SynchronousMachine.earthingStarPointX").unwrap_or(0.0) == 0.0 {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "sc452:SynchronousMachine-attributes".into(),
                    name:        "C:452:SC:SynchronousMachine.earthing:attributes".into(),
                    class:       "SynchronousMachine".into(),
                    property:    "earthing".into(),
                    message:     "Missing required properties .earthingStarPointR or .earthingStarPointX when earthing=true.".into(),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
    }
    v
}

fn check_series_compensator_varistor_required(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SeriesCompensator").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let obj = match Fields::of_class(entry, "SeriesCompensator") {
            Some(o) => o, None => continue,
        };
        if obj.bool("SeriesCompensator.varistorPresent").unwrap_or(false) {
            if obj.f64("SeriesCompensator.varistorRatedCurrent").unwrap_or(0.0) == 0.0 {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "sc600:SeriesCompensator.varistorRatedCurrent-required".into(),
                    name:        "C:600:SC:SeriesCompensator.varistorRatedCurrent:required".into(),
                    class:       "SeriesCompensator".into(),
                    property:    "SeriesCompensator.varistorRatedCurrent".into(),
                    message:     "The attribute is missing when SeriesCompensator.varistorPresent is true.".into(),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
            if obj.f64("SeriesCompensator.varistorVoltageThreshold").unwrap_or(0.0) == 0.0 {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "sc600:SeriesCompensator.varistorVoltageThreshold-required".into(),
                    name:        "C:600:SC:SeriesCompensator.varistorVoltageThreshold:required".into(),
                    class:       "SeriesCompensator".into(),
                    property:    "SeriesCompensator.varistorVoltageThreshold".into(),
                    message:     "The attribute is missing when SeriesCompensator.varistorPresent is true.".into(),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
        }
    }
    v
}
