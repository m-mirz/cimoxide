use std::path::Path;

use pyo3::exceptions::{PyKeyError, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

use cimdecoder::{CimDataset, CimEntry};

fn map_err<E: std::fmt::Display>(e: E) -> PyErr {
    PyRuntimeError::new_err(e.to_string())
}

fn entry_to_python(py: Python<'_>, entry: &CimEntry) -> PyResult<PyObject> {
    let mut val = entry.element.to_json_value();
    if let Some(obj) = val.as_object_mut() {
        obj.insert("_type".to_string(), entry.element.type_name().into());
    }
    pythonize::pythonize(py, &val)
        .map(|b| b.unbind())
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))
}

/// An iterator over the MRIDs in a CimDataset.
#[pyclass]
struct PyCimDatasetIter {
    mrids: std::vec::IntoIter<String>,
}

#[pymethods]
impl PyCimDatasetIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<String> {
        slf.mrids.next()
    }
}

/// A parsed CGMES dataset, keyed by MRID.
///
/// Obtain via `CimDataset.decode_file`, `decode_files`, or `decode_str`.
/// Each element is returned as a plain Python `dict` with a `"_type"` key and
/// one entry per CIM attribute (snake_case field names, values matching the
/// JSON serialization of the Rust structs).
#[pyclass]
pub struct PyCimDataset {
    inner: std::sync::Mutex<CimDataset>,
}

impl PyCimDataset {
    fn lock(&self) -> PyResult<std::sync::MutexGuard<'_, CimDataset>> {
        self.inner.lock().map_err(|e| map_err(e.to_string()))
    }
}

#[pymethods]
impl PyCimDataset {
    /// Parse a single CGMES RDF/XML file.
    #[staticmethod]
    fn decode_file(path: &str) -> PyResult<Self> {
        let ds = CimDataset::decode_file(Path::new(path)).map_err(map_err)?;
        Ok(Self {
            inner: std::sync::Mutex::new(ds),
        })
    }

    /// Parse multiple CGMES RDF/XML files, merging them into one dataset.
    #[staticmethod]
    fn decode_files(paths: Vec<String>) -> PyResult<Self> {
        let path_bufs: Vec<std::path::PathBuf> =
            paths.iter().map(std::path::PathBuf::from).collect();
        let path_refs: Vec<&Path> = path_bufs.iter().map(|p| p.as_path()).collect();
        let ds = CimDataset::decode_files(&path_refs).map_err(map_err)?;
        Ok(Self {
            inner: std::sync::Mutex::new(ds),
        })
    }

    /// Parse CGMES RDF/XML from a string.
    #[staticmethod]
    fn decode_str(content: &str) -> PyResult<Self> {
        let ds = CimDataset::decode_str(content).map_err(map_err)?;
        Ok(Self {
            inner: std::sync::Mutex::new(ds),
        })
    }

    /// Merge another dataset into this one (other becomes empty after the call).
    ///
    /// Scalar fields: last-wins. ResourceList fields: union.
    /// Do not pass the same object as both self and other.
    fn merge(&self, py: Python<'_>, other: Py<PyCimDataset>) -> PyResult<()> {
        let other_ds = {
            let borrowed = other.borrow(py);
            let mut guard = borrowed.lock()?;
            std::mem::replace(&mut *guard, CimDataset::new())
        };
        self.lock()?.merge(other_ds);
        Ok(())
    }

    /// Release all RdfBlock memory after the final merge.
    fn drop_blocks(&self) -> PyResult<()> {
        self.lock()?.drop_blocks();
        Ok(())
    }

    fn __len__(&self) -> PyResult<usize> {
        Ok(self.lock()?.entries.len())
    }

    fn __contains__(&self, mrid: &str) -> PyResult<bool> {
        Ok(self.lock()?.entries.contains_key(mrid))
    }

    /// Return the element dict for the given MRID. Raises `KeyError` if not found.
    fn __getitem__(&self, py: Python<'_>, mrid: &str) -> PyResult<PyObject> {
        let ds = self.lock()?;
        match ds.entries.get(mrid) {
            Some(entry) => entry_to_python(py, entry),
            None => Err(PyKeyError::new_err(mrid.to_string())),
        }
    }

