use std::io;
use std::fs::File;

use pyo3::prelude::*;
use pyo3::exceptions::PyReferenceError;

use super::Library;


impl Library {
    pub fn update_db_movie(&mut self, movie_id: u64) -> PyResult<()>{
        match self.conn.movie_exist(movie_id) {
            Ok(true) => return Ok(()),
            Err(e) => return Err(PyReferenceError::new_err(format!("database error movie exist {}", e))),
            _ => ()
        };

        let movie = self.tmdb.movie(movie_id)?;

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

    pub fn update_db_person(&mut self, person_id: u64) -> PyResult<()>{
        match self.conn.person_exist(person_id) {
            Ok(true) => return Ok(()),
            Err(e) => return Err(PyReferenceError::new_err(format!("database error person exist {}", e))),
            _ => ()
        };

        let person = self.tmdb.person(person_id)?;
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

    pub fn update_db_tv(&mut self, tv_id: u64) -> PyResult<()>{
        match self.conn.tv_exist(tv_id) {
            Ok(true) => return Ok(()),
            Err(e) => return Err(PyReferenceError::new_err(format!("database error tv exist {}", e))),
            _ => ()
        };

        let tv = self.tmdb.tv(tv_id)?;
    
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

    pub fn update_db_episode(&mut self, tv_id: u64, season_number: u64, episode_number: u64) -> PyResult<u64>{
        
        self.update_db_tv(tv_id)?;

        match self.conn.episode_exist(tv_id, season_number, episode_number) {
            Ok(Some(episode_id)) => return Ok(episode_id),
            Err(e) => return Err(PyReferenceError::new_err(format!("database error episode exist {}", e))),
            _ => ()
        };

        let episode = self.tmdb.tv_episode(tv_id, season_number, episode_number)?;

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

    pub fn update_rsc(&self, rsc_path: &str) -> PyResult<()>{
        if rsc_path.len() == 0{
            return Ok(())
        }

        let resp = match reqwest::blocking::get("https://image.tmdb.org/t/p/original".to_string() + rsc_path){
            Ok(resp) => resp.bytes().unwrap(),
            Err(e) => return Err(PyReferenceError::new_err(format!("reqwest error getting poster path {}", e))),
        };

        let mut out = File::create(self.rsc_path.clone()+rsc_path)?;

        io::copy(&mut resp.as_ref(), &mut out)?;

        Ok(())
    }
}