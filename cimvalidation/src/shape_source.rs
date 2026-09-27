//! Where a bag family's shape table comes from.
//!
//! By default it is the table `cimgen` generated. With the `dynamic-shapes`
//! feature, a directory of SHACL TTL files can supply it instead, so a new
//! profile version is a data load rather than a recompile.
//!
//! Resolution order, highest first:
//!
//! 1. [`load_from`] — an explicit call, for libraries and tests
//! 2. `CIMOXIDE_SHACL_DIR` — for the CLI and the Python bindings
//! 3. the generated table
//!
//! A directory that is missing or fails to parse falls back to the generated
//! table **with a warning**. Silently validating against a stale profile
//! because a path had a typo is the worst outcome available here.
//!
//! This mirrors [`cimstructs::schema_source`] closely enough that the two
//! should be read side by side. The one difference that matters: there, the
//! generator and the loader build the table by separate code paths and a test
//! compares them. Here both call `cimschema::shacl::resolve`, so there is no
//! second implementation to drift — this module only turns the resolved model
//! into the `&'static` form the interpreter needs.

use crate::shapes::ShapeDef;

/// Resolve the shape table for a bag family.
///
/// `generated` is the table `cimgen` emitted, used unless something overrides
/// it.
pub fn resolve(family: &'static str, generated: &'static [ShapeDef]) -> &'static [ShapeDef] {
    #[cfg(feature = "dynamic-shapes")]
    {
        dynamic::resolve(family, generated)
    }
    #[cfg(not(feature = "dynamic-shapes"))]
    {
        let _ = family;
        generated
    }
}

