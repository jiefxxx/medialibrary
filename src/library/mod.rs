use std::fmt;
use std::sync::{Arc, Mutex};

use crate::database::DATABASE;

use pyo3::prelude::*;


mod update_db;
pub mod video;
pub mod movie;
pub mod tv;
pub mod cast;
pub mod keyword;
pub mod trailer;

use video::Video;

use self::movie::{Movie, MovieSearch};
use self::tv::{Tv, Season, Episode, TvSearch};
use self::video::VideoSearch;

lazy_static! {
    pub static ref RSCPATH: Arc<Mutex<String>> = Arc::new(Mutex::new("".to_string()));
}

#[pyclass]
pub struct Library{
}

#[pymethods]
impl Library {
    #[new]
    pub fn new(database_path: &str, rsc_path: String) -> Self {
        *RSCPATH.lock().unwrap() = rsc_path.to_string();
        DATABASE.connect(database_path);
        Library{ }
    }

    pub fn new_video(&self,user: String,  path: String, media_type: u8) -> PyResult<Video> {
        Ok(self.video(user.clone(), DATABASE.create_video(Video::from_path(user, path, media_type)?)?)?.unwrap())
    }

    pub fn videos(&self, user: String) -> VideoSearch{
        VideoSearch::new(user)
    }
    
    pub fn video(&self, user: String, video_id: u64) -> PyResult<Option<Video>>{
        Ok(DATABASE.get_video(&user, video_id)?)
    }

    pub fn movies(&self, user: String) -> MovieSearch{
        MovieSearch::new(&user)
    }

    pub fn movie(&self, user:String, movie_id: u64) -> PyResult<Option<Movie>>{
        Ok(DATABASE.get_movie(&user, movie_id)?)
    }

    pub fn tvs(&self, user: String) -> TvSearch{
        TvSearch::new(&user)
    }

    pub fn tv(&self, user: String, tv_id: u64) -> PyResult<Option<Tv>>{
        Ok(DATABASE.get_tv(&user, tv_id)?)
    }

    pub fn tv_season(&self, user: String, tv_id: u64, season_number: u64) -> PyResult<Option<Season>>{
        Ok(DATABASE.get_season(&user, tv_id, season_number)?)
    }

    pub fn tv_episode(&self, user:String, tv_id: u64, season_number: u64, episode_number: u64) -> PyResult<Option<Episode>>{
        Ok(DATABASE.get_episode(&user, tv_id, season_number, episode_number)?)
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