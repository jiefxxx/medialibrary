
use std::collections::HashMap;
use std::fmt;

use crate::database::SqlLibrary;

use pyo3::types::{PyFloat, PyLong, PyTuple, PyUnicode};
use pyo3::{prelude::*, types::PyDict};
use pyo3::exceptions::PyReferenceError;

mod update_db;
pub mod video;

use regex::Regex;
use video::{Video, VideoResult};

fn create_sql_param(kwargs: Option<&PyDict>) -> PyResult<HashMap<&str, Option<(String, String)>>>{
    let mut map = HashMap::new();
    let kwargs = match kwargs {
        Some(kwargs) => kwargs,
        None => return Ok(map),
    };
    
    for key in kwargs.keys(){
        let item = kwargs.get_item(key).unwrap();
        let value;
        if item.is_none(){
            value = None;
        }
        else if item.is_instance::<PyTuple>()?{
            let tuple: &PyTuple = item.extract()?;
            let data = tuple.get_item(1)?;
            if data.is_instance::<PyUnicode>()?{
                value = Some((tuple.get_item(0)?.extract()?, data.extract()?));
            }
            else if data.is_instance::<PyFloat>()?{
                value = Some((tuple.get_item(0)?.extract()?, data.extract::<f64>()?.to_string()));
            }
            else if data.is_instance::<PyLong>()?{
                value = Some((tuple.get_item(0)?.extract()?, data.extract::<i64>()?.to_string()));
            }
            else{
                return Err(PyReferenceError::new_err(format!("args must be (string,[int, float or string])")));
            }   
        }
        else if item.is_instance::<PyUnicode>()?{
            value = Some(("=".to_string(), item.extract()?));
        }
        else if item.is_instance::<PyFloat>()?{
            value = Some(("=".to_string(), item.extract::<f64>()?.to_string()));
        }
        else if item.is_instance::<PyLong>()?{
            value = Some(("=".to_string(), item.extract::<i64>()?.to_string()));
        }
        else{
            return Err(PyReferenceError::new_err(format!("args must be (string,[int, float, string]), int, float, string or None")));
        }
        map.insert(key.extract()?, value);
    }

    Ok(map)
    
}

#[pyclass]
pub struct Library{
    conn: SqlLibrary,
    rsc_path: String,
}

#[pymethods]
impl Library {
    #[new]
    pub fn new(database_path: &str, rsc_path: String) -> Self {
        let conn = SqlLibrary::new(database_path).unwrap();
        conn.init_db().unwrap();
        Library{ conn, rsc_path}
    }

    pub fn create_video(&self, path: String, media_type: u8) -> PyResult<u64> {
        let video = Video::from_path(path, media_type)?;
        let video_id = self.conn.create_video(video)?;
        Ok(video_id)
    }

    #[args(kwargs = "**")]
    pub fn get_videos(&self, kwargs: Option<&PyDict>) -> PyResult<Vec<VideoResult>>{
        Ok(self.conn.get_videos(create_sql_param(kwargs)?)?)
    }
    
    pub fn get_video(&self, video_id: u64) -> PyResult<Option<Video>>{
        Ok(self.conn.get_video(video_id)?)
    }

    pub fn edit_movie(&mut self, video_id: u64, movie_id: u64) -> PyResult<()>{
        match self.conn.get_video_media_type(video_id)?{
            Some(0) => (),
            Some(media_type) => return Err(Error::new(ErrorKind::MediaType,"mediatype error".to_string(),&format!("media type not movie {}", media_type)).into()),
            None => return Err(Error::new(ErrorKind::MediaType, "mediatype error".to_string(),&format!("undefined ")).into())
        };
        self.create_movie(movie_id)?;

        self.conn.edit_video_media_id(video_id, movie_id)?;
        Ok(())
    }

    pub fn edit_tv(&mut self, video_id: u64, tv_id: u64, season: u64, episode: u64) -> PyResult<()>{
        match self.conn.get_video_media_type(video_id)?{
            Some(1) => (),
            Some(media_type) => return Err(Error::new(ErrorKind::MediaType,"mediatype error".to_string(),&format!("media type not tv {}", media_type)).into()),
            None => return Err(Error::new(ErrorKind::MediaType, "mediatype error".to_string(),&format!("undefined ")).into())
        };

        let episode_id = self.create_episode(tv_id, season, episode)?;

        self.conn.edit_video_media_id(video_id, episode_id)?;

        Ok(())
    }

    #[staticmethod]
    pub fn parse_tv(path: &str) -> PyResult<(String, u64, u64)>{
        let re = Regex::new(r".*[/](.*)[.][sS](\d+)[eE](\d+)[.]?.*[.](.*)").unwrap();
        for cap in re.captures_iter(path) {
            return Ok((cap[1].to_string().replace(".", " "), cap[2].parse::<u64>()?, cap[3].parse::<u64>()?))
        }
        return Err(Error::new(ErrorKind::ParseName, "could not parse name".to_string(), &format!("tv path: {}", path)).into())
    }

    #[staticmethod]
    pub fn parse_movie(path: &str) -> PyResult<(String, u64)>{
        let re = Regex::new(r".*[/](.*)[.](\d{4})[.]?.*[.](.*)").unwrap();
        for cap in re.captures_iter(path) {
            return Ok((cap[1].to_string().replace(".", " "), cap[2].parse::<u64>()?))
        }
        return Err(Error::new(ErrorKind::ParseName, "could not parse name".to_string(), &format!("movie path: {}", path)).into())
    }

}

#[derive(Debug)]
pub enum ErrorKind{
    ParseName,
    MediaType
}

#[derive(Debug)]
pub struct Error{
    kind: ErrorKind,
    description: String,
    location: String,
}

impl Error{
    pub fn new(kind: ErrorKind, description: String, location: &str) -> Error{
        Error{
            kind,
            description,
            location: location.to_string(), 
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {:?} at {} {}", &self.kind, &self.location, &self.description)
    }
}