use std::fmt;
use std::sync::{Arc, Mutex};

use crate::rustmdb::model::ErrorModel;
use pyo3::prelude::*;

use self::{model::{Movie, Person, SearchMovie, SearchTv, Tv, TvEpisode}, movie::MovieSearch, tv::TvSearch};

pub mod model;
pub mod tv;
pub mod movie;

use strsim::jaro;


lazy_static! {
    pub static ref TMDBKEY: Arc<Mutex<String>> = Arc::new(Mutex::new("".to_string()));
    pub static ref LANGUAGE: Arc<Mutex<String>> = Arc::new(Mutex::new("fr".to_string()));
}

#[pyclass]
pub struct Tmdb{
}

#[pymethods]
impl Tmdb{
    #[new]
    pub fn new() -> Tmdb{
        Tmdb{}
    }

    pub fn search_movie_id(&self, title: &str, year: u64) -> PyResult<Option<u64>>{
        let movies  = self.search_movie(title).year(year).request()?;
        if movies.results.len() == 0{
            return Ok(None)
        }
        
        let mut score = 0.0;
        let mut best: &SearchMovie = &movies.results[0];
        for movie in &movies.results{
            let score_original_title = jaro(title, &movie.original_title);
            if score_original_title > score  || (score_original_title == score && movie.release_date[..4] == year.to_string()){
                score = score_original_title;
                best = movie;
            }

            let score_title = jaro(title, &movie.title);
            if score_title > score || (score_title == score && movie.release_date[..4] == year.to_string()){
                score = score_title;
                best = movie;
            }
        }

        Ok(Some(best.id))
    }

    pub fn search_tv_id(&self, title: &str) -> PyResult<Option<u64>>{
        let tvs  = self.search_tv(title).request()?;
        if tvs.results.len() == 0{
            return Ok(None)
        }
        let mut score = 0.0;
        let mut best: &SearchTv = &tvs.results[0];
        for tv in &tvs.results{
            let score_original_title = jaro(title, &tv.original_name);
            let score_title = jaro(title, &tv.name);
            if score_original_title > score{
                score = score_original_title;
                best = tv;
            }
            if score_title > score{
                score = score_title;
                best = tv;
            }
        }
        Ok(Some(best.id))
    }
}

impl Tmdb{
    pub fn search_movie<'a>(&self, title: &'a str) -> MovieSearch<'a>{
        MovieSearch::new(&title)
    }

    pub fn search_tv<'a>(&self, title:&'a str) -> TvSearch<'a>{
        TvSearch::new(title)
    }

    pub fn movie(&self, id: u64) -> Result<Movie, Error>{
        let parameters = format!("api_key={}&language={}&append_to_response=credits", *TMDBKEY.lock().unwrap(), *LANGUAGE.lock().unwrap());
        let body = match reqwest::blocking::get(format!("https://api.themoviedb.org/3/movie/{}?{}",id, parameters)){
            Ok(body) => body,
            Err(e) => return Err(Error::from_reqwest(e, &format!("tmdb.movie({})", &id)))
        };
        if body.status().is_success(){
            match body.json(){
                Ok(movie) => return Ok(movie),
                Err(e) => return Err(Error::from_reqwest(e, &format!("tmdb.movie({}) parse body", &id))),
            };
        }
        let e: ErrorModel = match body.json(){
            Ok(e) => e,
            Err(e) => return Err(Error::from_reqwest(e, &format!("tmdb.movie({}) parse error", id))),
        };
        Err(Error::new(ErrorKind::Tmdb, e.status_message, &format!("tmdb.movie({}) return error", id)))
    }

    pub fn tv(&self, id: u64) -> Result<Tv, Error>{
        let parameters = format!("api_key={}&language={}&append_to_response=credits", *TMDBKEY.lock().unwrap(), *LANGUAGE.lock().unwrap());
        let body = match reqwest::blocking::get(format!("https://api.themoviedb.org/3/tv/{}?{}",id, parameters)){
            Ok(body) => body,
            Err(e) => return Err(Error::from_reqwest(e, &format!("tmdb.tv({})", id)))
        };
        if body.status().is_success(){
            match body.json(){
                Ok(movie) => return Ok(movie),
                Err(e) => return Err(Error::from_reqwest(e, &format!("tmdb.tv({}) parse body", id))),
            };
        }
        let e: ErrorModel = match body.json(){
            Ok(e) => e,
            Err(e) => return Err(Error::from_reqwest(e, &format!("tmdb.tv({}) parse error", id))),
        };
        Err(Error::new(ErrorKind::Tmdb, e.status_message, &format!("tmdb.tv({}) return error", id)))
    }

    pub fn tv_episode(&self, id: u64, season: u64, episode: u64) -> Result<TvEpisode, Error>{
        let parameters = format!("api_key={}&language={}&append_to_response=credits", *TMDBKEY.lock().unwrap(), *LANGUAGE.lock().unwrap());
        let body = match reqwest::blocking::get(format!("https://api.themoviedb.org/3/tv/{}/season/{}/episode/{}?{}",id, season, episode, parameters)){
            Ok(body) => body,
            Err(e) => return Err(Error::from_reqwest(e, &format!("tmdb.episode({} s{}e{})", id, season, episode)))
        };
        if body.status().is_success(){
            match body.json(){
                Ok(movie) => return Ok(movie),
                Err(e) => return Err(Error::from_reqwest(e, &format!("tmdb.episode({} s{}e{}) parse body", id, season, episode))),
            };
        }
        let e: ErrorModel = match body.json(){
            Ok(e) => e,
            Err(e) => return Err(Error::from_reqwest(e, &format!("tmdb.episode({} s{}e{}) parse error", id, season, episode))),
        };
        Err(Error::new(ErrorKind::Tmdb, e.status_message, &format!("tmdb.episode({} s{}e{}) return error", id, season, episode)))

    }

    pub fn person(&self, id: u64) -> Result<Person, Error>{
        let parameters = format!("api_key={}&language={}", *TMDBKEY.lock().unwrap(), *LANGUAGE.lock().unwrap());
        let body = match reqwest::blocking::get(format!("https://api.themoviedb.org/3/person/{}?{}",id, parameters)){
            Ok(body) => body,
            Err(e) => return Err(Error::from_reqwest(e, &format!("tmdb.person({})", id)))
        };
        if body.status().is_success(){
            match body.json(){
                Ok(movie) => return Ok(movie),
                Err(e) => return Err(Error::from_reqwest(e, &format!("tmdb.person({}) parse body", id))),
            };
        }
        let e: ErrorModel = match body.json(){
            Ok(e) => e,
            Err(e) => return Err(Error::from_reqwest(e, &format!("tmdb.person({}) parse error", id))),
        };
        Err(Error::new(ErrorKind::Tmdb, e.status_message, &format!("tmdb.person({}) return error", id)))
    }
}

#[derive(Debug)]
pub enum ErrorKind{
    Timeout,
    Connection,
    Json,
    ReqwestError,
    Tmdb,
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

    pub fn from_reqwest(e: reqwest::Error, location: &str) -> Error{
        if e.is_timeout(){
            Error::new(ErrorKind::Timeout, e.to_string(), location)
        }
        else if e.is_connect() {
            Error::new(ErrorKind::Connection, e.to_string(), location)
        }
        else{
            Error::new(ErrorKind::ReqwestError, e.to_string(), location)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {:?} at {} {}", &self.kind, &self.location, &self.description)
    }
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::from_reqwest(err, "Undefined")
    }
}
