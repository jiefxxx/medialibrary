use crate::rustmdb::{Tv, TvEpisode};

use super::SqlLibrary;


impl SqlLibrary{
    pub fn create_tv(&mut self ,tv: &Tv) -> Result<(Vec<u64>, Vec<String>), rusqlite::Error>{

        let tx = self.conn.transaction()?;

        let mut person_ids = Vec::new();
        let mut rsc_path = Vec::new();

        tx.execute(
            "INSERT INTO tvs (
                TvID,
                OriginalTitle,
                OriginalLanguage,
                Title,
                ReleaseDate,
                Overview,
                Popularity,
                PosterPath,
                BackdropPath,
                Status,
                VoteAverage,
                VoteCount,
                InProduction, 
                NumberOfEpisodes,
                NumberOfSeasons,
                EpisodeRunTime) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",

            &[
            &tv.id.to_string(),
            &tv.original_name,
            &tv.original_language,
            &tv.name,
            &tv.first_air_date,
            &tv.overview.as_ref().unwrap_or(&"".to_string()),
            &tv.popularity.to_string(),
            &tv.poster_path.as_ref().unwrap_or(&"".to_string()),
            &tv.backdrop_path.as_ref().unwrap_or(&"".to_string()),
            &tv.status,
            &tv.vote_average.to_string(),
            &tv.vote_count.to_string(),
            &tv.in_production.to_string(),
            &tv.number_of_episodes.to_string(),
            &tv.number_of_seasons.to_string(),
            &tv.episode_run_time.get(0).unwrap_or(&0).to_string()],
        )?;

        for season in &tv.seasons{
            tx.execute(
                "INSERT INTO seasons (
                    SeasonID,
                    TvID,
                    SeasonNumber,
                    EpisodeCount,
                    Title,
                    Overview,
                    PosterPath,
                    ReleaseDate) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
    
                &[
                &season.id.to_string(),
                &tv.id.to_string(),
                &season.season_number.to_string(),
                &season.episode_count.to_string(),
                &season.name,
                &season.overview.as_ref().unwrap_or(&"".to_string()),
                &season.poster_path.as_ref().unwrap_or(&"".to_string())],
            )?;
        }

        if let Some(backdrop_path) = &tv.backdrop_path{
            rsc_path.push(backdrop_path.clone())
        }

        if let Some(poster_path) = &tv.poster_path{
            rsc_path.push(poster_path.clone())
        }

        for genre in &tv.genres{
            tx.execute(
                "INSERT OR IGNORE INTO tv_genres (
                    GenreID,
                    GenreName) values (?1, ?2)",
    
                &[
                &genre.id.to_string(),
                &genre.name],
            )?;

            tx.execute(
                "INSERT INTO tv_genre_links (
                    GenreID,
                    TvID) values (?1, ?2)",
    
                &[
                &genre.id.to_string(),
                &tv.id.to_string()],
            )?;
        }

        for cast in &tv.credits.cast{
            if cast.cast_id.is_none(){
                continue
            }
    
            tx.execute(
                "INSERT INTO tv_casts (
                    PersonID,
                    TvID,
                    Character,
                    Order) values (?1, ?2, ?3, ?4)",
    
                &[
                &cast.cast_id.unwrap().to_string(),
                &tv.id.to_string(),
                &cast.character, 
                &cast.order.to_string()],
            )?;

            person_ids.push(cast.cast_id.unwrap())
        }

        tx.commit()?;

        Ok((person_ids, rsc_path))
    }

    pub fn create_episode(&mut self, tv_id: u64, episode: &TvEpisode) -> Result<(Vec<u64>, Vec<String>), rusqlite::Error>{
        let season_id = self.get_season_id(tv_id, episode.season_number).unwrap().unwrap();
        
        let tx = self.conn.transaction()?;

        let mut person_ids = Vec::new();
        let rsc_path = Vec::new();
        
        tx.execute(
            "INSERT INTO episodes (
                EpisodeID,
                SeasonID,
                TvID,
                SeasonNumber,
                EpisodeNumber,
                ReleaseDate,
                Title,
                Overview,
                VoteAverage,
                VoteCount) values (?1, ?2, ?3)",

            &[
            &episode.id.to_string(),
            &season_id.to_string(),
            &tv_id.to_string(),
            &episode.season_number.to_string(),
            &episode.episode_number.to_string(),
            &episode.air_date,
            &episode.name,
            &episode.overview.as_ref().unwrap_or(&"".to_string()),
            &episode.vote_average.to_string(),
            &episode.vote_count.to_string()],
        )?;

        for cast in &episode.credits.cast{
            if cast.cast_id.is_none(){
                continue
            }
    
            tx.execute(
                "INSERT INTO episode_casts (
                    PersonID,
                    EpisodeID,
                    Character,
                    Order) values (?1, ?2, ?3, ?4)",
    
                &[
                &cast.cast_id.unwrap().to_string(),
                &episode.id.to_string(),
                &cast.character, 
                &cast.order.to_string()],
            )?;

            person_ids.push(cast.cast_id.unwrap())
        }

        tx.commit()?;

        Ok((person_ids, rsc_path))
    }

    pub fn get_season_id(&self, tv_id: u64, season_number: u64) -> Result<Option<u64>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT SeasonID from seasons
             WHERE tvID = ?1 and SeasonNumber = ?2",
        )?;
    
        let rows = stmt.query_map(&[&tv_id.to_string(), &season_number.to_string()], |row| row.get(0))?;
        for row in rows{
            return Ok(Some(row?))
        }
        Ok(None)
    }

    pub fn tv_exist(&self, tv_id: u64) -> Result<bool, rusqlite::Error>{
        let mut stmt = self.conn.prepare(
            "SELECT Title from tvs
             WHERE tvID = ?1",
        )?;
    
        let rows = stmt.query_map(&[&tv_id.to_string()], |row| row.get(0))?;
        for row in rows{
            let _unused: String = row?;
            return Ok(true)
        }
        Ok(false)
    }

    pub fn episode_exist(&self, tv_id: u64, season: u64, episode: u64) -> Result<Option<u64>, rusqlite::Error>{
        let mut stmt = self.conn.prepare(
            "SELECT EpisodeID from episodes
             WHERE TvID = ?1 AND SeasonNumber = ?2 AND EpisodeNumber = ?3",
        )?;
    
        let rows = stmt.query_map(&[&tv_id.to_string(), 
                                                                    &season.to_string(), 
                                                                    &episode.to_string()], |row| row.get(0))?;
        for row in rows{
            return Ok(Some(row?))
        }
        Ok(None)
    }
}