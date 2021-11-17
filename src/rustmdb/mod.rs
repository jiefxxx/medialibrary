use std::fmt;

use crate::rustmdb::model::ErrorModel;

use self::{model::{Movie, Person, Tv, TvEpisode}, movie::MovieSearch, tv::TvSearch};

pub mod model;
pub mod tv;
pub mod movie;

pub struct Tmdb{
    api_key: &'static str,
    language: &'static str,
}

impl Tmdb{
    pub fn new(api_key: &'static str, language: &'static str) -> Tmdb{
        Tmdb{api_key, language}
    }

    pub fn search_movie<'a>(&self, title: &'a str) -> MovieSearch<'a>{
        MovieSearch::new(self.api_key, &title, self.language)
    }

    pub fn search_tv<'a>(&self, title:&'a str) -> TvSearch<'a>{
        TvSearch::new(self.api_key, title, self.language)
    }

    pub fn movie(&self, id: u64) -> Result<Movie, Error>{
        let parameters = format!("api_key={}&language={}&append_to_response=credits", self.api_key, self.language);
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
        let parameters = format!("api_key={}&language={}&append_to_response=credits", self.api_key, self.language);
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
        let parameters = format!("api_key={}&language={}&append_to_response=credits", self.api_key, self.language);
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
        let parameters = format!("api_key={}&language={}", self.api_key, self.language);
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
