use std::io;
use std::fs::File;

use pyo3::prelude::*;
use pyo3::exceptions::PyReferenceError;

use crate::rustmdb::{get_movie, get_person, get_tv, get_tv_episode};

use super::Library;


impl Library {
    pub fn create_movie(&mut self, movie_id: u64) -> PyResult<()>{
        if self.conn.movie_exist(movie_id)?{
            return Ok(())
        }
        self.update_movie(movie_id)
    }

    pub fn update_movie(&mut self, movie_id: u64) -> PyResult<()>{
        let movie = get_movie(movie_id)?;
        let (person_ids, rsc_paths) = self.conn.create_movie(&movie)?;
        for person_id in person_ids{
            self.create_person(person_id)?;
        }
        for rsc_path in rsc_paths{
            self.update_rsc(&rsc_path)?;
        }

        Ok(())
    }

    pub fn create_person(&mut self, person_id: u64) -> PyResult<()>{
        if self.conn.person_exist(person_id)?{
            return Ok(())
        }
        self.update_person(person_id)
    }

    pub fn update_person(&mut self, person_id: u64) -> PyResult<()>{  
        let person = get_person(person_id)?;
        let (_person_ids, rsc_paths) = self.conn.create_person(&person)?;
        for rsc_path in rsc_paths{
            self.update_rsc(&rsc_path)?;
        }
        Ok(())
    }

    pub fn create_tv(&mut self, tv_id: u64) -> PyResult<()>{
        if self.conn.tv_exist(tv_id)?{
            return Ok(())
        }
        self.update_tv(tv_id)
    }

    pub fn update_tv(&mut self, tv_id: u64) -> PyResult<()>{
        let tv = get_tv(tv_id)?;
        let (person_ids, rsc_paths) = self.conn.create_tv(&tv)?;
        for person_id in person_ids{
            self.create_person(person_id)?;
        }
        for rsc_path in rsc_paths{
            self.update_rsc(&rsc_path)?;
        }
        Ok(())

    }

    pub fn create_episode(&mut self, tv_id: u64, season_number: u64, episode_number: u64) -> PyResult<u64>{
        if let Some(episode_id) = self.conn.episode_exist(tv_id, season_number, episode_number)?{
            return Ok(episode_id)
        }
        self.update_episode(tv_id, season_number, episode_number)
    }

    pub fn update_episode(&mut self, tv_id: u64, season_number: u64, episode_number: u64) -> PyResult<u64>{
        self.create_tv(tv_id)?;

        let episode = get_tv_episode(tv_id, season_number, episode_number)?;
        let (person_ids, rsc_paths) = self.conn.create_episode(tv_id, &episode)?;
        for person_id in person_ids{
            self.create_person(person_id)?;
        }
        for rsc_path in rsc_paths{
            self.update_rsc(&rsc_path)?;
        }
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