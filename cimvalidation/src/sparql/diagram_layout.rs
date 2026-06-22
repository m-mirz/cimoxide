use cimdecoder::CimDataset;
use crate::Violation;

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    check_diagram_object_identified_object_type(dataset)
}

const DISALLOWED_TYPES: &[&str] = &[
    "Diagram", "DiagramObject", "VisibilityLayer",
    "DiagramStyle", "DiagramObjectStyle", "TextDiagramObject",
];

fn check_diagram_object_identified_object_type(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();

    for type_name in &["DiagramObject", "TextDiagramObject"] {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            let id_obj_ref = if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::DiagramObject>() {
                o.identified_object_.as_ref()
            } else if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::TextDiagramObject>() {
                o.base.identified_object_.as_ref()
            } else {
                continue;
            };

            let id_obj_ref = match id_obj_ref { Some(r) => r, None => continue };
            let target_id = id_obj_ref.mrid.trim_start_matches('#');
            let target_type = match dataset.entries.get(target_id).map(|e| e.element.type_name()) {
                Some(t) => t, None => continue,
            };
            if DISALLOWED_TYPES.contains(&target_type) {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "dlu:DiagramObject.IdentifiedObject-DLvalueType".into(),
                    name:        "C:301:DL:DiagramObject.IdentifiedObject:internalValueType".into(),
                    class:       (*type_name).to_string(),
                    property:    "DiagramObject.IdentifiedObject".into(),
                    message:     "The value type shall not be an instance of cim:Diagram, cim:DiagramObject, cim:VisibilityLayer, cim:DiagramStyle, cim:DiagramObjectStyle or cim:TextDiagramObject.".into(),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
        }
    }
    v
}
