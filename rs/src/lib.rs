use cpython::{py_fn, py_module_initializer, PyNone, PyResult, Python};

py_module_initializer!(vault, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "test", py_fn!(py, run()))?;
    Ok(())
});

fn run(_: Python) -> PyResult<PyNone> {
    println!("Yay! This message is implemented in Rust.");
    Ok(PyNone)
}