    /// Insert or replace the element at `mrid`.
    ///
    /// `value` must be a dict shaped like the ones returned by `__getitem__`: a
    /// `"_type"` key naming a known CIM class, plus attribute keys. Raises
    /// `ValueError` if `"_type"` is missing or not a recognized CIM type.
    fn __setitem__(&self, py: Python<'_>, mrid: String, value: Py<PyAny>) -> PyResult<()> {
        let json_val: serde_json::Value =
            pythonize::depythonize(value.bind(py)).map_err(map_err)?;
        let type_name = json_val
            .get("_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| PyValueError::new_err("element dict missing \"_type\" key"))?
            .to_string();
        let reg = cimstructs::registry::json_registry();
        let ctor = reg
            .get(type_name.as_str())
            .ok_or_else(|| PyValueError::new_err(format!("unknown CIM type \"{type_name}\"")))?;
        let element = ctor(json_val).map_err(map_err)?;
        self.lock()?.set(mrid, element);
        Ok(())
    }

    /// Remove the element at `mrid`. Raises `KeyError` if not found.
    fn __delitem__(&self, mrid: &str) -> PyResult<()> {
        match self.lock()?.remove(mrid) {
            Some(_) => Ok(()),
            None => Err(PyKeyError::new_err(mrid.to_string())),
        }
    }

    /// Iterate over all MRIDs in the dataset.
    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<PyCimDatasetIter> {
        let mrids: Vec<String> = slf.lock()?.entries.keys().cloned().collect();
        Ok(PyCimDatasetIter {
            mrids: mrids.into_iter(),
        })
    }

    /// Return the element dict for the given MRID, or `None` if not found.
    fn get(&self, py: Python<'_>, mrid: &str) -> PyResult<Option<PyObject>> {
        let ds = self.lock()?;
        match ds.entries.get(mrid) {
            Some(entry) => Ok(Some(entry_to_python(py, entry)?)),
            None => Ok(None),
        }
    }

    /// Return all MRIDs as a list.
    fn mrids(&self) -> PyResult<Vec<String>> {
        Ok(self.lock()?.entries.keys().cloned().collect())
    }

    /// Return a `dict[str, list[str]]` mapping type names to lists of MRIDs.
    ///
    /// No elements are deserialized, but the whole index is copied out: one
    /// Python `str` per MRID in the dataset, across every type. To count a
    /// single type, use `count_type` instead — it copies nothing.
    fn by_type(&self, py: Python<'_>) -> PyResult<PyObject> {
        let ds = self.lock()?;
        let dict = PyDict::new(py);
        for (type_name, mrids) in &ds.by_type {
            dict.set_item(type_name, mrids)?;
        }
        Ok(dict.into_any().unbind())
    }

    /// Return the number of elements of the given CIM type (0 if unknown).
    ///
    /// An O(1) index lookup that copies nothing into Python — use this instead
    /// of `len(ds.by_type()[name])`, which materialises the entire index.
    fn count_type(&self, type_name: &str) -> PyResult<usize> {
        let ds = self.lock()?;
        Ok(ds.by_type.get(type_name).map_or(0, |mrids| mrids.len()))
    }

    /// Return a list of element dicts for all objects of the given CIM type name.
    ///
    /// Example: `ds.get_type("ACLineSegment")` → `[{"_type": "ACLineSegment", "r": 0.12, ...}, ...]`
    fn get_type(&self, py: Python<'_>, type_name: &str) -> PyResult<Vec<PyObject>> {
        let ds = self.lock()?;
        let mrids = ds.by_type.get(type_name).cloned().unwrap_or_default();
        let mut result = Vec::with_capacity(mrids.len());
        for mrid in &mrids {
            if let Some(entry) = ds.entries.get(mrid) {
                result.push(entry_to_python(py, entry)?);
            }
        }
        Ok(result)
    }

    /// Return all entries as a `dict[str, dict]` (MRID → element dict).
    ///
    /// This deserializes every element — prefer `get_type` or `__getitem__` for
    /// partial access.
    fn entries(&self, py: Python<'_>) -> PyResult<PyObject> {
        let ds = self.lock()?;
        let dict = PyDict::new(py);
        for (mrid, entry) in &ds.entries {
            dict.set_item(mrid, entry_to_python(py, entry)?)?;
        }
        Ok(dict.into_any().unbind())
    }

    /// Encode this dataset as a single CGMES profile's RDF/XML text.
    ///
    /// Only elements/fields whose CIM schema origin includes `profile` (e.g.
    /// `"EQ"`, `"SSH"`) are emitted. If the dataset contains a decoded `FullModel`
    /// header for this profile, it is reused verbatim; otherwise a minimal
    /// synthetic header is generated.
    fn to_xml_for_profile(&self, profile: &str) -> PyResult<String> {
        let ds = self.lock()?;
        cimconvert::dataset_to_xml_for_profile(&ds, profile).map_err(map_err)
    }

    /// Run a SPARQL 1.1 query over this dataset.
    ///
    /// The dataset is materialised into an in-memory RDF graph on every call, so hold on
    /// to the results rather than querying in a tight loop. The CGMES namespaces
    /// (`cim:`, `eu:`, `md:`, `dm:`, `rdf:`) and `xsd:` are pre-bound.
    ///
    /// Returns a list of dicts for SELECT, a bool for ASK, and a list of
    /// `(subject, predicate, object)` string triples for CONSTRUCT/DESCRIBE.
    #[cfg(feature = "sparql")]
    fn query(&self, py: Python<'_>, sparql: &str) -> PyResult<PyObject> {
        use cimsparql::QueryResults;

        let ds = self.lock()?;
        let store = cimsparql::CimStore::from_dataset(&ds).map_err(map_err)?;
        match store.query(sparql).map_err(map_err)? {
            QueryResults::Boolean(b) => Ok(b.into_pyobject(py)?.to_owned().unbind().into()),
            QueryResults::Solutions(solutions) => {
                let variables: Vec<String> =
                    solutions.variables().iter().map(|v| v.as_str().to_string()).collect();
                let rows = PyList::empty(py);
                for solution in solutions {
                    let solution = solution.map_err(map_err)?;
                    let row = PyDict::new(py);
                    for name in &variables {
                        if let Some(term) = solution.get(name.as_str()) {
                            row.set_item(name, term_to_string(term))?;
                        }
                    }
                    rows.append(row)?;
                }
                Ok(rows.into())
            }
            QueryResults::Graph(triples) => {
                let rows = PyList::empty(py);
                for triple in triples {
                    let t = triple.map_err(map_err)?;
                    rows.append((
                        node_to_string(&t.subject),
                        t.predicate.as_str().to_string(),
                        term_to_string(&t.object),
                    ))?;
                }
                Ok(rows.into())
            }
        }
    }

    /// Encode and write one RDF/XML file per profile into `dir`.
    ///
    /// Creates `dir` (and parents) if it doesn't exist, then writes
    /// `dir/{profile}.xml` for each entry in `profiles`, e.g. `["EQ", "SSH"]` ->
    /// `dir/EQ.xml`, `dir/SSH.xml`.
    fn write_xml_files(&self, dir: &str, profiles: Vec<String>) -> PyResult<()> {
        let ds = self.lock()?;
        let dir_path = Path::new(dir);
        std::fs::create_dir_all(dir_path).map_err(map_err)?;
        for profile in &profiles {
            let xml = cimconvert::dataset_to_xml_for_profile(&ds, profile).map_err(map_err)?;
            let path = dir_path.join(format!("{profile}.xml"));
            std::fs::write(&path, &xml).map_err(map_err)?;
        }
        Ok(())
    }
}

