use crate::rustmdb::model::{SearchMovie, SearchTv};

use super::Library;
use strsim::jaro;

impl Library {
    pub fn search_movie_id(&self, title: &str, year: u64) -> Option<u64>{
        let movies  = self.tmdb.search_movie(title).year(year).request().unwrap();
        if movies.results.len() == 0{
            return None
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

        Some(best.id)
    }

    pub fn search_tv_id(&self, title: &str) -> Option<u64>{
        let tvs  = self.tmdb.search_tv(title).request().unwrap();
        if tvs.results.len() == 0{
            return None
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
        Some(best.id)
    }
}