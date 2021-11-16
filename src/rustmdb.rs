extern crate reqwest;
extern crate serde;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Person {
    pub birthday: Option<String>,
    pub known_for_department: Option<String>,
    pub deathday: Option<String>,
    pub id: u64,
    pub name: String,
    pub also_known_as: Vec<String>,
    pub gender: u8,
    pub biography: String,
    pub popularity: f64,
    pub place_of_birth: Option<String>,
    pub profile_path:  Option<String>,
    pub adult: bool,
    pub imdb_id: Option<String>,
    pub homepage: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct SearchResult<T>{
    pub page: u64,
    pub total_results: u64,
    pub total_pages: u64,
    pub results: Vec<T>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct SearchMovie {
    pub id: u64,
    pub title: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: String,
    pub genre_ids: Vec<u16>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub adult: bool,
    pub vote_count: u64,
    pub vote_average: f64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ProductionCompanie {
    pub id: u64,
    pub name: String,
    pub logo_path: Option<String>,
    pub origin_country: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ProductionCountrie {
    pub iso_3166_1: String,
    pub name: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Language {
    pub iso_639_1: String,
    pub name: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Cast {
    pub adult: bool,
    pub gender: Option<u8>,
    pub id: u64,
    pub known_for_department: Option<String>,
    pub name: String,
    pub original_name: String,
    pub popularity: f64,
    pub profile_path: Option<String>,
    pub cast_id: Option<u64>,
    pub character: Option<String>,
    pub credit_id: String,
    pub order: u64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Crew {
    pub adult: bool,
    pub gender: Option<u8>,
    pub id: u64,
    pub known_for_department: Option<String>,
    pub name: String,
    pub original_name: String,
    pub popularity: f64,
    pub profile_path: Option<String>,
    pub credit_id: String,
    pub department: String,
    pub job: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Credits {
    pub cast: Vec<Cast>,
    pub crew: Vec<Crew>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Movie {
    pub id: u64,
    pub budget: u64,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub homepage: Option<String>,
    pub title: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: String,
    pub popularity: f64,
    pub adult: bool,
    pub vote_count: u64,
    pub vote_average: f64,
    pub tagline: Option<String>,
    pub status: String,
    pub genres: Vec<Genre>,
    pub production_companies: Vec<ProductionCompanie>,
    pub production_countries: Vec<ProductionCountrie>,
    pub spoken_languages: Vec<Language>,
    pub credits: Credits,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct CreatedBy {
    pub gender: Option<u8>,
    pub id: u64,
    pub name: String,
    pub profile_path: Option<String>,
    pub credit_id: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct LastEpisodeToAir {
   pub air_date: String,
   pub episode_number: u64,
   pub id: u64,
   pub name: String,
   pub overview: String,
   pub production_code: String,
   pub season_number: u64,
   pub still_path: Option<String>,
   pub vote_average: f64,
   pub vote_count: u64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Network {
   pub name: String,
   pub id: u64,
   pub logo_path: Option<String>,
   pub origin_country: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Season {
    pub air_date: Option<String>,
    pub episode_count: u64,
    pub name: String,
    pub id: u64,
    pub poster_path: Option<String>,
    pub season_number: u64,
    pub overview: Option<String>,
}


#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Tv {
    pub id: u64,
    pub backdrop_path: Option<String>,
    pub poster_path: Option<String>,
    pub episode_run_time: Vec<u64>,
    pub first_air_date: String,
    pub genres: Vec<Genre>,
    pub homepage: Option<String>,
    pub in_production: bool,
    pub languages: Vec<String>,
    pub last_air_date: String,
    pub name: String,
    pub number_of_episodes: u64,
    pub number_of_seasons: u64,
    pub origin_country: Vec<String>,
    pub original_language: String,
    pub original_name: String,
    pub overview: Option<String>,
    pub popularity: f64,
    pub production_companies: Vec<ProductionCompanie>,
    pub production_countries: Vec<ProductionCountrie>,
    pub spoken_languages: Vec<Language>,
    pub credits: Credits,
    pub status: String,
    pub tagline: String,
    pub vote_count: u64,
    pub vote_average: f64,
    pub created_by: Vec<CreatedBy>,
    pub last_episode_to_air: LastEpisodeToAir,
    pub networks: Vec<Network>,
    pub seasons: Vec<Season>,
}


#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct GuestStar{
    pub id: u64,
    pub name: String,
    pub credit_id: String,
    pub character: String,
    pub order: u64, 
    pub profile_path: Option<String>, 
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TvEpisode{
    pub air_date: String,
    pub guest_stars: Vec<GuestStar>,
    pub name: String,
    pub overview: Option<String>,
    pub id: u64,
    pub production_code: Option<String>,
    pub season_number: u64,
    pub episode_number: u64,
    pub still_path: Option<String>,
    pub vote_average: f64,
    pub vote_count: u64,
    pub credits: Credits,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct SearchTv {
    pub id: u64,
    pub name: String,
    pub original_name: String,
    pub original_language: String,
    pub original_country: Option<Vec<String>>,
    pub overview: Option<String>,
    pub first_air_date: Option<String>,
    pub genre_ids: Vec<u16>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub vote_count: u64,
    pub vote_average: f64,
}
pub struct TvSearch <'a>{
    api_key: &'a str,
    language: &'a str,
    query: &'a str,
    page: u64,
    include_adult: Option<bool>,
    first_air_date_year: Option<u64>
}

impl <'a>TvSearch<'a> {
    fn new(api_key: &'a str, query: &'a str, language: &'a str) -> TvSearch<'a>{
        TvSearch{
            api_key,
            language,
            query,
            page: 1,
            include_adult: None,
            first_air_date_year: None,
        }
    }
    #[allow(dead_code)]
    pub fn page(&mut self, page: u64) -> &mut TvSearch<'a>{
        self.page = page;
        self
    }

    #[allow(dead_code)]
    pub fn language(&mut self, language: &'a str) -> &mut TvSearch<'a>{
        self.language = language;
        self
    }

    #[allow(dead_code)]
    pub fn include_adult(&mut self, include_adult: bool) -> &mut TvSearch<'a>{
        self.include_adult = Some(include_adult);
        self
    }

    #[allow(dead_code)]
    pub fn request(&self) -> Result<SearchResult<SearchTv>, reqwest::Error>{

        let mut parameters = format!("api_key={}&query={}&page={}&language={}", self.api_key, self.query, self.page, self.language);

        if let Some(include_adult) = self.include_adult{
            parameters += "&include_adult=";
            parameters += &include_adult.to_string();
        }

        if let Some(first_air_date_year) = self.first_air_date_year{
            parameters += "&first_air_date_year=";
            parameters += &first_air_date_year.to_string();
        }
        let body =reqwest::blocking::get(format!("https://api.themoviedb.org/3/search/tv?{}",parameters))?;
        Ok(body.json()?)
    }
}

pub struct MovieSearch <'a>{
    api_key: &'a str,
    language: &'a str,
    query: &'a str,
    page: u64,
    include_adult: Option<bool>,
    region: Option<&'a str>,
    year: Option<u64>,
    primary_release_year: Option<u64>
}

impl <'a>MovieSearch<'a> {
    fn new(api_key: &'a str, query: &'a str, language: &'a str) -> MovieSearch<'a>{
        MovieSearch{
            api_key,
            language,
            query,
            page: 1,
            include_adult: None,
            region: None,
            year: None,
            primary_release_year: None,
        }
    }

    #[allow(dead_code)]
    pub fn page(&mut self, page: u64) -> &mut MovieSearch<'a>{
        self.page = page;
        self
    }

    #[allow(dead_code)]
    pub fn language(&mut self, language: &'a str) -> &mut MovieSearch<'a>{
        self.language = language;
        self
    }

    #[allow(dead_code)]
    pub fn include_adult(&mut self, include_adult: bool) -> &mut MovieSearch<'a>{
        self.include_adult = Some(include_adult);
        self
    }

    #[allow(dead_code)]
    pub fn region(&mut self, region: &'a str)-> &mut MovieSearch<'a>{
        self.region = Some(region);
        self
    }

    pub fn year(&mut self, year: u64)-> &mut MovieSearch<'a>{
        self.year = Some(year);
        self
    }

    #[allow(dead_code)]
    pub fn primary_release_year(&mut self, primary_release_year: u64)-> &mut MovieSearch<'a>{
        self.primary_release_year = Some(primary_release_year);
        self
    }



    pub fn request(&self) -> Result<SearchResult<SearchMovie>, reqwest::Error>{

        let mut parameters = format!("api_key={}&query={}&page={}&language={}", self.api_key, self.query, self.page, self.language);

        if let Some(region) = self.region{
            parameters += "&region=";
            parameters += region;
        }

        if let Some(include_adult) = self.include_adult{
            parameters += "&include_adult=";
            parameters += &include_adult.to_string();
        }

        if let Some(year) = self.year{
            parameters += "&year=";
            parameters += &year.to_string();
        }

        if let Some(primary_release_year) = self.primary_release_year{
            parameters += "&primary_release_year=";
            parameters += &primary_release_year.to_string();
        }
        let body =reqwest::blocking::get(format!("https://api.themoviedb.org/3/search/movie?{}",parameters))?;
        Ok(body.json()?)
    }
}

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