/// Lexical value of a term: IRIs and literal values as plain strings, so Python callers get
/// `"urn:uuid:..."` and `"2.2"` rather than N-Triples decoration.
#[cfg(feature = "sparql")]
fn term_to_string(term: &cimsparql::Term) -> String {
    match term {
        cimsparql::Term::NamedNode(n) => n.as_str().to_string(),
        cimsparql::Term::BlankNode(b) => format!("_:{}", b.as_str()),
        cimsparql::Term::Literal(l) => l.value().to_string(),
    }
}

#[cfg(feature = "sparql")]
fn node_to_string(node: &cimsparql::NamedOrBlankNode) -> String {
    match node {
        cimsparql::NamedOrBlankNode::NamedNode(n) => n.as_str().to_string(),
        cimsparql::NamedOrBlankNode::BlankNode(b) => format!("_:{}", b.as_str()),
    }
}

fn violations_to_py(violations: Vec<cimvalidation::Violation>) -> Vec<PyViolation> {
    violations
        .into_iter()
        .map(|v| PyViolation {
            object_id:   v.object_id,
            rule_id:     v.rule_id,
            name:        v.name,
            class:       v.class,
            property:    v.property,
            message:     v.message,
            severity:    v.severity,
            description: v.description,
        })
        .collect()
}

