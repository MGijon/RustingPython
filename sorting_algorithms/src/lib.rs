#[macro_use]
extern crate cpython;

use cpython::{PyResult, Python};

fn colour(_py: Python, val: String) -> PyResult<String>{
    match &*val{
        "online" => Ok("green".to_string()),
        _ => Ok("red".to_string()),
    }
}
py_module_initializer!(sorting_algorithms, initsorting_algorithms, Pyinit_sorting_algorithms, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "colour", py_fn!(py, colour(val: String)))?;
    Ok(())
});
