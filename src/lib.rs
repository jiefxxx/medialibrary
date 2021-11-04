mod database;

use pyo3::prelude::*;
use pyo3::exceptions::PyReferenceError;

use database::SqlLibrary;

use tmdb::model::*;
use tmdb::themoviedb::*;

use std::io;
use std::fs::File;

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}


#[pyclass]
struct Library {
    conn: SqlLibrary,
    tmdb: TMDb,
    rsc_path: String,
}

#[pymethods]
impl Library {
    #[new]
    fn new(database_path: &str, api_key: &str, language: &str, rsc_path: String) -> Self {
        let api_key = string_to_static_str(api_key.to_string());
        let language = string_to_static_str(language.to_string());
        let tmdb = TMDb { api_key: api_key, language: &language};
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

    pub fn edit_movie(&self, video_id: u64, movie_id: u64) -> PyResult<()>{
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

    pub fn edit_tv(&self, video_id: u64, tv_id: u64, season: u32, episode: u32) -> PyResult<()>{
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

    fn update_db_movie(&self, movie_id: u64) -> PyResult<()>{
        match self.conn.movie_exist(movie_id) {
            Ok(true) => return Ok(()),
            Err(e) => return Err(PyReferenceError::new_err(format!("database error movie exist {}", e))),
            _ => ()
        };

        let movie: Movie = match self.tmdb.fetch().id(movie_id).append_credits().execute(){
            Ok(movie) => movie,
            Err(e) => return Err(PyReferenceError::new_err(format!("tmdb error {} for MovieID {:?}", e, movie_id))),
        };

        let overview = match movie.overview {
            Some(overview) => overview,
            None => "".to_string(),
        };

        let poster_path = match movie.poster_path {
            Some(poster_path) => poster_path,
            None => "".to_string(),
        };

        if let Err(e) = self.conn.create_movie(movie.id, 
            &movie.original_title, &movie.original_language, &movie.title, &movie.release_date,
            &overview, movie.popularity, &poster_path){
            return Err(PyReferenceError::new_err(format!("database error create movie{}", e)))
        }

        for genre in movie.genres{
            if let Err(e) = self.conn.create_movie_genre(genre.id, &genre.name){
                return Err(PyReferenceError::new_err(format!("database error create movie genre {}", e)))
            }
            if let Err(e) = self.conn.link_movie_genre(movie.id, genre.id){
                return Err(PyReferenceError::new_err(format!("database error create movie genre {}", e)))
            }
        }

        if let Some(credits) = movie.credits{
            for cast in credits.cast{
                if let Err(e) = self.conn.create_movie_cast(movie.id, &cast.name, &cast.character, cast.order.into()){
                    return Err(PyReferenceError::new_err(format!("database error create movie cast {}", e)))
                }
            }
        }

        self.update_rsc(&poster_path)?;

        Ok(())
    }

    fn update_db_tv(&self, tv_id: u64) -> PyResult<()>{
        match self.conn.tv_exist(tv_id) {
            Ok(false) => return Ok(()),
            Err(e) => return Err(PyReferenceError::new_err(format!("database error tv exist {}", e))),
            _ => ()
        };

        let tv: TV = match self.tmdb.fetch().id(tv_id).append_credits().execute(){
            Ok(tv) => tv,
            Err(e) => return Err(PyReferenceError::new_err(format!("tmdb error {} for MovieID {:?}", e, tv_id))),
        };

        let poster_path = match tv.poster_path {
            Some(poster_path) => poster_path,
            None => "".to_string(),
        };

        if let Err(e) = self.conn.create_tv(tv.id, 
            &tv.original_name, &tv.original_language, &tv.name, &tv.first_air_date,
            &tv.overview, tv.popularity, &poster_path){
            return Err(PyReferenceError::new_err(format!("database error create movie{}", e)))
        }

        for genre in tv.genres{
            if let Err(e) = self.conn.create_tv_genre(genre.id, &genre.name){
                return Err(PyReferenceError::new_err(format!("database error create movie genre {}", e)))
            }
            if let Err(e) = self.conn.link_tv_genre(tv.id, genre.id){
                return Err(PyReferenceError::new_err(format!("database error create movie genre {}", e)))
            }
        }

        for season in tv.seasons {

            let poster_path = match season.poster_path {
                Some(poster_path) => poster_path,
                None => "".to_string(),
            };
            
            if let Err(e) = self.conn.create_season(tv.id, season.season_number, season.episode_count, 
                &season.name, &season.overview, &poster_path){
                return Err(PyReferenceError::new_err(format!("database error create tv season {}", e)))
            }

            self.update_rsc(&poster_path)?;
        }

        if let Some(credits) = tv.credits{
            for cast in credits.cast{
                if let Err(e) = self.conn.create_tv_cast(tv.id, &cast.name, &cast.character, cast.order){
                    return Err(PyReferenceError::new_err(format!("database error create movie cast {}", e)))
                }
            }
        }

        self.update_rsc(&poster_path)?;

        Ok(())

    }

    fn update_db_episode(&self, tv_id: u64, season: u32, episode: u32) -> PyResult<u64>{
        
        self.update_db_tv(tv_id)?;

        match self.conn.episode_exist(tv_id, season, episode) {
            Ok(Some(episode_id)) => return Ok(episode_id),
            Err(e) => return Err(PyReferenceError::new_err(format!("database error episode exist {}", e))),
            _ => ()
        };

        Ok(match self.conn.create_episode(tv_id, season, episode){
            Ok(episode_id) => episode_id,
            Err(e) => return Err(PyReferenceError::new_err(format!("database error create episode {}", e))),
        })
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
