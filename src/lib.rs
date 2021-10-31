use pyo3::prelude::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[pyfunction]
fn say_hello(path: String) {
    println!("Hello world! {:?}", path)
}

#[pymodule]
fn medialibrary(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(say_hello, module)?)
}