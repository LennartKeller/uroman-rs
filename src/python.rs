//! Python bindings for uroman-rs using PyO3.
//!
//! This module provides Python wrappers for the main Uroman functionality,
//! allowing Python users to access the fast Rust romanization library.

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use std::io::{BufReader, Cursor};

use crate::{Uroman as RustUroman, RomFormat, Edge as RustEdge};

/// Python wrapper for the Uroman struct.
///
/// This class provides methods to romanize text in various writing systems
/// to Latin script.
///
/// # Example
/// ```python
/// from uroman import Uroman
///
/// uroman = Uroman()
/// result = uroman.romanize("こんにちは")
/// print(result)  # "kon'nichiha"
/// ```
#[pyclass]
#[derive(Clone)]
pub struct PyUroman {
    inner: RustUroman,
}

#[pymethods]
impl PyUroman {
    /// Creates a new Uroman instance.
    ///
    /// Returns:
    ///     Uroman: A new Uroman instance with romanization rules loaded.
    #[new]
    pub fn new() -> Self {
        Self {
            inner: RustUroman::new(),
        }
    }

    /// Romanizes a given string.
    ///
    /// Args:
    ///     text (str): The text to romanize.
    ///     lcode (str, optional): ISO 639-3 language code (e.g., 'jpn', 'ara', 'zho').
    ///     format (str, optional): Output format - 'str', 'edges', 'alts', or 'lattice'.
    ///                            Defaults to 'str'.
    ///
    /// Returns:
    ///     str or list: Romanized text as a string (for 'str' format) or
    ///                  a list of Edge objects (for other formats).
    ///
    /// Example:
    ///     >>> uroman = Uroman()
    ///     >>> uroman.romanize("こんにちは")
    ///     "kon'nichiha"
    ///     >>> uroman.romanize("مرحبا", lcode="ara")
    ///     "mrhba"
    #[pyo3(signature = (text, lcode=None, format="str"))]
    pub fn romanize(
        &self,
        text: &str,
        lcode: Option<&str>,
        format: &str,
    ) -> PyResult<PyObject> {
        let rom_format = match format {
            "str" => RomFormat::Str,
            "edges" => RomFormat::Edges,
            "alts" => RomFormat::Alts,
            "lattice" => RomFormat::Lattice,
            _ => return Err(PyValueError::new_err(
                "Invalid format. Must be 'str', 'edges', 'alts', or 'lattice'."
            )),
        };

        let result = self.inner.romanize_with_format(text, lcode, Some(rom_format));

        Python::with_gil(|py| {
            match result {
                crate::RomanizationResult::Str(s) => Ok(s.into_pyobject(py).unwrap().into_any().unbind()),
                crate::RomanizationResult::Edges(edges) => {
                    let py_edges: Vec<PyEdge> = edges.into_iter().map(PyEdge::from).collect();
                    Ok(py_edges.into_pyobject(py).unwrap().into_any().unbind())
                }
            }
        })
    }

    /// Romanizes text with Unicode escape sequences decoded first.
    ///
    /// Args:
    ///     text (str): The text to romanize (may contain \\uXXXX escape sequences).
    ///     lcode (str, optional): ISO 639-3 language code.
    ///     format (str, optional): Output format. Defaults to 'str'.
    ///
    /// Returns:
    ///     str or list: Romanized text.
    #[pyo3(signature = (text, lcode=None, format="str"))]
    pub fn romanize_escaped(
        &self,
        text: &str,
        lcode: Option<&str>,
        format: &str,
    ) -> PyResult<PyObject> {
        let rom_format = match format {
            "str" => RomFormat::Str,
            "edges" => RomFormat::Edges,
            "alts" => RomFormat::Alts,
            "lattice" => RomFormat::Lattice,
            _ => return Err(PyValueError::new_err(
                "Invalid format. Must be 'str', 'edges', 'alts', or 'lattice'."
            )),
        };

        let result = self.inner.romanize_escaped_with_format(text, lcode, Some(rom_format));

        Python::with_gil(|py| {
            match result {
                crate::RomanizationResult::Str(s) => Ok(s.into_pyobject(py).unwrap().into_any().unbind()),
                crate::RomanizationResult::Edges(edges) => {
                    let py_edges: Vec<PyEdge> = edges.into_iter().map(PyEdge::from).collect();
                    Ok(py_edges.into_pyobject(py).unwrap().into_any().unbind())
                }
            }
        })
    }