/// Resolve the profile index for a bag family.
///
/// Loaded from the same directory as the shapes, because a shape table and a
/// profile index that disagree would run the wrong rules — the index decides
/// which profile a dataset is, the table decides what that profile checks.
pub fn resolve_profiles(
    family: &'static str,
    generated_iris: &'static [(&'static str, &'static str)],
    generated_codes: &'static [&'static str],
) -> (&'static [(&'static str, &'static str)], &'static [&'static str]) {
    #[cfg(feature = "dynamic-shapes")]
    {
        dynamic::resolve_profiles(family, generated_iris, generated_codes)
    }
    #[cfg(not(feature = "dynamic-shapes"))]
    {
        let _ = family;
        (generated_iris, generated_codes)
    }
}

#[cfg(feature = "dynamic-shapes")]
pub use dynamic::{load_from, load_table, ShapeError, SHACL_DIR_ENV};

#[cfg(feature = "dynamic-shapes")]
mod dynamic {
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Mutex, OnceLock};

    use cimschema::family::{self, Family};
    use cimschema::shacl::resolve as r;
    use cimschema::shacl::skip::SkipCollector;

    use crate::shapes::{
        AltBranch, Check, ClosedShape, Constraint, NodeKind, Path as SPath, PropShape, ShapeDef,
        Target,
    };

    /// Environment variable naming a directory of SHACL constraint files.
    pub const SHACL_DIR_ENV: &str = "CIMOXIDE_SHACL_DIR";

    #[derive(Debug)]
    pub enum ShapeError {
        /// The table was already resolved, so this load would have been ignored.
        TooLate,
        /// A table was already loaded for this family.
        AlreadyLoaded,
        /// No family with this id, or it is not a bag family.
        UnknownFamily(String),
        /// The RDFS the shapes resolve against could not be read.
        Schema(String),
        Parse(String),
    }

    impl std::fmt::Display for ShapeError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::TooLate => write!(
                    f,
                    "the shape table is already resolved; load the shapes before validating"
                ),
                Self::AlreadyLoaded => write!(f, "shapes are already loaded for this family"),
                Self::UnknownFamily(id) => write!(f, "unknown or non-bag family: {id}"),
                Self::Schema(e) => write!(f, "cannot read the schema the shapes resolve against: {e}"),
                Self::Parse(e) => write!(f, "{e}"),
            }
        }
    }

    impl std::error::Error for ShapeError {}

    /// A loaded table: shapes and the profile index that selects them.
    type Loaded = (
        &'static [ShapeDef],
        &'static [(&'static str, &'static str)],
        &'static [&'static str],
    );

    fn explicit() -> &'static Mutex<HashMap<&'static str, Loaded>> {
        static E: OnceLock<Mutex<HashMap<&'static str, Loaded>>> = OnceLock::new();
        E.get_or_init(|| Mutex::new(HashMap::new()))
    }

    /// Memoizes what the environment variable produced, so shapes and profiles
    /// come from one load rather than two.
    fn from_env() -> &'static Mutex<HashMap<&'static str, Option<Loaded>>> {
        static E: OnceLock<Mutex<HashMap<&'static str, Option<Loaded>>>> = OnceLock::new();
        E.get_or_init(|| Mutex::new(HashMap::new()))
    }

    /// Set once `resolve` has run, so a late [`load_from`] can report that it
    /// would have had no effect instead of silently doing nothing.
    static RESOLVED: AtomicBool = AtomicBool::new(false);

    /// Load a bag family's shapes from a directory of SHACL TTL files.
    ///
    /// Takes precedence over `CIMOXIDE_SHACL_DIR`. Must be called before the
    /// first validation: the table is memoized, so afterwards this returns
    /// [`ShapeError::TooLate`] rather than being quietly ignored.
    ///
    /// As in `cimstructs::schema_source`, the guard is best-effort — there is a
    /// narrow race between the check and the insert — and is documented rather
    /// than locked, because the contract is "call before validating".
    pub fn load_from(family_id: &str, dir: &Path) -> Result<(), ShapeError> {
        let family = bag_family(family_id)?;
        if RESOLVED.load(Ordering::SeqCst) {
            return Err(ShapeError::TooLate);
        }
        let loaded = load(family, dir)?;
        let mut map = explicit().lock().unwrap();
        if map.contains_key(family.id) {
            return Err(ShapeError::AlreadyLoaded);
        }
        map.insert(family.id, loaded);
        Ok(())
    }

    /// Build a family's shape table from SHACL without installing it.
    pub fn load_table(family_id: &str, dir: &Path) -> Result<&'static [ShapeDef], ShapeError> {
        let family = bag_family(family_id)?;
        Ok(load(family, dir)?.0)
    }

    fn bag_family(id: &str) -> Result<&'static Family, ShapeError> {
        family::by_id(id)
            .filter(|f| !f.typed)
            .ok_or_else(|| ShapeError::UnknownFamily(id.to_string()))
    }

    pub(super) fn resolve(
        family_id: &'static str,
        generated: &'static [ShapeDef],
    ) -> &'static [ShapeDef] {
        env_loaded(family_id).map_or(generated, |l| l.0)
    }

    pub(super) fn resolve_profiles(
        family_id: &'static str,
        generated_iris: &'static [(&'static str, &'static str)],
        generated_codes: &'static [&'static str],
    ) -> (&'static [(&'static str, &'static str)], &'static [&'static str]) {
        env_loaded(family_id).map_or((generated_iris, generated_codes), |l| (l.1, l.2))
    }

    /// The loaded table for this family, from an explicit call or the
    /// environment, or `None` to use the generated one.
    fn env_loaded(family_id: &'static str) -> Option<Loaded> {
        RESOLVED.store(true, Ordering::SeqCst);

        if let Some(l) = explicit().lock().unwrap().get(family_id) {
            return Some(*l);
        }

        let mut cache = from_env().lock().unwrap();
        if let Some(cached) = cache.get(family_id) {
            return *cached;
        }

        let loaded = match std::env::var_os(SHACL_DIR_ENV) {
            None => None,
            Some(dir) => match bag_family(family_id) {
                Err(_) => None,
                Ok(family) => match load(family, &PathBuf::from(dir)) {
                    Ok(l) => Some(l),
                    Err(e) => {
                        eprintln!(
                            "warning: {SHACL_DIR_ENV} set but the {family_id} shapes could not \
                             be loaded ({e}); falling back to the generated shape table"
                        );
                        None
                    }
                },
            },
        };
        cache.insert(family_id, loaded);
        loaded
    }

    /// Read and intern one family's shapes and profile index.
    ///
    /// The shapes resolve against the family's RDFS, which
    /// `cimstructs::schema_source` already knows how to find: a shape naming a
    /// class this workspace has no schema for cannot be checked, so the two
    /// have to agree on what the family contains.
    fn load(family: &'static Family, dir: &Path) -> Result<Loaded, ShapeError> {
        // Fail on the SHACL directory before the schema one, so a bad path is
        // reported as what it is rather than as a missing vocabulary.
        if !dir.is_dir() {
            return Err(ShapeError::Parse(format!(
                "not a directory: {}",
                dir.display()
            )));
        }
        let (spec, others) = specs(family, dir)?;
        let other_refs: Vec<&cimschema::model::CimSpecification> = others.iter().collect();

        let mut collector = SkipCollector::new();
        let table = r::load_shape_table(family, &spec, &other_refs, dir, &mut collector)
            .map_err(|e| ShapeError::Parse(e.to_string()))?;

        let mut interner = Interner::default();
        let shapes: Vec<ShapeDef> = table
            .shapes
            .iter()
            .map(|s| shape(s, &mut interner))
            .collect();

        let iris: Vec<(&'static str, &'static str)> = table
            .profile_iris
            .iter()
            .map(|(iri, code)| (interner.intern(iri), interner.intern(code)))
            .collect();
        let codes: Vec<&'static str> =
            table.profiles.iter().map(|c| interner.intern(c)).collect();

        Ok((Vec::leak(shapes), Vec::leak(iris), Vec::leak(codes)))
    }

    /// Import the RDFS the shapes resolve against.
    ///
    /// Found the way `PROF` is: beside the SHACL directory, which is how the
    /// ENTSO-E library is laid out. That keeps a load self-contained — one
    /// directory decides both what the classes are and what the rules are —
    /// and avoids depending on the process's working directory, which the
    /// family's compiled-in glob does.
    ///
    /// `CIMOXIDE_RDFS_DIR` still wins, so a caller already pointing the class
    /// table somewhere is not silently overridden.
    fn specs(
        family: &'static Family,
        shacl_dir: &Path,
    ) -> Result<
        (cimschema::model::CimSpecification, Vec<cimschema::model::CimSpecification>),
        ShapeError,
    > {
        let import = |f: &'static Family, pattern: &str| {
            cimschema::import::import_schema_files(pattern, f, false)
                .map_err(|e| ShapeError::Schema(e.to_string()))
        };

        // `.../NCP/SHACL` → `.../`, the library root every family sits under.
        let root = shacl_dir.parent().and_then(Path::parent);

        let spec = import(family, &schema_glob(family, shacl_dir, root))?;

        // Every other family, for the cross-family class lists the value-type
        // rules use: NCP's association value-type shapes name CGMES classes
        // beside NC ones. A family whose schema cannot be read is skipped so
        // one missing tree does not fail the load — but it **warns**, because
        // the failure mode is silent: those classes simply drop out of the
        // allowed lists, and a reference that should have passed starts
        // reporting as the wrong class.
        let mut others = Vec::new();
        for f in family::FAMILIES.iter().filter(|f| f.id != family.id) {
            match import(f, &schema_glob(f, shacl_dir, root)) {
                Ok(s) => others.push(s),
                Err(e) => eprintln!(
                    "warning: the {} schema could not be read ({e}); its classes will be \
                     missing from {}'s cross-family value-type rules",
                    f.id, family.id
                ),
            }
        }
        Ok((spec, others))
    }

    /// Where to look for a family's RDFS.
    ///
    /// `CIMOXIDE_RDFS_DIR` wins, so a caller already pointing the class table
    /// somewhere is not silently overridden — but it names one family's
    /// directory, so it applies only to the family being loaded.
    ///
    /// Otherwise the family's compiled-in glob is re-rooted at the library the
    /// SHACL directory came from, which keeps a load self-contained and
    /// independent of the process's working directory.
    fn schema_glob(f: &'static Family, shacl_dir: &Path, root: Option<&Path>) -> String {
        let file_name = |p: &str| {
            Path::new(p)
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "*.rdf".to_string())
        };

        // Only for the family whose SHACL directory this is.
        if shacl_dir
            .parent()
            .and_then(|p| p.file_name())
            .zip(Path::new(f.default_schema).parent().and_then(Path::parent).and_then(|p| p.file_name()))
            .is_some_and(|(a, b)| a == b)
        {
            if let Some(dir) = std::env::var_os(cimstructs::schema_source::RDFS_DIR_ENV) {
                return PathBuf::from(dir)
                    .join(file_name(f.default_schema))
                    .to_string_lossy()
                    .into_owned();
            }
        }

        // `application-profiles-library/CGMES/RDFS/<glob>` → `<root>/CGMES/RDFS/<glob>`.
        let tail: Option<PathBuf> = Path::new(f.default_schema)
            .components()
            .skip(1)
            .fold(None::<PathBuf>, |acc, c| {
                Some(acc.map_or_else(|| PathBuf::from(c.as_os_str()), |p| p.join(c.as_os_str())))
            });
        match (root, tail) {
            (Some(root), Some(tail)) => {
                let candidate = root.join(tail);
                if candidate
                    .parent()
                    .is_some_and(|p| p.is_dir())
                {
                    return candidate.to_string_lossy().into_owned();
                }
                f.default_schema.to_string()
            }
            _ => f.default_schema.to_string(),
        }
    }

    /// Interns resolved strings so they satisfy the `&'static` signatures the
    /// IR and the violation types require. Bounded by schema size and leaked
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
            Vec::leak(items.iter().map(|s| self.intern(s)).collect::<Vec<_>>())
        }
    }

    // The conversions below are mechanical and total: the resolved model and
    // the IR have the same shape, and a variant added to one without the other
    // is a compile error rather than silent drift.

    fn shape(s: &r::ShapeDef, i: &mut Interner) -> ShapeDef {
        ShapeDef {
            targets: Vec::leak(s.targets.iter().map(|t| target(t, i)).collect::<Vec<_>>()),
            props: Vec::leak(
                s.props
                    .iter()
                    .map(|p| &*Box::leak(Box::new(prop(p, i))))
                    .collect::<Vec<_>>(),
            ),
            closed: s.closed.as_ref().map(|c| &*Box::leak(Box::new(closed(c, i)))),
            profiles: i.intern_all(&s.profiles),
            file: i.intern(&s.file),
        }
    }

    fn target(t: &r::Target, i: &mut Interner) -> Target {
        match t {
            r::Target::Class(classes) => Target::Class(i.intern_all(classes)),
            r::Target::SubjectsOf(f) => Target::SubjectsOf(i.intern(f)),
        }
    }

    fn prop(p: &r::PropShape, i: &mut Interner) -> PropShape {
        PropShape {
            path: path(&p.path, i),
            checks: Vec::leak(p.checks.iter().map(|c| check(c, i)).collect::<Vec<_>>()),
        }
    }

    fn path(p: &r::Path, i: &mut Interner) -> SPath {
        match p {
            r::Path::Forward(f) => SPath::Forward(i.intern(f)),
            r::Path::Inverse(f) => SPath::Inverse(i.intern(f)),
            r::Path::RefType(f) => SPath::RefType(i.intern(f)),
            r::Path::Alternative(branches) => SPath::Alternative(Vec::leak(
                branches
                    .iter()
                    .map(|b| match b {
                        r::AltBranch::Forward(f) => AltBranch::Forward(i.intern(f)),
                        r::AltBranch::Inverse(f) => AltBranch::Inverse(i.intern(f)),
                    })
                    .collect::<Vec<_>>(),
            )),
        }
    }

    fn check(c: &r::Check, i: &mut Interner) -> Check {
        Check {
            constraint: constraint(&c.constraint, i),
            rule_id: i.intern(&c.rule_id),
            name: i.intern(&c.name),
            message: i.intern(&c.message),
            description: i.intern(&c.description),
            severity: i.intern(&c.severity),
        }
    }

    fn constraint(c: &r::Constraint, i: &mut Interner) -> Constraint {
        match c {
            r::Constraint::MinCount(n) => Constraint::MinCount(*n),
            r::Constraint::MaxCount(n) => Constraint::MaxCount(*n),
            r::Constraint::MaxLength(n) => Constraint::MaxLength(*n),
            r::Constraint::MinLength(n) => Constraint::MinLength(*n),
            r::Constraint::Datatype(d) => Constraint::Datatype(i.intern(d)),
            r::Constraint::HasValue(v) => Constraint::HasValue(i.intern(v)),
            r::Constraint::NodeKind(k) => Constraint::NodeKind(match k {
                r::NodeKind::Iri => NodeKind::Iri,
                r::NodeKind::Literal => NodeKind::Literal,
                r::NodeKind::BlankNode => NodeKind::BlankNode,
            }),
            r::Constraint::Class(v) => Constraint::Class(i.intern_all(v)),
            r::Constraint::RefClass(v) => Constraint::RefClass(i.intern_all(v)),
            r::Constraint::In(v) => Constraint::In(i.intern_all(v)),
        }
    }

    fn closed(c: &r::ClosedShape, i: &mut Interner) -> ClosedShape {
        ClosedShape {
            allowed: i.intern_all(&c.allowed),
            rule_id: i.intern(&c.rule_id),
            name: i.intern(&c.name),
            message: i.intern(&c.message),
            description: i.intern(&c.description),
            severity: i.intern(&c.severity),
        }
    }
}
