use pyo3::prelude::*;
use pyo3::types::PyList;
use streamxl_core::sheet_parser::CellValue;
use streamxl_core::writer::WriteCell;
use streamxl_core::{XlsxStream, XlsxWriter};

// ── Reading ───────────────────────────────────────────────────────────────────

fn cell_to_pyobject(py: Python<'_>, cell: &CellValue) -> PyResult<PyObject> {
    match cell {
        CellValue::String(s) => Ok(s.clone().into_pyobject(py)?.into_any().unbind()),
        CellValue::Number(n) => Ok(n.into_pyobject(py)?.into_any().unbind()),
        CellValue::Bool(b) => Ok(b.into_pyobject(py)?.as_any().clone().unbind()),
        CellValue::Empty => Ok(py.None()),
    }
}

#[pyfunction]
fn read(py: Python<'_>, path: &str) -> PyResult<Py<PyList>> {
    let stream = XlsxStream::open(path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

    let result = PyList::empty(py);
    for row_result in stream.rows() {
        let row = row_result
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        let py_row = PyList::empty(py);
        for cell in &row {
            py_row.append(cell_to_pyobject(py, cell)?)?;
        }
        result.append(py_row)?;
    }
    Ok(result.into())
}

// ── Writing ───────────────────────────────────────────────────────────────────

fn pyobject_to_writecell(py: Python<'_>, obj: &PyObject) -> PyResult<WriteCell> {
    let bound = obj.bind(py);

    if bound.is_none() {
        return Ok(WriteCell::Empty);
    }
    if let Ok(b) = bound.extract::<bool>() {
        return Ok(WriteCell::Bool(b));
    }
    if let Ok(n) = bound.extract::<f64>() {
        return Ok(WriteCell::Num(n));
    }
    if let Ok(s) = bound.extract::<String>() {
        return Ok(WriteCell::Str(s));
    }
    // Fallback: convert to string via Python str()
    Ok(WriteCell::Str(bound.str()?.to_string()))
}

/// Write all rows to an XLSX file in one call.
///
/// rows: iterable of iterables of Python values (str, int, float, bool, None).
#[pyfunction]
fn write(py: Python<'_>, path: &str, rows: PyObject) -> PyResult<()> {
    let mut writer = XlsxWriter::new(path);

    let iter = rows.bind(py).try_iter()?;
    for row_obj in iter {
        let row_obj = row_obj?;
        let row_iter = row_obj.try_iter()?;
        let mut cells: Vec<WriteCell> = Vec::new();
        for cell_obj in row_iter {
            let cell = cell_obj?.unbind();
            cells.push(pyobject_to_writecell(py, &cell)?);
        }
        writer.write_row(&cells);
    }

    writer
        .finish()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))
}

/// Context-manager writer for streaming large files row by row.
///
///     with streamxl.writer("out.xlsx") as w:
///         w.write_row(["Name", "Age"])
///         w.write_row(["Alice", 30])
#[pyclass]
struct PyXlsxWriter {
    inner: Option<XlsxWriter>,
}

#[pymethods]
impl PyXlsxWriter {
    #[new]
    fn new(path: &str) -> Self {
        Self {
            inner: Some(XlsxWriter::new(path)),
        }
    }

    fn write_row(&mut self, py: Python<'_>, row: PyObject) -> PyResult<()> {
        let writer = self.inner.as_mut().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("writer already closed")
        })?;

        let iter = row.bind(py).try_iter()?;
        let mut cells: Vec<WriteCell> = Vec::new();
        for item in iter {
            let cell = item?.unbind();
            cells.push(pyobject_to_writecell(py, &cell)?);
        }
        writer.write_row(&cells);
        Ok(())
    }

    fn close(&mut self) -> PyResult<()> {
        if let Some(w) = self.inner.take() {
            w.finish()
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        }
        Ok(())
    }

    fn __enter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __exit__(
        &mut self,
        _exc_type: PyObject,
        _exc_val: PyObject,
        _exc_tb: PyObject,
    ) -> PyResult<bool> {
        self.close()?;
        Ok(false)
    }
}

// ── Module ────────────────────────────────────────────────────────────────────

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read, m)?)?;
    m.add_function(wrap_pyfunction!(write, m)?)?;
    m.add_class::<PyXlsxWriter>()?;
    Ok(())
}
