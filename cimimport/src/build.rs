//! Building a CGMES dataset object by object, for any source format.
//!
//! Every object is named by a key the source gives it (`bus:14`,
//! `branch:4-7:2`), and its mRID is a v5 UUID of the model's seed and that key:
//! converting the same case again gives the same mRIDs, so a DL layout, a
//! difference model or an edit saved against one conversion still matches the
//! next. Fields are set by their `Class.attr` key and checked against the
//! CGMES tables as they are set, so a misspelt one fails at once instead of
//! being written as an unknown element.

use cimmodel::base::{ClassDef, TypeRegistry};
use cimmodel::convert::dataset_to_xml_for_profile;
use cimmodel::registry::type_registry;
use cimmodel::{CimDataset, Element, FieldValue};
use sha1::{Digest, Sha1};

/// Namespace of the converters' v5 UUIDs.
const UUID_NAMESPACE: [u8; 16] = [
    0x5c, 0x1f, 0x0e, 0x7a, 0x52, 0x3d, 0x4b, 0x9f, 0x8e, 0x21, 0x6a, 0xd4, 0x0b, 0x77, 0xc3, 0x19,
];

/// The CGMES profiles a conversion writes, in the order of their files.
pub const PROFILES: [&str; 4] = ["EQ", "SSH", "TP", "SV"];

/// What a header says about the model, besides its profile.
#[derive(Clone, Debug)]
pub struct Header {
    /// `Model.created` and `Model.scenarioTime`.
    pub time: String,
    pub description: String,
    pub modeling_authority_set: String,
}

pub struct Builder {
    pub ds: CimDataset,
    seed: String,
    reg: &'static TypeRegistry,
}

/// A v5 UUID of `name` in [`UUID_NAMESPACE`], RFC 4122 section 4.3.
fn uuid_v5(name: &str) -> String {
    let mut h = Sha1::new();
    h.update(UUID_NAMESPACE);
    h.update(name.as_bytes());
    let d = h.finalize();
    let mut b = [0u8; 16];
    b.copy_from_slice(&d[..16]);
    b[6] = (b[6] & 0x0f) | 0x50;
    b[8] = (b[8] & 0x3f) | 0x80;
    let hex: String = b.iter().map(|x| format!("{x:02x}")).collect();
    format!(
        "{}-{}-{}-{}-{}",
        &hex[..8],
        &hex[8..12],
        &hex[12..16],
        &hex[16..20],
        &hex[20..]
    )
}

impl Builder {
    /// `seed` tells models apart: the case's name, so that two cases' buses
    /// numbered alike still get different mRIDs.
    pub fn new(seed: &str) -> Self {
        Self {
            ds: CimDataset::new(),
            seed: seed.to_string(),
            reg: type_registry(),
        }
    }

    /// The dataset key (`_` and the mRID) of the object `key` names.
    pub fn id(&self, key: &str) -> String {
        format!("_{}", uuid_v5(&format!("{}/{key}", self.seed)))
    }