/// A single SHACL or custom validation finding.
#[pyclass(frozen)]
pub struct PyViolation {
    pub object_id:   String,
    pub rule_id:     String,
    pub name:        String,
    pub class:       String,
    pub property:    String,
    pub message:     String,
    pub severity:    String,
    pub description: String,
}

#[pymethods]
impl PyViolation {
    #[getter] fn object_id(&self)   -> &str { &self.object_id }
    #[getter] fn rule_id(&self)     -> &str { &self.rule_id }
    #[getter] fn name(&self)        -> &str { &self.name }
    /// The CIM class of the offending element. Named ``class_`` to avoid the Python keyword.
    #[getter] fn class_(&self)      -> &str { &self.class }
    #[getter] fn property(&self)    -> &str { &self.property }
    #[getter] fn message(&self)     -> &str { &self.message }
    #[getter] fn severity(&self)    -> &str { &self.severity }
    #[getter] fn description(&self) -> &str { &self.description }

    fn __repr__(&self) -> String {
        format!("[{}] {} — {} ({})", self.severity, self.rule_id, self.message, self.object_id)
    }
}

/// Validate a set of CGMES profile files using two-phase validation.
///
/// Phase 1 runs per-profile (local) SHACL and SPARQL rules against each file's
/// individual dataset before merging. Phase 2 runs cross-profile rules on the
/// merged dataset. This is the recommended entry point for validation.
///
/// Parameters
/// ----------
/// paths:
///     Paths to the CGMES RDF/XML files to validate.
/// profiles:
///     Profile short names to check, e.g. ``["EQ", "SSH"]``.
///     ``None`` (default) uses auto-detected profiles.
/// solved:
///     ``True`` forces solved-case checks; ``False`` forces not-solved checks;
///     ``None`` (default) auto-detects from the dataset.
/// common:
///     Enable cross-profile common checks (default ``False``).
/// quality:
///     Enable CIMdesk modeling quality checks (default ``False``).
/// silence:
///     Rule IDs to suppress, e.g. ``["Rule-EQ-1", "Rule-EQ-2"]``.
#[pyfunction]
#[pyo3(signature = (paths, profiles=None, solved=None, common=false, quality=false, silence=None))]
fn validate_files(
    paths: Vec<String>,
    profiles: Option<Vec<String>>,
    solved: Option<bool>,
    common: bool,
    quality: bool,
    silence: Option<Vec<String>>,
) -> PyResult<Vec<PyViolation>> {
    // Decode each file into its own dataset, in parallel.
    let path_bufs: Vec<std::path::PathBuf> = paths.iter().map(std::path::PathBuf::from).collect();
    let path_refs: Vec<&Path> = path_bufs.iter().map(|p| p.as_path()).collect();
    let per_file: Vec<CimDataset> =
        CimDataset::decode_files_parallel_separate(&path_refs).map_err(map_err)?;

    let cfg = cimvalidation::combined_config(
        &per_file,
        profiles,
        solved,
        common,
        quality,
        silence.unwrap_or_default(),
    );
    let violations = cimvalidation::validate_files(per_file, &cfg);

    Ok(violations_to_py(violations))
}

#[pymodule]
fn cimoxide(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCimDataset>()?;
    m.add_class::<PyCimDatasetIter>()?;
    m.add_class::<PyViolation>()?;
    m.add_function(wrap_pyfunction!(validate_files, m)?)?;
    Ok(())
}