    /// Romanizes text from a string containing multiple lines.
    ///
    /// Args:
    ///     text (str): Multi-line text to romanize.
    ///     lcode (str, optional): ISO 639-3 language code.
    ///     format (str, optional): Output format. Defaults to 'str'.
    ///     decode_unicode (bool, optional): Whether to decode Unicode escapes. Defaults to False.
    ///
    /// Returns:
    ///     str: Romanized text with newlines preserved.
    #[pyo3(signature = (text, lcode=None, format="str", decode_unicode=false))]
    pub fn romanize_text(
        &self,
        text: &str,
        lcode: Option<&str>,
        format: &str,
        decode_unicode: bool,
    ) -> PyResult<String> {
        let rom_format = match format {
            "str" => RomFormat::Str,
            "edges" => RomFormat::Edges,
            "alts" => RomFormat::Alts,
            "lattice" => RomFormat::Lattice,
            _ => return Err(PyValueError::new_err(
                "Invalid format. Must be 'str', 'edges', 'alts', or 'lattice'."
            )),
        };

        let reader = BufReader::new(Cursor::new(text.as_bytes()));
        let mut output = Vec::new();

        self.inner
            .romanize_file(reader, &mut output, lcode, rom_format, None, decode_unicode, true)
            .map_err(|e| PyValueError::new_err(format!("Romanization error: {}", e)))?;

        String::from_utf8(output)
            .map_err(|e| PyValueError::new_err(format!("UTF-8 conversion error: {}", e)))
    }

    /// Returns a string representation of the Uroman instance.
    fn __repr__(&self) -> String {
        "Uroman()".to_string()
    }
}

/// Python wrapper for the Edge struct.
///
/// Represents a romanization edge with position and text information.
#[pyclass]
#[derive(Clone)]
pub struct PyEdge {
    #[pyo3(get)]
    pub start: usize,
    #[pyo3(get)]
    pub end: usize,
    #[pyo3(get)]
    pub text: String,
    #[pyo3(get)]
    pub edge_type: String,
    #[pyo3(get)]
    pub is_numeric: bool,
    #[pyo3(get)]
    pub value: Option<f64>,
    #[pyo3(get)]
    pub orig_text: Option<String>,
}

impl From<RustEdge> for PyEdge {
    fn from(edge: RustEdge) -> Self {
        let data = edge.get_data();
        PyEdge {
            start: data.start,
            end: data.end,
            text: data.txt.clone(),
            edge_type: data.r#type.clone(),
            is_numeric: edge.is_numeric(),
            value: edge.value(),
            orig_text: if edge.is_numeric() {
                Some(edge.orig_txt().to_string())
            } else {
                None
            },
        }
    }
}

#[pymethods]
impl PyEdge {
    /// Returns a string representation of the Edge.
    fn __repr__(&self) -> String {
        format!(
            "Edge(start={}, end={}, text='{}', type='{}')",
            self.start, self.end, self.text, self.edge_type
        )
    }

    /// Returns the romanized text of the edge.
    fn __str__(&self) -> String {
        self.text.clone()
    }
}

/// Python module for uroman-rs.
///
/// This module provides Python bindings for the uroman-rs library,
/// a fast universal romanization tool.
#[pymodule]
fn uroman_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add classes with clean names
    m.add_class::<PyUroman>()?;
    m.add_class::<PyEdge>()?;

    // Add aliases for better API
    m.add("Uroman", m.getattr("PyUroman")?)?;
    m.add("Edge", m.getattr("PyEdge")?)?;

    // Add module-level convenience function
    m.add_function(wrap_pyfunction!(romanize, m)?)?;

    Ok(())
}

/// Convenience function to romanize text without creating a Uroman instance.
///
/// Args:
///     text (str): The text to romanize.
///     lcode (str, optional): ISO 639-3 language code.
///     format (str, optional): Output format. Defaults to 'str'.
///
/// Returns:
///     str or list: Romanized text.
///
/// Example:
///     >>> import uroman
///     >>> uroman.romanize("こんにちは")
///     "kon'nichiha"
#[pyfunction]
#[pyo3(signature = (text, lcode=None, format="str"))]
fn romanize(text: &str, lcode: Option<&str>, format: &str) -> PyResult<PyObject> {
    let uroman = PyUroman::new();
    uroman.romanize(text, lcode, format)
}