    fn class(&self, name: &str) -> &'static ClassDef {
        self.reg
            .by_type_name(name)
            .unwrap_or_else(|| panic!("CGMES has no class {name}"))
    }

    /// Adds an object of `class` with its mRID and name, and returns its id.
    pub fn add(&mut self, class: &str, key: &str, name: &str) -> String {
        let id = self.id(key);
        let class = self.class(class);
        assert!(
            !self.ds.entries.contains_key(&id),
            "two objects keyed {key}"
        );
        self.insert(id.clone(), Element::new(class, id.clone()));
        // The SV's classes are no IdentifiedObjects: they have neither.
        if self.reg.attr(class, "IdentifiedObject.mRID").is_none() {
            return id;
        }
        self.text(&id, "IdentifiedObject.mRID", &id[1..]);
        if !name.is_empty() {
            self.text(&id, "IdentifiedObject.name", name);
        }
        id
    }

    /// A new object into both of the dataset's maps. Not `CimDataset::set`,
    /// which looks for the id among every object of its type first: a model of
    /// thousands of buses has a hundred thousand terminals.
    fn insert(&mut self, id: String, element: Element) {
        self.ds
            .by_type
            .entry(element.type_name().to_string())
            .or_default()
            .push(id.clone());
        self.ds.entries.insert(id, element);
    }

    fn put(&mut self, id: &str, attr: &str, value: FieldValue) {
        let element = self
            .ds
            .entries
            .get_mut(id)
            .unwrap_or_else(|| panic!("no object {id}"));
        let def = self
            .reg
            .attr(element.class(), attr)
            .unwrap_or_else(|| panic!("{} has no attribute {attr}", element.type_name()));
        element.add_field(def.id, value);
    }

    pub fn text(&mut self, id: &str, attr: &str, value: &str) {
        self.put(id, attr, FieldValue::Text(value.to_string()));
    }

    /// A number as Rust writes an `f64`: the shortest text that reads back as
    /// the same value, never in exponent form.
    pub fn num(&mut self, id: &str, attr: &str, value: f64) {
        // `-0` reads oddly in a model and means the same.
        let value = if value == 0.0 { 0.0 } else { value };
        self.text(id, attr, &value.to_string());
    }

    pub fn int(&mut self, id: &str, attr: &str, value: i64) {
        self.text(id, attr, &value.to_string());
    }

    pub fn flag(&mut self, id: &str, attr: &str, value: bool) {
        self.text(id, attr, if value { "true" } else { "false" });
    }

    /// A reference to another object, by its id.
    pub fn link(&mut self, id: &str, attr: &str, target: &str) {
        self.put(id, attr, FieldValue::Resource(target.to_string()));
    }

    /// An enumeration value, `Type.value` (`WindingConnection.Y`).
    pub fn value(&mut self, id: &str, attr: &str, value: &str) {
        // `eu:LimitKind` and `eu:SVCControlMode` are tabled as classes, not
        // enumerations (see `cimmodel::convert`); the writer knows them.
        debug_assert!(
            self.reg
                .enum_value(self.class("IdentifiedObject"), value)
                .is_some()
                || value
                    .split_once('.')
                    .is_some_and(|(ty, _)| self.reg.by_type_name(ty).is_some()),
            "CGMES has no enumeration value {value}"
        );
        self.put(id, attr, FieldValue::Resource(value.to_string()));
    }

    /// The model's files, one per profile of [`PROFILES`], each with its
    /// header; each depends on the one before it (SV on TP, TP on SSH, SSH on
    /// EQ), and is named `<stem>_<profile>.xml`.
    pub fn finish(
        mut self,
        stem: &str,
        header: &Header,
    ) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
        let profiles = self
            .reg
            .schema("cgmes")
            .expect("the CGMES family is registered")
            .profiles;
        let full_model = self.class("FullModel");
        let mut previous: Option<String> = None;
        for code in PROFILES {
            let uri = profiles
                .iter()
                .find(|(c, _)| *c == code)
                .map(|(_, uri)| *uri)
                .unwrap_or_else(|| panic!("CGMES has no profile {code}"));
            let id = format!(
                "urn:uuid:{}",
                uuid_v5(&format!("{}/model:{code}", self.seed))
            );
            self.insert(id.clone(), Element::new(full_model, id.clone()));
            self.text(&id, "Model.profile", uri);
            self.text(&id, "Model.created", &header.time);
            self.text(&id, "Model.scenarioTime", &header.time);
            self.text(&id, "Model.description", &header.description);
            self.text(
                &id,
                "Model.modelingAuthoritySet",
                &header.modeling_authority_set,
            );
            self.text(&id, "Model.version", "1");
            if let Some(previous) = &previous {
                self.link(&id, "Model.DependentOn", previous);
            }
            previous = Some(id);
        }
        PROFILES
            .iter()
            .map(|code| {
                Ok((
                    format!("{stem}_{code}.xml"),
                    dataset_to_xml_for_profile(&self.ds, code)?,
                ))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uuids_are_version_5_and_stable() {
        let a = uuid_v5("case14/bus:1");
        assert_eq!(a, uuid_v5("case14/bus:1"));
        assert_ne!(a, uuid_v5("case14/bus:2"));
        assert_eq!(a.len(), 36);
        assert_eq!(&a[14..15], "5");
        assert!(matches!(&a[19..20], "8" | "9" | "a" | "b"));
    }

    #[test]
    #[should_panic(expected = "has no attribute ACLineSegment.rr")]
    fn a_misspelt_attribute_fails_at_once() {
        let mut b = Builder::new("t");
        let id = b.add("ACLineSegment", "l", "L");
        b.num(&id, "ACLineSegment.rr", 1.0);
    }
}
