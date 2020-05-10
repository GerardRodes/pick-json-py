use cpython::{PyResult, Python, py_module_initializer, py_fn, PyErr};
use pick_json;

py_module_initializer!(pick_json_py, |py, m| {
  m.add(py, "__doc__", "This module is implemented in Rust.")?;
  m.add(py, "pick_json", py_fn!(py, pick_json_py(filepath: &str, property: &str)))?;
  Ok(())
});

fn pick_json_py(py: Python, filepath: &str, property: &str) -> PyResult<String> {
  match pick_json::pick_json(filepath, property) {
    Ok(v) => Ok(v),
    Err(e) => return Err(PyErr::new::<cpython::exc::ReferenceError, _>(py, format!("{}", e))),
  }
}
