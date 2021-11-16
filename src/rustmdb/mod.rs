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

    pub fn movie(&self, id: u64) -> Result<Movie, reqwest::Error>{
        let parameters = format!("api_key={}&language={}&append_to_response=credits", self.api_key, self.language);
        let body =reqwest::blocking::get(format!("https://api.themoviedb.org/3/movie/{}?{}",id, parameters))?;
        Ok(body.json()?)
    }

    pub fn tv(&self, id: u64) -> Result<Tv, reqwest::Error>{
        let parameters = format!("api_key={}&language={}&append_to_response=credits", self.api_key, self.language);
        let body =reqwest::blocking::get(format!("https://api.themoviedb.org/3/tv/{}?{}",id, parameters))?;
        Ok(body.json()?)
    }

    pub fn tv_episode(&self, id: u64, season: u64, episode: u64) -> Result<Option<TvEpisode>, reqwest::Error>{
        let parameters = format!("api_key={}&language={}&append_to_response=credits", self.api_key, self.language);
        let body =reqwest::blocking::get(format!("https://api.themoviedb.org/3/tv/{}/season/{}/episode/{}?{}",id, season, episode, parameters))?;
        if body.status().is_success(){
            Ok(Some(body.json()?))
        }
        else{
            println!("error episode for id:{} season:{} episode: {} error={:?}", id, season, episode, body.text()?);
            Ok(None)
        }

    }

    pub fn person(&self, id: u64) -> Result<Person, reqwest::Error>{
        let parameters = format!("api_key={}&language={}", self.api_key, self.language);
        let body =reqwest::blocking::get(format!("https://api.themoviedb.org/3/person/{}?{}",id, parameters))?;
        Ok(body.json()?)
    }

    
}