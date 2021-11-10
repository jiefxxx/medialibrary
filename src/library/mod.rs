
use std::collections::HashMap;

use crate::database::SqlLibrary;
use crate::rustmdb::Tmdb;

use pyo3::types::{PyFloat, PyLong, PyTuple, PyUnicode};
use pyo3::{prelude::*, types::PyDict};
use pyo3::exceptions::PyReferenceError;

mod update_db;
pub mod video;
mod tmdb;

use video::{Video, VideoResult};

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

#[pyclass]
pub struct Library{
    conn: SqlLibrary,
    tmdb: Tmdb,
    rsc_path: String,
}

#[pymethods]
impl Library {
    #[new]
    pub fn new(database_path: &str, api_key: &str, language: &str, rsc_path: String) -> Self {
        let api_key = string_to_static_str(api_key.to_string());
        let language = string_to_static_str(language.to_string());
        let tmdb = Tmdb::new(api_key, language);
        let conn = SqlLibrary::new(database_path).unwrap();
        conn.init_db().unwrap();
        Library{ conn, tmdb, rsc_path}
    }

    pub fn create_video(&self, path: String, media_type: u8) -> PyResult<u64> {
        let video = Video::from_path(path, media_type)?;
        let video_id = self.conn.create_video(video).unwrap();
        Ok(video_id)
    }

    #[args(kwargs = "**")]
    pub fn get_videos(&self, kwargs: Option<&PyDict>) -> PyResult<Vec<VideoResult>>{
        let mut map = HashMap::new();
        if let Some(kwargs) = kwargs{
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
        }
        match self.conn.get_videos(map){
            Ok(video) => Ok(video),
            Err(e) => Err(PyReferenceError::new_err(format!("database error get video media type {}", e))),
        }
    }

    pub fn edit_movie(&mut self, video_id: u64, movie_id: u64) -> PyResult<()>{
        match self.conn.get_video_media_type(video_id){
            Ok(Some(0)) => (),
            Err(e) => return Err(PyReferenceError::new_err(format!("database error get video media type {}", e))),
            Ok(Some(media_type)) => return Err(PyReferenceError::new_err(format!("edit movie error media type not 0 {}", media_type))),
            Ok(None) => return Err(PyReferenceError::new_err(format!("media_type undefined ")))
        };
        self.update_db_movie(movie_id)?;

        if let Err(e) = self.conn.edit_video_media_id(video_id, movie_id){
            return Err(PyReferenceError::new_err(format!("database error edit video media id {}", e)))
        }
        Ok(())
    }

    pub fn edit_tv(&mut self, video_id: u64, tv_id: u64, season: u64, episode: u64) -> PyResult<()>{
        match self.conn.get_video_media_type(video_id){
            Ok(Some(1)) => (),
            Err(e) => return Err(PyReferenceError::new_err(format!("database error get video media type {}", e))),
            Ok(Some(media_type)) => return Err(PyReferenceError::new_err(format!("edit movie error media type not 1 {}", media_type))),
            Ok(None) => return Err(PyReferenceError::new_err(format!("media_type undefined ")))
        };

        let episode_id = self.update_db_episode(tv_id, season, episode)?;

        if let Err(e) = self.conn.edit_video_media_id(video_id, episode_id){
            return Err(PyReferenceError::new_err(format!("database error edit video media id {}", e)))
        }

        Ok(())
    }

    //tmdb part

    pub fn find_movie(&self, title: &str, year: u64) -> PyResult<Option<u64>>{
        todo!();
    }

    pub fn find_tv(&self, title: &str) -> PyResult<Option<u64>>{
        todo!();
    }

    pub fn find_movie_id(&self,  title: &str, year: u64) -> PyResult<Option<u64>>{
       Ok(self.search_movie_id(title, year))
    }

    pub fn find_tv_id(&self,  title: &str) -> PyResult<Option<u64>>{
        Ok(self.search_tv_id(title))
     }


}