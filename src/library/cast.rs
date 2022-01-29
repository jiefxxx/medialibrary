use pyo3::PyObjectProtocol;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Crew{
    #[pyo3(get)]
    pub id: u64,
    #[pyo3(get)]
    pub job: String,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub profile_path: String,
}

#[pyclass]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Cast{
    #[pyo3(get)]
    pub id: u64,
    #[pyo3(get)]
    pub character: String,
    #[pyo3(get)]
    pub ord: u64,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub profile_path: String,
}

#[pyclass]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Person{
    #[pyo3(get)]
    pub id: u64,
    #[pyo3(get)]
    pub birthday: String,
    #[pyo3(get)]
    pub known_for_department: String,
    #[pyo3(get)]
    pub deathday: String,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub gender: u64,
    #[pyo3(get)]
    pub biography: String,
    #[pyo3(get)]
    pub popularity: f64,
    #[pyo3(get)]
    pub place_of_birth: String,
    #[pyo3(get)]
    pub profile_path: String,
}

#[pyproto]
impl PyObjectProtocol for Cast {
    fn __str__(&self) -> PyResult<String>{
        Ok(format!("{:?}", self))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

#[pyproto]
impl PyObjectProtocol for Person {
    fn __str__(&self) -> PyResult<String>{
        Ok(format!("{:?}", self))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

#[pyproto]
impl PyObjectProtocol for Crew {
    fn __str__(&self) -> PyResult<String>{
        Ok(format!("{:?}", self))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}