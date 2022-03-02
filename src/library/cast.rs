use std::collections::HashMap;

use pyo3::PyObjectProtocol;
use pyo3::prelude::*;

use crate::database::DATABASE;

use super::movie::MovieResult;
use super::movie::MovieSearch;
use super::tv::Episode;
use super::tv::EpisodeSearch;
use super::tv::TvSearch;

#[pyclass]
#[derive(Debug, Serialize, Clone)]
pub struct Crew{
    #[pyo3(get)]
    pub user: String,
    #[pyo3(get)]
    pub id: u64,
    #[pyo3(get)]
    pub job: String,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub profile_path: String,
}

#[pymethods]
impl Crew{
    pub fn full(&self) -> PyResult<Person>{
        Ok(DATABASE.get_person(&self.user, self.id)?.unwrap())
    }
}

#[pyclass]
#[derive(Debug, Serialize, Clone)]
pub struct Cast{
    #[pyo3(get)]
    pub user: String,
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

#[pymethods]
impl Cast{
    pub fn full(&self) -> PyResult<Person>{
        Ok(DATABASE.get_person(&self.user, self.id)?.unwrap())
    }
}

#[pyclass]
#[derive(Debug, Serialize, Clone)]
pub struct Person{
    pub user: String,
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
    #[pyo3(get)]
    pub cast_movie: Vec<MovieResult>,
    #[pyo3(get)]
    pub crew_movie: Vec<MovieResult>,
    #[pyo3(get)]
    pub cast_tv: Vec<Episode>,
    #[pyo3(get)]
    pub crew_tv: Vec<Episode>,
}

#[pymethods]
impl Person{
    pub fn set_movie(&mut self) -> PyResult<()>{
        self.cast_movie = MovieSearch::new(&self.user).cast(self.id)?.results()?;
        self.crew_movie = MovieSearch::new(&self.user).crew(self.id)?.results()?;
        Ok(())
    }

    pub fn set_tv(&mut self) -> PyResult<()>{
        self.cast_tv = EpisodeSearch::new(&self.user).cast(self.id)?.results()?;
        self.crew_tv = EpisodeSearch::new(&self.user).crew(self.id)?.results()?;
        Ok(())
    }

    pub fn delete(&mut self)  -> PyResult<()>{
        if MovieSearch::new(&self.user).cast(self.id)?.exist()?{
            return Ok(())
        }
        if MovieSearch::new(&self.user).crew(self.id)?.exist()?{
            return Ok(())
        }
        if TvSearch::new(&self.user).cast(self.id)?.exist()?{
            return Ok(())
        }
        if TvSearch::new(&self.user).crew(self.id)?.exist()?{
            return Ok(())
        }
        
        DATABASE.delete_person(self.id)?;

        return Ok(())
        
    }

    pub fn json(&self) -> PyResult<String>{
        return Ok(serde_json::to_string(self).unwrap())
    }
}

#[pyclass]
#[derive(Debug, Serialize, Clone)]
pub struct PersonResult{
    pub user: String,
    #[pyo3(get)]
    pub id: u64,
    #[pyo3(get)]
    pub birthday: String,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub profile_path: String,
}

#[pymethods]
impl PersonResult{
    pub fn full(&self) -> PyResult<Person>{
        Ok(DATABASE.get_person(&self.user, self.id)?.unwrap())
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PersonSearch{
   parameters: HashMap<String, Option<(String, String)>>,
   user: String,
}

impl PersonSearch{
    pub fn new(user: &String) -> PersonSearch{
        PersonSearch{
            parameters: HashMap::new(),
            user: user.clone(),
        }
    }
}

#[pymethods]
impl PersonSearch{
    pub fn id(&mut self, id: u64) -> PyResult<PersonSearch>{
        self.find("Persons.id", "=", Some(id.to_string()))
    }

    pub fn find(&mut self, column: &str, operator: &str, value: Option<String>) -> PyResult<PersonSearch>{
        if let Some(value) = value {
            self.parameters.insert(column.to_string(), Some((operator.to_string(), value)));
        }
        else{
            self.parameters.insert(column.to_string(), None);
        }
        Ok(self.clone())
    }

    pub fn exist(&self) -> PyResult<bool>{
        Ok(self.results()?.len() > 0)
    }

    pub fn results(&self) -> PyResult<Vec<PersonResult>>{
        Ok(DATABASE.get_persons(&self.user, &self.parameters)?)
    }

    pub fn json_results(&self) -> PyResult<String>{
        let list = self.results()?;
        Ok(serde_json::to_string(&list).unwrap())
    }
}

#[pyproto]
impl PyObjectProtocol for PersonSearch {
    fn __str__(&self) -> PyResult<String>{
        Ok(format!("{:?}", self))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
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
impl PyObjectProtocol for PersonResult {
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