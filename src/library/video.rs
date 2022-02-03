use std::collections::HashMap;

use pyo3::PyObjectProtocol;
use pyo3::types::PyList;
use pyo3::{prelude::*, types::PyTuple};
use regex::Regex;

use crate::database::DATABASE;

use super::movie::Movie;
use super::tv::Episode;
use super::update_db::{create_movie, create_episode};
use super::{Error, ErrorKind};

#[pyclass]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Video{
    pub user: String,
    #[pyo3(get)]
    pub id: u64,
    #[pyo3(get)]
    pub path: String,
    #[pyo3(get)]
    pub media_type: u8,
    #[pyo3(get)]
    pub media_id: Option<u64>,
    #[pyo3(get)]
    pub bit_rate: u64,
    #[pyo3(get)]
    pub duration: u64,
    #[pyo3(get)]
    pub size: u64,
    #[pyo3(get)]
    pub adding: String,
    #[pyo3(get)]
    pub codec: Option<String>,
    #[pyo3(get)]
    pub width: u64,
    #[pyo3(get)]
    pub height: u64,
    #[pyo3(get)]
    pub subtitles: Vec<String>,
    #[pyo3(get)]
    pub audios: Vec<String>,
}

#[pymethods]
impl Video{

    pub fn parse_tv(&self) -> PyResult<(String, u64, u64)>{
        let re = Regex::new(r".*[/](.*)[.][sS](\d+)[eE](\d+)[.]?.*[.](.*)").unwrap();
        for cap in re.captures_iter(&self.path) {
            return Ok((cap[1].to_string().replace(".", " "), cap[2].parse::<u64>()?, cap[3].parse::<u64>()?))
        }
        return Err(Error::new(ErrorKind::ParseName, "could not parse name".to_string(), &format!("tv path: {}", self.path)).into())
    }

    pub fn parse_movie(&self) -> PyResult<(String, u64)>{
        let re = Regex::new(r".*[/](.*)[.](\d{4})[.]?.*[.](.*)").unwrap();
        for cap in re.captures_iter(&self.path) {
            return Ok((cap[1].to_string().replace(".", " "), cap[2].parse::<u64>()?))
        }
        return Err(Error::new(ErrorKind::ParseName, "could not parse name".to_string(), &format!("movie path: {}", self.path)).into())
    }

    pub fn set_movie(&mut self, movie_id: u64) -> PyResult<()>{
        if self.media_type != 0{
            return Err(Error::new(ErrorKind::MediaType,"mediatype error".to_string(),&format!("media type not movie {}", self.media_type)).into())
        }

        create_movie(&self.user, movie_id)?;

        DATABASE.edit_video_media_id(self.id, movie_id)?;
        Ok(())
    }

    pub fn set_tv(&mut self, tv_id: u64, season: u64, episode: u64) -> PyResult<()>{
        if self.media_type != 1{
            return Err(Error::new(ErrorKind::MediaType,"mediatype error".to_string(),&format!("media type not episode {}", self.media_type)).into())
        }

        let episode_id = create_episode(&self.user, tv_id, season, episode)?;

        DATABASE.edit_video_media_id(self.id, episode_id)?;

        Ok(())
    }

    pub fn movie(&self) -> PyResult<Option<Movie>>{
        if self.media_type != 0{
            return Err(Error::new(ErrorKind::MediaType,"mediatype error".to_string(),&format!("media type not movie {}", self.media_type)).into())
        }
        if let Some(media_id) = self.media_id{
            Ok(DATABASE.get_movie(&self.user, media_id)?)
        }
        else{
            Ok(None)
        }
    }

    pub fn tv_episode(&self) -> PyResult<Option<Episode>>{
        if self.media_type != 1{
            return Err(Error::new(ErrorKind::MediaType,"mediatype error".to_string(),&format!("media type not episode {}", self.media_type)).into())
        }
        if let Some(media_id) = self.media_id{
            Ok(DATABASE.get_episode_by_id(&self.user, media_id)?)
        }
        else{
            Ok(None)
        }
    }

    pub fn json(&self) -> PyResult<String>{
        return Ok(serde_json::to_string(self).unwrap())
    }
}

impl Video{
    pub fn new(user: String, path: String, media_type: u8) -> Video{
        Video{
            user: user,
            id: 0,
            path,
            media_type,
            media_id: None,
            bit_rate: 0,
            duration: 0,
            size: 0,
            adding: String::new(),
            codec: None,
            width: 0,
            height: 0,
            subtitles: Vec::new(),
            audios: Vec::new(),
        }
    }

