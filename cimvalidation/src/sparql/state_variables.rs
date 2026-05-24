use cimdecoder::CimDataset;
use crate::Violation;

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(check_cs_converter_state_value_range(dataset));
    v.extend(check_topological_island_count(dataset));
    v
}

fn check_cs_converter_state_value_range(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    const RECTIFIER: &str = "http://iec.ch/TC57/CIM100#CsOperatingModeKind.rectifier";
    const INVERTER:  &str = "http://iec.ch/TC57/CIM100#CsOperatingModeKind.inverter";

    for mrid in dataset.by_type.get("CsConverter").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let obj = match entry.element.as_any().downcast_ref::<cimstructs::CsConverter>() {
            Some(o) => o, None => continue,
        };
        let mode = match obj.operating_mode.as_ref() { Some(r) => r.uri.as_str(), None => continue };

        if mode == RECTIFIER {
            let alpha = obj.alpha.unwrap_or(0.0);
            if alpha < 10.0 || alpha > 18.0 {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "svc.CsConverter.alpha-valueRangeTypical".into(),
                    name:        "CsConverter.alpha-valueRangeTypical".into(),
                    class:       "CsConverter".into(),
                    property:    "CsConverter.alpha".into(),
                    message:     "The alpha value is outside typical range (10-18 degrees) for a rectifier.".into(),
                    severity:    "sh:Warning".into(),
                    description: String::new(),
                });
            }
        } else if mode == INVERTER {
            let gamma = obj.gamma.unwrap_or(0.0);
            if gamma < 17.0 || gamma > 20.0 {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "svc.CsConverter.gamma-valueRangeTypical".into(),
                    name:        "CsConverter.gamma-valueRangeTypical".into(),
                    class:       "CsConverter".into(),
                    property:    "CsConverter.gamma".into(),
                    message:     "The gamma value is outside typical range (17-20 degrees) for an inverter.".into(),
                    severity:    "sh:Warning".into(),
                    description: String::new(),
                });
            }
        }
    }
    v
}

fn check_topological_island_count(dataset: &CimDataset) -> Vec<Violation> {
    let count = dataset.by_type.get("TopologicalIsland").map_or(0, |v| v.len());
    if count == 0 {
        vec![Violation {
            object_id:   "global".into(),
            rule_id:     "sv456:TopologicalIsland-instance".into(),
            name:        "TopologicalIsland-instance".into(),
            class:       "TopologicalIsland".into(),
            property:    "rdf:type".into(),
            message:     "No TopologicalIsland instantiated.".into(),
            severity:    "sh:Violation".into(),
            description: String::new(),
        }]
    } else {
        Vec::new()
    }
}
