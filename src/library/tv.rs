use std::collections::HashMap;

use pyo3::PyObjectProtocol;
use pyo3::prelude::*;

use crate::database::DATABASE;

use super::cast::Cast;
use super::cast::Crew;
use super::keyword::Keyword;
use super::trailer::Trailer;
use super::video::VideoResult;
use super::video::VideoSearch;

#[pyclass]
#[derive(Debug, Serialize, Clone)]
pub struct Tv{
    pub user: String,
    #[pyo3(get)]
    pub id: u64,
    #[pyo3(get)]
    pub original_title: String,
    #[pyo3(get)]
    pub original_language: String,
    #[pyo3(get)]
    pub title: String,
    #[pyo3(get)]
    pub release_date: String,
    #[pyo3(get)]
    pub overview: String,
    #[pyo3(get)]
    pub popularity: f64,
    #[pyo3(get)]
    pub poster_path: String,
    #[pyo3(get)]
    pub backdrop_path: String,
    #[pyo3(get)]
    pub vote_average: f64,
    #[pyo3(get)]
    pub vote_count: i64,
    #[pyo3(get)]
    pub status: String,
    #[pyo3(get)]
    pub genres: Vec<String>,
    #[pyo3(get)]
    pub number_of_episodes: u64,
    #[pyo3(get)]
    pub number_of_seasons: u64,
    #[pyo3(get)]
    pub episode_run_time: u64,
    #[pyo3(get)]
    pub adding: String,
    #[pyo3(get)]
    pub seasons: Vec<Season>,
    #[pyo3(get)]
    pub cast: Vec<Cast>,
    #[pyo3(get)]
    pub crew: Vec<Crew>,
    #[pyo3(get)]
    pub trailer: Vec<Trailer>,
    #[pyo3(get)]
    pub keyword: Vec<Keyword>,
    #[pyo3(get)]
    pub watched: u64,
    #[pyo3(get)]
    pub updated: String,
}

#[pymethods]
impl Tv{

    pub fn set_seasons(&mut self) -> PyResult<()>{
        self.seasons = DATABASE.get_seasons(&self.user, self.id)?;
        Ok(())
    }

    pub fn set_persons(&mut self) -> PyResult<()>{
        self.cast = DATABASE.get_tv_cast(self.id)?;
        self.crew = DATABASE.get_tv_crew(self.id)?;
        Ok(())
    }

    pub fn set_trailers(&mut self) -> PyResult<()>{
        self.trailer = DATABASE.get_tv_trailer(self.id)?;
        Ok(())
    }

    pub fn set_keywords(&mut self) -> PyResult<()>{
        self.keyword = DATABASE.get_tv_keywords(self.id)?;
        Ok(())
    }

    pub fn season(&self, season_number: u64) -> PyResult<Option<Season>>{
        Ok(DATABASE.get_season(&self.user, self.id, season_number)?)
    }

    pub fn episode(&self, season_number: u64, episode_number: u64) -> PyResult<Option<Episode>>{
        Ok(DATABASE.get_episode(&self.user, self.id, season_number, episode_number)?)
    }

    /* pub fn set_watched(&self, user: String, watched: bool) -> PyResult<()>{
        Ok(DATABASE.set_tv_watched(user, self.id, watched)?)
    }*/

    pub fn json(&self) -> PyResult<String>{
        return Ok(serde_json::to_string(self).unwrap())
    }

}

