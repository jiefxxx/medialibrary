use super::model::{SearchResult, SearchTv};

pub struct TvSearch <'a>{
    api_key: &'a str,
    language: &'a str,
    query: &'a str,
    page: u64,
    include_adult: Option<bool>,
    first_air_date_year: Option<u64>
}

impl <'a>TvSearch<'a> {
    pub fn new(api_key: &'a str, query: &'a str, language: &'a str) -> TvSearch<'a>{
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