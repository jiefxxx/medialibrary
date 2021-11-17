#[macro_use]
extern crate serde_derive;

mod database;
mod rustmdb;
mod library;

use pyo3::prelude::*;
use pyo3::create_exception;
use pyo3::exceptions::PyException;


use library::Library;
use library::video::Video;

create_exception!(medialibrary, TmdbError, PyException);

impl std::convert::From<rustmdb::Error> for PyErr {
    fn from(err: rustmdb::Error) -> PyErr {
        TmdbError::new_err(err.to_string())
    }
}

#[pyfunction]
fn say_hello(path: &str)  -> PyResult<()> {
    println!("Hello world! {:?}", path);
    Ok(())
}

#[pymodule]
fn medialibrary(py: Python, module: &PyModule) -> PyResult<()> {
    module.add("TmdbError", py.get_type::<TmdbError>())?;
    module.add_function(wrap_pyfunction!(say_hello, module)?)?;
    module.add_class::<Library>()?;
    module.add_class::<Video>()?;
    Ok(())
}
