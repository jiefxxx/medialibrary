use std::collections::HashMap;

use pyo3::PyObjectProtocol;
use pyo3::prelude::*;

use crate::database::DATABASE;

use super::movie::MovieResult;
use super::movie::MovieSearch;
use super::tv::TvResult;
use super::tv::TvSearch;

#[pyclass]
#[derive(Debug, Serialize, Clone)]
pub struct Collection{
    #[pyo3(get)]
    pub user: String,
    #[pyo3(get)]
    pub id: u64,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub description: String,
    #[pyo3(get)]
    pub creator: String,
    #[pyo3(get)]
    pub creation_date: String,
    #[pyo3(get)]
    pub poster_path: String,
    #[pyo3(get)]
    pub movie: Vec<MovieResult>,
    #[pyo3(get)]
    pub tv: Vec<TvResult>, 
}

#[pymethods]
impl Collection{

    pub fn set_movie(&mut self) -> PyResult<()>{
        self.movie = MovieSearch::new(&self.user).collection(self.id)?.results()?;
        Ok(())
    }

    pub fn set_tv(&mut self) -> PyResult<()>{
        self.tv = TvSearch::new(&self.user).collection(self.id)?.results()?;
        Ok(())
    }

    pub fn edit_description(&mut self, description: String){
        self.description = description;
    }

    pub fn edit_poster_path(&mut self, poster_path: String){
        self.poster_path = poster_path;
    }

    pub fn save(&self)  -> PyResult<Collection>{
        Ok(DATABASE.update_collection(&self.user, &self)?)
    }

    pub fn delete(&self) -> PyResult<()>{
        DATABASE.delete_collection(self.id)?;
        Ok(())
    }


    pub fn json(&self) -> PyResult<String>{
        return Ok(serde_json::to_string(self).unwrap())
    }
}

#[pyproto]
impl PyObjectProtocol for Collection {
    fn __str__(&self) -> PyResult<String>{
        Ok(format!("{:?}", self))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

#[pyclass]
#[derive(Debug, Serialize, Clone)]
pub struct CollectionResult{
    #[pyo3(get)]
    pub user: String,
    #[pyo3(get)]
    pub id: u64,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub creator: String,
    #[pyo3(get)]
    pub creation_date: String,
    #[pyo3(get)]
    pub poster_path: String,
}

#[pymethods]
impl CollectionResult{
    pub fn full(&self) -> PyResult<Collection>{
        Ok(DATABASE.get_collection(&self.user, self.id)?.unwrap())
    }
}

#[pyproto]
impl PyObjectProtocol for CollectionResult {
    fn __str__(&self) -> PyResult<String>{
        Ok(format!("{:?}", self))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct CollectionSearch{
   parameters: HashMap<String, Option<(String, String)>>,
   user: String,
}

impl CollectionSearch{
    pub fn new(user: &String) -> CollectionSearch{
        CollectionSearch{
            parameters: HashMap::new(),
            user: user.clone(),
        }
    }
}

#[pymethods]
impl CollectionSearch{
    pub fn movie(&mut self, movie_id: u64) -> PyResult<CollectionSearch>{
        self.find("MovieCollectionLinks.movie_id", "=", Some(movie_id.to_string()))
    }

    pub fn tv(&mut self, tv_id: u64) -> PyResult<CollectionSearch>{
        self.find("TvCollectionLinks.tv_id", "=", Some(tv_id.to_string()))
    }

    pub fn find(&mut self, column: &str, operator: &str, value: Option<String>) -> PyResult<CollectionSearch>{
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

    pub fn results(&self) -> PyResult<Vec<CollectionResult>>{
        Ok(DATABASE.get_collections(&self.user, &self.parameters)?)
    }

    pub fn json_results(&self) -> PyResult<String>{
        let list = self.results()?;
        Ok(serde_json::to_string(&list).unwrap())
    }
}

#[pyproto]
impl PyObjectProtocol for CollectionSearch {
    fn __str__(&self) -> PyResult<String>{
        Ok(format!("{:?}", self))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}
