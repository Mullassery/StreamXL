use pyo3::prelude::*;
use pyo3::types::PyList;
use streamxl_core::sheet_parser::CellValue;
use streamxl_core::XlsxStream;

fn cell_to_pyobject(py: Python<'_>, cell: &CellValue) -> PyResult<PyObject> {
    match cell {
        CellValue::String(s) => Ok(s.clone().into_pyobject(py)?.into_any().unbind()),
        CellValue::Number(n) => Ok(n.into_pyobject(py)?.into_any().unbind()),
        // bool's into_pyobject returns Borrowed (singleton True/False); use as_any + clone
        CellValue::Bool(b) => Ok(b.into_pyobject(py)?.as_any().clone().unbind()),
        CellValue::Date(d) => Ok(d.clone().into_pyobject(py)?.into_any().unbind()),
        CellValue::Empty => Ok(py.None()),
    }
}

#[pyfunction]
#[pyo3(signature = (path, sheet=None))]
fn read(py: Python<'_>, path: &str, sheet: Option<&str>) -> PyResult<Py<PyList>> {
    let stream = if let Some(sheet_name) = sheet {
        XlsxStream::open_sheet(path, Some(sheet_name))
    } else {
        XlsxStream::open(path)
    }
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

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read, m)?)?;
    Ok(())
}
