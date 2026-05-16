use std::collections::HashMap;

pub const CGMES_VERSION_3_0_0: &str = "3.0.0";

pub const DATA_TYPE_STRING: &str = "String";
pub const DATA_TYPE_INTEGER: &str = "Integer";
pub const DATA_TYPE_BOOLEAN: &str = "Boolean";
pub const DATA_TYPE_FLOAT: &str = "Float";
pub const DATA_TYPE_DATE: &str = "Date";
pub const DATA_TYPE_DATE_TIME: &str = "DateTime";
pub const DATA_TYPE_DECIMAL: &str = "Decimal";
pub const DATA_TYPE_MONTH_DAY: &str = "MonthDay";

#[derive(Debug, Default, Clone)]
pub struct CimAttribute {
    pub id: String,
    pub label: String,
    pub namespace: String,
    pub comment: String,
    pub cim_multiplicity: String,
    pub is_list: bool,
    pub cim_association_used: String,
    pub is_association_used: bool,
    pub cim_is_fixed: String,
    pub is_fixed: bool,
    pub cim_inverse_role: String,
    pub has_inverse_role: bool,
    pub inverse_role_attribute: String,
    pub is_inverse_role_attribute_list: bool,
    pub cim_stereotype: String,
    pub rdf_range: String,
    pub cim_data_type: String,
    pub data_type: String,
    pub is_primitive: bool,
    pub rdf_domain: String,
    pub rdf_type: String,
    pub default_value: String,
    pub is_enum_value: bool,
    pub lang_type: String,
    pub is_cim_datatype: bool,
    pub is_class: bool,
    pub use_id_reference: bool,
    pub origin: String,
    pub origins: Vec<String>,
    pub cim_categories: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub struct CimType {
    pub id: String,
    pub label: String,
    pub namespace: String,
    pub comment: String,
    pub cim_stereotype: String,
    pub rdf_type: String,
    pub super_type: String,
    pub has_class_attributes: bool,
    pub primitive_types: Vec<String>,
    pub cim_datatypes: Vec<String>,
    pub enum_types: Vec<String>,
    pub origin: String,
    pub origins: Vec<String>,
    pub cim_categories: Vec<String>,
    pub attributes: Vec<CimAttribute>,
}

#[derive(Debug, Default, Clone)]
pub struct CimDatatype {
    pub id: String,
    pub label: String,
    pub namespace: String,
    pub comment: String,
    pub cim_stereotype: String,
    pub rdf_type: String,
    pub lang_type: String,
    pub primitive_type: String,
    pub cim_category: String,
    pub attributes: Vec<CimAttribute>,
}

#[derive(Debug, Default, Clone)]
pub struct CimPrimitive {
    pub id: String,
    pub label: String,
    pub namespace: String,
    pub comment: String,
    pub cim_stereotype: String,
    pub rdf_type: String,
    pub data_type: String,
    pub lang_type: String,
}

#[derive(Debug, Default, Clone)]
pub struct CimEnum {
    pub id: String,
    pub label: String,
    pub namespace: String,
    pub comment: String,
    pub cim_stereotype: String,
    pub rdf_type: String,
    pub origin: String,
    pub origins: Vec<String>,
    pub values: Vec<CimEnumValue>,
}

#[derive(Debug, Default, Clone)]
pub struct CimEnumValue {
    pub id: String,
    pub label: String,
    pub comment: String,
    pub cim_stereotype: String,
    pub rdf_type: String,
}

#[derive(Debug, Default, Clone)]
pub struct CimOntology {
    pub id: String,
    pub namespace: String,
    pub owl_version_iri: String,
    pub owl_version_info: String,
    pub keyword: String,
    pub rdf_type: String,
    pub name: String,
    pub priority: u32,
}

#[derive(Debug, Default)]
pub struct CimSpecification {
    pub specification_namespaces: HashMap<String, String>,
    pub profile_namespaces: HashMap<String, String>,
    pub ontologies: HashMap<String, CimOntology>,
    pub ontology_list: Vec<String>,
    pub types: HashMap<String, CimType>,
    pub enums: HashMap<String, CimEnum>,
    pub primitive_types: HashMap<String, CimPrimitive>,
    pub cim_datatypes: HashMap<String, CimDatatype>,
    pub cgmes_version: String,
}

impl CimSpecification {
    pub fn new() -> Self {
        CimSpecification {
            cgmes_version: CGMES_VERSION_3_0_0.to_string(),
            ..Default::default()
        }
    }
}
