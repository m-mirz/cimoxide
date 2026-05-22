#[derive(Debug, Clone)]
pub struct Violation {
    pub object_id:   String,
    pub rule_id:     String,
    pub class:       String,
    pub property:    String,
    pub message:     String,
    pub severity:    String,
    pub name:        String,
    pub description: String,
}