    pub fn from_path(user: String, path: String, media_type: u8) -> PyResult<Video>{
        Python::with_gil(|py| {
            let media_info = PyModule::import(py, "pymediainfo")?.getattr("MediaInfo")?;
            let args = PyTuple::new(py, &[&path]);
            let tracks: &PyList = media_info.getattr("parse")?.call1(args)?.getattr("tracks")?.extract()?;
            let mut video = Video::new(user, path, media_type);
            for track in tracks{
                let track_type: String = track.getattr("track_type")?.extract()?;
                match track_type.as_ref(){
                    "General" => {
                        video.bit_rate = track.getattr("overall_bit_rate")?.extract()?;
                        video.duration = track.getattr("duration")?.extract()?;
                        video.size = track.getattr("file_size")?.extract()?;
                    },
                    "Video" => {
                        video.codec = track.getattr("codec_id")?.extract()?;
                        video.width = track.getattr("width")?.extract()?;
                        video.height = track.getattr("height")?.extract()?;
                    },
                    "Audio" => {
                        if let Ok(language) = track.getattr("language"){
                            if let Ok(extracted) = language.extract(){
                                video.audios.push(extracted);
                            }
                        }
                    }
                    "Text" => {
                        if let Ok(language) = track.getattr("language"){
                            if let Ok(extracted) = language.extract(){
                                video.subtitles.push(extracted);
                            }
                        }
                    }
                    _ => ()
                }
            }
            Ok(video)
        })
    }
}


#[pyproto]
impl PyObjectProtocol for Video {
    fn __str__(&self) -> PyResult<String>   {
        Ok(format!("{:?}", self))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

#[pyclass]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct MovieMinimal{
    #[pyo3(get)]
    pub id: u64,
    #[pyo3(get)]
    pub title: String,
    #[pyo3(get)]
    pub release_date: String,
}

#[pyclass]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct EpisodeMinimal{
    #[pyo3(get)]
    pub id: u64,
    #[pyo3(get)]
    pub title: String,
    #[pyo3(get)]
    pub season_number: u64,
    #[pyo3(get)]
    pub episode_number: u64
}


#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum MediaInfo{
    Tv(EpisodeMinimal),
    Movie(MovieMinimal),
    Unknown,
}

#[pyclass]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct VideoResult{
    pub user: String,
    #[pyo3(get)]
    pub id: u64,
    #[pyo3(get)]
    pub path: String,
    #[pyo3(get)]
    pub media_type: u8,
    #[pyo3(get)]
    pub adding: String,
    #[pyo3(get)]
    pub duration: u64,
    #[pyo3(get)]
    pub codec: Option<String>,
    #[pyo3(get)]
    pub size: u64,
    #[pyo3(get)]
    pub subtitles: Vec<String>,
    #[pyo3(get)]
    pub audios: Vec<String>,
    pub info: MediaInfo,
}

#[pymethods]
impl VideoResult{
    pub fn full(&self) -> PyResult<Video>{
        Ok(DATABASE.get_video(&self.user, self.id)?.unwrap())
    }

}


#[pyproto]
impl PyObjectProtocol for VideoResult {
    fn __str__(&self) -> PyResult<String>{
        Ok(format!("{:?}", self))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct VideoSearch{
   parameters: HashMap<String, Option<(String, String)>>,
   user: String,
}

impl VideoSearch{
    pub fn new(user: String) -> VideoSearch{
        VideoSearch{
            parameters: HashMap::new(),
            user,
        }
    }
}

#[pymethods]
impl VideoSearch{
    pub fn path(&mut self, path: String) -> PyResult<VideoSearch>{
        self.find("path", "=", Some(path))
    }

    pub fn movie(&mut self) -> PyResult<VideoSearch>{
        self.find("media_type", "=", Some("0".to_string()))
    }

    pub fn tv(&mut self) -> PyResult<VideoSearch>{
        self.find("media_type", "=", Some("1".to_string()))
    }

    pub fn unknown(&mut self) -> PyResult<VideoSearch>{
        self.find("media_id", "is", None)
        
    }

    pub fn media_id(&mut self, id: u64)  -> PyResult<VideoSearch>{
        self.find("media_id", "=", Some(id.to_string()))
    }

    pub fn id(&mut self, id: u64) -> PyResult<VideoSearch>{
        self.find("id", "=", Some(id.to_string()))
    }

    pub fn find(&mut self, column: &str, operator: &str, value: Option<String>) -> PyResult<VideoSearch>{
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

    pub fn results(&self) -> PyResult<Vec<VideoResult>>{
        Ok(DATABASE.get_videos(&self.user, &self.parameters)?)
    }

    pub fn json_results(&self) -> PyResult<String>{
        let list = self.results()?;
        Ok(serde_json::to_string(&list).unwrap())
    }
}

#[pyproto]
impl PyObjectProtocol for VideoSearch {
    fn __str__(&self) -> PyResult<String>{
        Ok(format!("{:?}", self))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}