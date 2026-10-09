//! What a class table records, derived from a parsed specification.
//!
//! `cimgen` renders the tables as Rust source and `cimmodel` builds them at
//! runtime from RDFS; both call these functions, so the two cannot disagree
//! on how an attribute is classified, typed or namespaced.

use crate::model::*;

/// What an attribute's value denotes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Literal,
    Association,
    Enum,
}

pub fn attr_kind(a: &CimAttribute) -> Kind {
    if a.is_enum_value {
        Kind::Enum
    } else if a.is_primitive || a.is_cim_datatype {
        Kind::Literal
    } else {
        Kind::Association
    }
}

/// The attribute's range: the class an association or enumeration points to,
/// or the CIM datatype of a literal.
pub fn attr_range(a: &CimAttribute) -> &str {
    if a.rdf_range.is_empty() {
        &a.cim_data_type
    } else {
        &a.rdf_range
    }
}

/// The XSD datatype a literal attribute is written in, as the local name
/// after `http://www.w3.org/2001/XMLSchema#`: `"double"` for `Float`, and so
/// on. `""` for associations and enumerations.
///
/// CGMES `Float` is `double`, not `float`: values are read into `f64`, and
/// claiming single precision would make a typed comparison disagree with the
/// decoded value. `URI` (`md:Model.profile` and friends) is `anyURI`, as the
/// SHACL files type those values. Anything unrecognised is `string`.
pub fn xsd_type(a: &CimAttribute) -> &'static str {
    if attr_kind(a) != Kind::Literal {
        return "";
    }
    match a.data_type.as_str() {
        DATA_TYPE_STRING => "string",
        DATA_TYPE_INTEGER => "integer",
        DATA_TYPE_BOOLEAN => "boolean",
        DATA_TYPE_FLOAT => "double",
        DATA_TYPE_DECIMAL => "decimal",
        DATA_TYPE_DATE => "date",
        DATA_TYPE_DATE_TIME => "dateTime",
        DATA_TYPE_MONTH_DAY => "gMonthDay",
        "URI" => "anyURI",
        _ => "string",
    }
}

/// The namespace an enumeration attribute's values live in: the decoder keeps
/// a value after its `#`, and this is what rebuilds
/// `http://iec.ch/TC57/CIM100#WindingConnection.D` from `WindingConnection.D`.
/// `""` for any other attribute, or an enumeration the schema does not define.
pub fn value_namespace<'a>(spec: &'a CimSpecification, a: &CimAttribute) -> &'a str {
    if attr_kind(a) != Kind::Enum {
        return "";
    }
    spec.enums.get(&a.rdf_range).map_or("", |e| e.namespace.as_str())
}

/// The family's enumerations, sorted by name; each one's values in
/// vocabulary order.
pub fn enums(spec: &CimSpecification) -> Vec<&CimEnum> {
    let mut v: Vec<&CimEnum> = spec.enums.values().collect();
    v.sort_by(|a, b| a.id.cmp(&b.id));
    v
}

/// Profile code → profile URI (the `owl:versionIRI` a dataset declares), sorted
/// by code.
pub fn profile_uris(spec: &CimSpecification) -> Vec<(&str, &str)> {
    let mut v: Vec<(&str, &str)> = spec
        .ontologies
        .iter()
        .filter(|(_, o)| !o.owl_version_iri.is_empty())
        .map(|(k, o)| (k.as_str(), o.owl_version_iri.as_str()))
        .collect();
    v.sort_unstable();
    v
}

/// XML prefix → namespace IRI, as the family's vocabularies bind them, sorted
/// by prefix.
pub fn namespaces(spec: &CimSpecification) -> Vec<(&str, &str)> {
    let mut v: Vec<(&str, &str)> =
        spec.profile_namespaces.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
    v.sort_unstable();
    v
}