#[pyproto]
impl PyObjectProtocol for Tv {
    fn __str__(&self) -> PyResult<String>{
        Ok(format!("{:?}", self))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

#[pyclass]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct TvResult{
    pub user: String,
    #[pyo3(get)]
    pub id: u64,
    #[pyo3(get)]
    pub title: String,
    #[pyo3(get)]
    pub release_date: String,
    #[pyo3(get)]
    pub poster_path: String,
    #[pyo3(get)]
    pub vote_average: f64,
    #[pyo3(get)]
    pub genres: Vec<String>,
    #[pyo3(get)]
    pub adding: String,
    #[pyo3(get)]
    pub watched: u64,
}


#[pyproto]
impl PyObjectProtocol for TvResult {
    fn __str__(&self) -> PyResult<String>{
        Ok(format!("{:?}", self))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct TvSearch{
   parameters: HashMap<String, Option<(String, String)>>,
   user: String,
}

impl TvSearch{
    pub fn new(user: &String) -> TvSearch{
        TvSearch{
            parameters: HashMap::new(),
            user: user.clone(),
        }
    }
}

#[pymethods]
impl TvSearch{
    pub fn id(&mut self, id: u64) -> PyResult<TvSearch>{
        self.find("TvsView.id", "=", Some(id.to_string()))
    }

    pub fn find(&mut self, column: &str, operator: &str, value: Option<String>) -> PyResult<TvSearch>{
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

    pub fn results(&self) -> PyResult<Vec<TvResult>>{
        Ok(DATABASE.get_tvs(&self.user, &self.parameters)?)
    }

    pub fn json_results(&self) -> PyResult<String>{
        let list = self.results()?;
        Ok(serde_json::to_string(&list).unwrap())
    }
}

#[pyproto]
impl PyObjectProtocol for TvSearch {
    fn __str__(&self) -> PyResult<String>{
        Ok(format!("{:?}", self))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

#[pyclass]
#[derive(Debug, Serialize, Clone)]
pub struct Season{
    pub user: String,
    #[pyo3(get)]
    pub tv_id: u64,
    #[pyo3(get)]
    pub season_number: u64,
    #[pyo3(get)]
    pub episode_count: u64,
    #[pyo3(get)]
    pub title: String,
    #[pyo3(get)]
    pub overview: String,
    #[pyo3(get)]
    pub poster_path: String,
    #[pyo3(get)]
    pub release_date: String,
    #[pyo3(get)]
    pub episodes: Vec<Episode>,
    #[pyo3(get)]
    pub tv: Option<Tv>,
    #[pyo3(get)]
    pub watched: u64,
    #[pyo3(get)]
    pub updated: String,
}

#[pymethods]
impl Season{

    pub fn set_tv(&mut self) -> PyResult<()>{
        self.tv = DATABASE.get_tv(&self.user, self.tv_id)?;
        Ok(())
    }

    pub fn episode(&mut self, episode_number: u64) -> PyResult<Option<Episode>>{
        Ok(DATABASE.get_episode(&self.user, self.tv_id, self.season_number, episode_number)?)
    }

    pub fn set_episodes(&mut self) -> PyResult<()>{
        self.episodes = DATABASE.get_episodes(&self.user, self.tv_id, self.season_number)?;
        Ok(())
    }

    pub fn set_episode_videos(&mut self) -> PyResult<()>{
        for episode in &mut self.episodes{
            episode.set_videos()?;
        }
        Ok(())
    }

    pub fn json(&self) -> PyResult<String>{
        return Ok(serde_json::to_string(self).unwrap())
    }

}


#[pyproto]
impl PyObjectProtocol for Season {
    fn __str__(&self) -> PyResult<String>{
        Ok(format!("{:?}", self))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}


#[pyclass]
#[derive(Debug, Serialize, Clone)]
pub struct Episode{
    pub user: String,
    #[pyo3(get)]
    pub tv_id: u64,
    #[pyo3(get)]
    pub id: u64,
    #[pyo3(get)]
    pub season_number: u64,
    #[pyo3(get)]
    pub episode_number: u64,
    #[pyo3(get)]
    pub release_date: String,
    #[pyo3(get)]
    pub title: String,
    #[pyo3(get)]
    pub overview: String,
    #[pyo3(get)]
    pub vote_average: f64,
    #[pyo3(get)]
    pub vote_count: u64,
    #[pyo3(get)]
    pub video: Vec<VideoResult>,
    #[pyo3(get)]
    pub tv: Option<Tv>,
    #[pyo3(get)]
    pub season: Option<Season>,
    #[pyo3(get)]
    pub watched: u64,
    #[pyo3(get)]
    pub updated: String,
}

#[pymethods]
impl Episode{

    pub fn set_tv(&mut self) -> PyResult<()>{
        self.tv = DATABASE.get_tv(&self.user, self.tv_id)?;
        Ok(())
    }

    pub fn set_season(&mut self) -> PyResult<()>{
        self.season = DATABASE.get_season(&self.user, self.tv_id, self.season_number)?;
        Ok(())
    }

    pub fn set_videos(&mut self) -> PyResult<()>{
        self.video = VideoSearch::new(self.user.clone()).tv()?.media_id(self.id)?.results()?;
        Ok(())
    }

    pub fn json(&self) -> PyResult<String>{
        return Ok(serde_json::to_string(self).unwrap())
    }

}



#[pyproto]
impl PyObjectProtocol for Episode {
    fn __str__(&self) -> PyResult<String>{
        Ok(format!("{:?}", self))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}