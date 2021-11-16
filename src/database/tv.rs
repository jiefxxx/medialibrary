use crate::rustmdb::{Tv, TvEpisode};

use super::SqlLibrary;


impl SqlLibrary{
    pub fn create_tv(&mut self ,tv: &Tv) -> Result<(Vec<u64>, Vec<String>), rusqlite::Error>{

        let tx = self.conn.transaction()?;

        let mut person_ids = Vec::new();
        let mut rsc_path = Vec::new();
        println!("adding tv {:?}", tv);
        tx.execute(
            "INSERT INTO Tvs (
                id,
                original_title,
                original_language,
                title,
                release_date,
                overview,
                popularity,
                poster_path,
                backdrop_path,
                status,
                vote_average,
                vote_count,
                in_production, 
                number_of_episodes,
                number_of_seasons,
                episode_run_time) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",

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
            println!("season {}", season.season_number);
            tx.execute(
                "INSERT INTO Seasons (
                    id,
                    tv_id,
                    season_number,
                    episode_count,
                    title,
                    overview,
                    poster_path,
                    release_date) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
    
                &[
                &season.id.to_string(),
                &tv.id.to_string(),
                &season.season_number.to_string(),
                &season.episode_count.to_string(),
                &season.name,
                &season.overview.as_ref().unwrap_or(&"".to_string()),
                &season.poster_path.as_ref().unwrap_or(&"".to_string()),
                &tv.first_air_date,],
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
                "INSERT OR IGNORE INTO TvGenres (
                    id,
                    name) values (?1, ?2)",
    
                &[
                &genre.id.to_string(),
                &genre.name],
            )?;

            tx.execute(
                "INSERT INTO TvGenreLinks (
                    genre_id,
                    tv_id) values (?1, ?2)",
    
                &[
                &genre.id.to_string(),
                &tv.id.to_string()],
            )?;
        }

        for cast in &tv.credits.cast{
            if  cast.order > 15{
                continue
            }
    
            tx.execute(
                "INSERT INTO TvCasts (
                    person_id,
                    tv_id,
                    character,
                    ord) values (?1, ?2, ?3, ?4)",
    
                &[
                &cast.id.to_string(),
                &tv.id.to_string(),
                &cast.character.as_ref().unwrap_or(&"".to_string()), 
                &cast.order.to_string()],
            )?;

            person_ids.push(cast.id)
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
            "INSERT INTO Episodes (
                id,
                season_id,
                tv_id,
                season_number,
                episode_number,
                release_date,
                title,
                overview,
                vote_average,
                vote_count) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",

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
            if cast.order > 15{
                continue
            }
    
            tx.execute(
                "INSERT INTO EpisodeCasts (
                    person_id,
                    episode_id,
                    character,
                    ord) values (?1, ?2, ?3, ?4)",
    
                &[
                &cast.id.to_string(),
                &episode.id.to_string(),
                &cast.character.as_ref().unwrap_or(&"".to_string()), 
                &cast.order.to_string()],
            )?;

            person_ids.push(cast.id)
        }

        tx.commit()?;

        Ok((person_ids, rsc_path))
    }

    pub fn get_season_id(&self, tv_id: u64, season_number: u64) -> Result<Option<u64>, rusqlite::Error> {
        println!("get season id {} {}", &tv_id, &season_number);
        let mut stmt = self.conn.prepare(
            "SELECT id from Seasons
             WHERE tv_id = ?1 and season_number = ?2",
        )?;
    
        let rows = stmt.query_map(&[&tv_id.to_string(), &season_number.to_string()], |row| row.get(0))?;
        for row in rows{
            return Ok(Some(row?))
        }
        Ok(None)
    }

    pub fn tv_exist(&self, tv_id: u64) -> Result<bool, rusqlite::Error>{
        let mut stmt = self.conn.prepare(
            "SELECT title from Tvs
             WHERE id = ?1",
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
            "SELECT id from Episodes
             WHERE tv_id = ?1 AND season_number = ?2 AND episode_number = ?3",
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