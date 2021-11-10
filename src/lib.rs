#[macro_use]
extern crate serde_derive;

mod database;
mod rustmdb;
mod library;

use pyo3::prelude::*;

use library::Library;
use library::video::Video;


#[pyfunction]
fn say_hello(path: &str)  -> PyResult<()> {
    println!("Hello world! {:?}", path);
    Ok(())
}

#[pymodule]
fn medialibrary(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(say_hello, module)?)?;
    module.add_class::<Library>()?;
    module.add_class::<Video>()?;
    Ok(())
}
