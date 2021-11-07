#[macro_use]
extern crate serde_derive;

mod database;
mod rustmdb;

use pyo3::prelude::*;
use pyo3::exceptions::PyReferenceError;

use database::SqlLibrary;

use rustmdb::Tmdb;

use std::io;
use std::fs::File;

use crate::rustmdb::{Movie, Tv};

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}


#[pyclass]
struct Library{
    conn: SqlLibrary,
    tmdb: Tmdb,
    rsc_path: String,
}

#[pymethods]
impl Library {
    #[new]
    fn new(database_path: &str, api_key: &str, language: &str, rsc_path: String) -> Self {
        let api_key = string_to_static_str(api_key.to_string());
        let language = string_to_static_str(language.to_string());
        let tmdb = Tmdb::new(api_key, language);
        let conn = SqlLibrary::new(database_path).unwrap();
        conn.init_db().unwrap();
        Library{ conn, tmdb, rsc_path}
    }

    pub fn create_video(&self, path: &str, media_type: u8, duration: f32, bit_rate: f32, 
        codec: &str, width: u32, height: u32, size: usize,
        subtitles: Vec<&str>, audios: Vec<&str>) -> PyResult<u64> {

        let video_id = self.conn.create_video(path, 
            media_type, 
            duration, 
            bit_rate, 
            codec, 
            width, 
            height, 
            size).unwrap();
        
        for language in subtitles{
            self.conn.create_video_subtitle(video_id, language).unwrap();
        }

        for language in audios{
            self.conn.create_video_audio(video_id, language).unwrap();
        }

        Ok(video_id)
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

    pub fn find_movie(&self, title: &str, year: u64) -> PyResult<Option<u64>>{
        let movies  = self.tmdb.search_movie(title).year(year).request().unwrap();
        
        movies.results[0].id;
        Ok(None)
    }

    fn update_db_movie(&mut self, movie_id: u64) -> PyResult<()>{
        match self.conn.movie_exist(movie_id) {
            Ok(true) => return Ok(()),
            Err(e) => return Err(PyReferenceError::new_err(format!("database error movie exist {}", e))),
            _ => ()
        };

        let movie: Movie = match self.tmdb.movie(movie_id){
            Ok(movie) => movie,
            Err(e) => return Err(PyReferenceError::new_err(format!("tmdb error {} for MovieID {:?}", e, movie_id))),
        };

        match self.conn.create_movie(&movie){
            Ok((person_ids, rsc_paths)) => {
                for person_id in person_ids{
                    self.update_db_person(person_id)?;
                }
                for rsc_path in rsc_paths{
                    self.update_rsc(&rsc_path)?;
                }
            },
            Err(e) => return Err(PyReferenceError::new_err(format!("database error create movie {:?} error:{}", movie, e))),
        };

        Ok(())
    }

    fn update_db_person(&mut self, person_id: u64) -> PyResult<()>{
        match self.conn.tv_exist(person_id) {
            Ok(false) => return Ok(()),
            Err(e) => return Err(PyReferenceError::new_err(format!("database error person exist {}", e))),
            _ => ()
        };

        let person = match  self.tmdb.person(person_id) {
            Ok(person) => person,
            Err(e) => return Err(PyReferenceError::new_err(format!("tmdb error {} for PersonID {:?}", e, person_id))),
        };

        match self.conn.create_person(&person){
            Ok((person_ids, rsc_paths)) => {
                for person_id in person_ids{
                    self.update_db_person(person_id)?;
                }
                for rsc_path in rsc_paths{
                    self.update_rsc(&rsc_path)?;
                }
            },
            Err(e) => return Err(PyReferenceError::new_err(format!("database error create person {:?} error:{}", person, e))),
        };

        Ok(())
    }

    fn update_db_tv(&mut self, tv_id: u64) -> PyResult<()>{
        match self.conn.tv_exist(tv_id) {
            Ok(false) => return Ok(()),
            Err(e) => return Err(PyReferenceError::new_err(format!("database error tv exist {}", e))),
            _ => ()
        };

        let tv: Tv = match self.tmdb.tv(tv_id){
            Ok(tv) => tv,
            Err(e) => return Err(PyReferenceError::new_err(format!("tmdb error {} for MovieID {:?}", e, tv_id))),
        };

    
        match self.conn.create_tv(&tv){
            Ok((person_ids, rsc_paths)) => {
                for person_id in person_ids{
                    self.update_db_person(person_id)?;
                }
                for rsc_path in rsc_paths{
                    self.update_rsc(&rsc_path)?;
                }
            },
            Err(e) => return Err(PyReferenceError::new_err(format!("database error create tv {:?} error:{}", tv, e))),
        };

        Ok(())

    }

    fn update_db_episode(&mut self, tv_id: u64, season_number: u64, episode_number: u64) -> PyResult<u64>{
        
        self.update_db_tv(tv_id)?;

        match self.conn.episode_exist(tv_id, season_number, episode_number) {
            Ok(Some(episode_id)) => return Ok(episode_id),
            Err(e) => return Err(PyReferenceError::new_err(format!("database error episode exist {}", e))),
            _ => ()
        };

        let episode = match self.tmdb.tv_episode(tv_id, season_number, episode_number){
            Ok(episode) => episode,
            Err(e) => return Err(PyReferenceError::new_err(format!("tmdb error {} for tv_id {:?} season {:?} episode {:?}", e, tv_id, season_number, episode_number))),
        };

        match self.conn.create_episode(tv_id, &episode){
            Ok((person_ids, rsc_paths)) => {
                for person_id in person_ids{
                    self.update_db_person(person_id)?;
                }
                for rsc_path in rsc_paths{
                    self.update_rsc(&rsc_path)?;
                }
            },
            Err(e) => return Err(PyReferenceError::new_err(format!("database error create episode {}", e))),
        };

        Ok(episode.id)
    }

    fn update_rsc(&self, rsc_path: &str) -> PyResult<()>{
        if rsc_path.len() == 0{
            return Ok(())
        }

        let resp = match reqwest::blocking::get("https://image.tmdb.org/t/p/original".to_string() + rsc_path){
            Ok(resp) => resp.bytes().unwrap(),
            Err(e) => return Err(PyReferenceError::new_err(format!("reqwest error getting poster path {}", e))),
        };

        let mut out = match File::create(self.rsc_path.clone()+rsc_path){
            Ok(out) => out,
            Err(e) => return Err(PyReferenceError::new_err(format!("file create error {}", e))),
        };

        io::copy(&mut resp.as_ref(), &mut out).expect("failed to copy content");

        Ok(())
    }

    pub fn get_video_id(&self, path: &str) ->PyResult<u64>{
        Ok(self.conn.get_video_id(path).unwrap())
    }

    pub fn get_unknown(&self) -> PyResult<Vec<(u64, u8, String)>>{
        Ok(self.conn.get_video_unknown().unwrap())
    }

}

#[pyfunction]
fn say_hello(path: &str) {
    println!("Hello world! {:?}", path)
}

#[pymodule]
fn medialibrary(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(say_hello, module)?)?;
    module.add_class::<Library>()?;
    Ok(())
}
