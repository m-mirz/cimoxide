//! Where a family's class table comes from.
//!
//! By default it is the table `cimgen` generated. With the `dynamic-schema`
//! feature, an ENTSO-E RDFS directory can supply it instead, so a new profile
//! version is a data load rather than a recompile.
//!
//! Resolution order, highest first:
//!
//! 1. [`load_from`] — an explicit call, for libraries and tests
//! 2. `CIMOXIDE_PROFILES` — for the CLI, the servers and the Python bindings.
//!    It lists directories, separated as in `PATH`, searched for profile files
//!    that declare themselves a family's vocabulary
//!    ([`cimschema::profile_file::Pool`]); a family it holds none of keeps its
//!    generated table
//! 3. the generated table
//!
//! A directory that is missing or fails to parse falls back to the generated
//! table **with a warning**. Silently serving a stale schema because a path had
//! a typo is the worst outcome available here.

use crate::base::Schema;

/// Resolve the schema for a family.
///
/// `generated` is the table `cimgen` emitted, used unless something overrides it.
pub fn resolve(family: &'static str, generated: &'static Schema) -> &'static Schema {
    #[cfg(feature = "dynamic-schema")]
    {
        dynamic::resolve(family, generated)
    }
    #[cfg(not(feature = "dynamic-schema"))]
    {
        let _ = family;
        generated
    }
}

#[cfg(feature = "dynamic-schema")]
pub use dynamic::{
    PROFILES_ENV, SchemaError, family_of_dir, load_from, load_pool_table, load_table,
};

