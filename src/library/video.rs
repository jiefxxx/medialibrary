use pyo3::PyObjectProtocol;
use pyo3::types::PyList;
use pyo3::{prelude::*, types::PyTuple};

#[pyclass]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Video{
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
    pub info: MediaInfo,
}


impl Video{
    pub fn new(path: String, media_type: u8) -> Video{
        Video{
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
            info: MediaInfo::Unknown,
        }
    }

    pub fn from_path(path: String, media_type: u8) -> PyResult<Video>{
        Python::with_gil(|py| {
            let media_info = PyModule::import(py, "pymediainfo")?.getattr("MediaInfo")?;
            let args = PyTuple::new(py, &[&path]);
            let tracks: &PyList = media_info.getattr("parse")?.call1(args)?.getattr("tracks")?.extract()?;
            let mut video = Video::new(path, media_type);
            for track in tracks{
                let track_type: String = track.getattr("track_type")?.extract()?;
                match track_type.as_ref(){
                    "General" => {
                        video.bit_rate = track.getattr("overall_bit_rate")?.extract()?;
                        video.duration = track.getattr("duration")?.extract()?;
                        video.size = track.getattr("file_size")?.extract()?;
                    },
                    "Video" => {
                        video.codec = track.getattr("codec_id")?.extract().unwrap();
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
                                video.audios.push(extracted);
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
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct MovieMinimal{
    #[pyo3(get)]
    pub id: u64,
    #[pyo3(get)]
    pub title: String,
    #[pyo3(get)]
    pub release_date: String,
}

#[pyclass]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
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


#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum MediaInfo{
    Tv(EpisodeMinimal),
    Movie(MovieMinimal),
    Unknown,
}

#[pyclass]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct VideoResult{
    #[pyo3(get)]
    pub id: u64,
    #[pyo3(get)]
    pub path: String,
    #[pyo3(get)]
    pub media_type: u8,
    #[pyo3(get)]
    pub adding: String,
    pub info: MediaInfo,
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