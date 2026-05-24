use std::path::Path;

use pyo3::exceptions::{PyKeyError, PyRuntimeError};
use pyo3::prelude::*;
use pyo3::types::PyDict;

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
            let mut guard = borrowed
                .inner
                .lock()
                .map_err(|e| map_err(e.to_string()))?;
            std::mem::replace(&mut *guard, CimDataset::new())
        };
        self.inner
            .lock()
            .map_err(|e| map_err(e.to_string()))?
            .merge(other_ds);
        Ok(())
    }

    /// Release all RdfBlock memory after the final merge.
    fn drop_blocks(&self) -> PyResult<()> {
        self.inner
            .lock()
            .map_err(|e| map_err(e.to_string()))?
            .drop_blocks();
        Ok(())
    }

    fn __len__(&self) -> PyResult<usize> {
        Ok(self
            .inner
            .lock()
            .map_err(|e| map_err(e.to_string()))?
            .entries
            .len())
    }

    fn __contains__(&self, mrid: &str) -> PyResult<bool> {
        Ok(self
            .inner
            .lock()
            .map_err(|e| map_err(e.to_string()))?
            .entries
            .contains_key(mrid))
    }

    /// Return the element dict for the given MRID. Raises `KeyError` if not found.
    fn __getitem__(&self, py: Python<'_>, mrid: &str) -> PyResult<PyObject> {
        let ds = self.inner.lock().map_err(|e| map_err(e.to_string()))?;
        match ds.entries.get(mrid) {
            Some(entry) => entry_to_python(py, entry),
            None => Err(PyKeyError::new_err(mrid.to_string())),
        }
    }

    /// Iterate over all MRIDs in the dataset.
    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<PyCimDatasetIter> {
        let mrids: Vec<String> = slf
            .inner
            .lock()
            .map_err(|e| map_err(e.to_string()))?
            .entries
            .keys()
            .cloned()
            .collect();
        Ok(PyCimDatasetIter {
            mrids: mrids.into_iter(),
        })
    }

    /// Return the element dict for the given MRID, or `None` if not found.
    fn get(&self, py: Python<'_>, mrid: &str) -> PyResult<Option<PyObject>> {
        let ds = self.inner.lock().map_err(|e| map_err(e.to_string()))?;
        match ds.entries.get(mrid) {
            Some(entry) => Ok(Some(entry_to_python(py, entry)?)),
            None => Ok(None),
        }
    }

    /// Return all MRIDs as a list.
    fn mrids(&self) -> PyResult<Vec<String>> {
        Ok(self
            .inner
            .lock()
            .map_err(|e| map_err(e.to_string()))?
            .entries
            .keys()
            .cloned()
            .collect())
    }

    /// Return a `dict[str, list[str]]` mapping type names to lists of MRIDs.
    ///
    /// This is a fast O(1) index lookup — no element deserialization occurs.
    fn by_type(&self, py: Python<'_>) -> PyResult<PyObject> {
        let ds = self.inner.lock().map_err(|e| map_err(e.to_string()))?;
        let dict = PyDict::new(py);
        for (type_name, mrids) in &ds.by_type {
            dict.set_item(type_name, mrids)?;
        }
        Ok(dict.into_any().unbind())
    }

    /// Return a list of element dicts for all objects of the given CIM type name.
    ///
    /// Example: `ds.get_type("ACLineSegment")` → `[{"_type": "ACLineSegment", "r": 0.12, ...}, ...]`
    fn get_type(&self, py: Python<'_>, type_name: &str) -> PyResult<Vec<PyObject>> {
        let ds = self.inner.lock().map_err(|e| map_err(e.to_string()))?;
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
        let ds = self.inner.lock().map_err(|e| map_err(e.to_string()))?;
        let dict = PyDict::new(py);
        for (mrid, entry) in &ds.entries {
            dict.set_item(mrid, entry_to_python(py, entry)?)?;
        }
        Ok(dict.into_any().unbind())
    }
}

#[pymodule]
fn cimoxide(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCimDataset>()?;
    m.add_class::<PyCimDatasetIter>()?;
    Ok(())
}