#[cfg(feature = "dynamic-schema")]
mod dynamic {
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Mutex, OnceLock};

    use cimschema::family::{self, Family};
    use cimschema::model::{CimAttribute, CimSpecification};

    use crate::base::{AttrDef, AttrKind, ClassDef, EnumDef, EnumValueDef, Schema};

    pub use cimschema::profile_file::PROFILES_ENV;
    use cimschema::profile_file::{Pool, Role};

    #[derive(Debug)]
    pub enum SchemaError {
        /// The registry was already built, so this load would have been ignored.
        TooLate,
        /// A table was already loaded for this family.
        AlreadyLoaded,
        /// No family with this id.
        UnknownFamily(String),
        Parse(String),
    }

    impl std::fmt::Display for SchemaError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::TooLate => write!(
                    f,
                    "the type registry is already built; load the schema before decoding"
                ),
                Self::AlreadyLoaded => write!(f, "a schema is already loaded for this family"),
                Self::UnknownFamily(id) => write!(f, "unknown family: {id}"),
                Self::Parse(e) => write!(f, "{e}"),
            }
        }
    }

    impl std::error::Error for SchemaError {}

    fn explicit() -> &'static Mutex<HashMap<&'static str, &'static Schema>> {
        static E: OnceLock<Mutex<HashMap<&'static str, &'static Schema>>> = OnceLock::new();
        E.get_or_init(|| Mutex::new(HashMap::new()))
    }

    /// Set once `resolve` has run, so a late [`load_from`] can report that it
    /// would have had no effect instead of silently doing nothing.
    static RESOLVED: AtomicBool = AtomicBool::new(false);

    /// Load a family's class table from a directory of RDFS vocabularies.
    ///
    /// Takes precedence over `CIMOXIDE_PROFILES`. Must be called before the
    /// first decode: the registry is memoized, so afterwards this returns
    /// [`SchemaError::TooLate`] rather than being quietly ignored.
    pub fn load_from(family_id: &str, dir: &Path) -> Result<(), SchemaError> {
        let family = family::by_id(family_id)
            .ok_or_else(|| SchemaError::UnknownFamily(family_id.to_string()))?;
        if RESOLVED.load(Ordering::SeqCst) {
            return Err(SchemaError::TooLate);
        }
        let classes = load_table(family.id, dir)?;
        let mut map = explicit().lock().unwrap();
        if map.contains_key(family.id) {
            return Err(SchemaError::AlreadyLoaded);
        }
        map.insert(family.id, classes);
        Ok(())
    }

    pub(super) fn resolve(family_id: &'static str, generated: &'static Schema) -> &'static Schema {
        RESOLVED.store(true, Ordering::SeqCst);

        if let Some(classes) = explicit().lock().unwrap().get(family_id) {
            return classes;
        }

        let Some(pool) = Pool::from_env() else {
            return generated;
        };
        let Some(family) = family::by_id(family_id) else {
            return generated;
        };
        let files = pool.files(family, Role::Vocabulary);
        if files.is_empty() {
            return generated;
        }
        match load_files(family, &files) {
            Ok(classes) => classes,
            Err(e) => {
                eprintln!(
                    "warning: {PROFILES_ENV} set but the {family_id} schema could not be \
                     loaded ({e}); falling back to the generated class table"
                );
                generated
            }
        }
    }

    /// Which family's vocabularies an RDFS directory holds, by what the files
    /// declare (see [`cimschema::profile_file`]): `None` when it holds neither
    /// family's, or both.
    pub fn family_of_dir(dir: &Path) -> Option<&'static Family> {
        match cimschema::profile_file::families_in(dir, cimschema::profile_file::Role::Vocabulary)
            .as_slice()
        {
            [family] => Some(family),
            _ => None,
        }
    }

    /// The files in `dir` that declare themselves `family`'s vocabulary.
    fn vocabulary(dir: &Path, family: &Family) -> Vec<PathBuf> {
        cimschema::profile_file::in_dir(dir, family, cimschema::profile_file::Role::Vocabulary)
    }

    /// Build a family's class table from RDFS without installing it.
    ///
    /// Useful for inspecting what a schema directory would produce, and for
    /// checking it against the generated table.
    pub fn load_table(family_id: &str, dir: &Path) -> Result<&'static Schema, SchemaError> {
        let family = family::by_id(family_id)
            .ok_or_else(|| SchemaError::UnknownFamily(family_id.to_string()))?;
        load_family(family, dir)
    }

    fn load_family(family: &'static Family, dir: &Path) -> Result<&'static Schema, SchemaError> {
        // The files that declare themselves the family's, so a directory
        // holding both families' vocabularies serves each its own.
        let files = vocabulary(dir, family);
        if files.is_empty() {
            return Err(SchemaError::Parse(format!(
                "no {} vocabulary in {}",
                family.id,
                dir.display()
            )));
        }
        load_files(family, &files)
    }

    /// Build a family's class table from the files a [`Pool`] found, without
    /// installing it: what `CIMOXIDE_PROFILES` would load.
    pub fn load_pool_table(family_id: &str, pool: &Pool) -> Result<&'static Schema, SchemaError> {
        let family = family::by_id(family_id)
            .ok_or_else(|| SchemaError::UnknownFamily(family_id.to_string()))?;
        load_files(family, &pool.files(family, Role::Vocabulary))
    }

    fn load_files(
        family: &'static Family,
        files: &[PathBuf],
    ) -> Result<&'static Schema, SchemaError> {
        if files.is_empty() {
            return Err(SchemaError::Parse(format!("no {} vocabulary", family.id)));
        }
        let spec = cimschema::import::import_schema_paths(files, family, false)
            .map_err(|e| SchemaError::Parse(e.to_string()))?;
        Ok(build(&spec))
    }

    /// Interns schema strings so they satisfy the `&'static` signatures the
    /// element trait and registry require. Bounded by schema size and leaked
    /// once per process.
    #[derive(Default)]
    struct Interner {
        seen: HashMap<String, &'static str>,
    }

    impl Interner {
        fn intern(&mut self, s: &str) -> &'static str {
            if let Some(v) = self.seen.get(s) {
                return v;
            }
            let leaked: &'static str = Box::leak(s.to_string().into_boxed_str());
            self.seen.insert(s.to_string(), leaked);
            leaked
        }

        fn intern_all(&mut self, items: &[String]) -> &'static [&'static str] {
            let v: Vec<&'static str> = items.iter().map(|s| self.intern(s)).collect();
            Vec::leak(v)
        }
    }

    // Classification, ranges, datatypes and namespaces come from
    // `cimschema::table`, which `cimgen::generator::classes_gen` calls too; the
    // `runtime_table_matches_generated` tests check the rendering on top.
    fn attr_kind(a: &CimAttribute) -> AttrKind {
        match cimschema::table::attr_kind(a) {
            cimschema::table::Kind::Literal => AttrKind::Literal,
            cimschema::table::Kind::Association => AttrKind::Association,
            cimschema::table::Kind::Enum => AttrKind::Enum,
        }
    }

    fn build(spec: &CimSpecification) -> &'static Schema {
        // Sorted key order, because the index in this vector is the class id
        // that `super_class` refers to.
        let mut ids: Vec<&String> = spec.types.keys().collect();
        ids.sort();
        let index_of: HashMap<&str, usize> = ids
            .iter()
            .enumerate()
            .map(|(i, id)| (id.as_str(), i))
            .collect();

        let mut interner = Interner::default();
        let mut out: Vec<ClassDef> = Vec::with_capacity(ids.len());

        for id in &ids {
            let t = &spec.types[*id];
            let attrs: Vec<AttrDef> = t
                .attributes
                .iter()
                .map(|a| AttrDef {
                    id: interner.intern(&a.id),
                    ns: interner.intern(&a.namespace),
                    kind: attr_kind(a),
                    range: interner.intern(cimschema::table::attr_range(a)),
                    is_list: a.is_list,
                    used: a.is_association_used,
                    xsd: cimschema::table::xsd_type(a),
                    value_ns: interner.intern(cimschema::table::value_namespace(spec, a)),
                    origins: interner.intern_all(&a.origins),
                    comment: interner.intern(&a.comment),
                })
                .collect();

            let qualified = format!("{}{}", spec.family.type_prefix, t.id);
            out.push(ClassDef {
                ns: interner.intern(&t.namespace),
                local: interner.intern(&t.id),
                qualified: interner.intern(&qualified),
                super_class: index_of.get(t.super_type.as_str()).copied(),
                concrete: !t.concrete_in.is_empty(),
                attrs: Vec::leak(attrs),
                origins: interner.intern_all(&t.origins),
                comment: interner.intern(&t.comment),
            });
        }

        let pairs = |interner: &mut Interner,
                     v: Vec<(&str, &str)>|
         -> &'static [(&'static str, &'static str)] {
            Vec::leak(
                v.into_iter()
                    .map(|(a, b)| (interner.intern(a), interner.intern(b)))
                    .collect(),
            )
        };
        let profiles = pairs(&mut interner, cimschema::table::profile_uris(spec));
        let namespaces = pairs(&mut interner, cimschema::table::namespaces(spec));
        let enums: Vec<EnumDef> = cimschema::table::enums(spec)
            .into_iter()
            .map(|e| EnumDef {
                ns: interner.intern(&e.namespace),
                local: interner.intern(&e.id),
                comment: interner.intern(&e.comment),
                values: Vec::leak(
                    e.values
                        .iter()
                        .map(|v| EnumValueDef {
                            id: interner.intern(&v.id),
                            comment: interner.intern(&v.comment),
                        })
                        .collect(),
                ),
            })
            .collect();
        Box::leak(Box::new(Schema {
            classes: Vec::leak(out),
            profiles,
            namespaces,
            enums: Vec::leak(enums),
        }))
    }
}
