use super::SqlLibrary;


impl SqlLibrary{
    pub fn create_tv(&self ,tv_id: u64, original_title: &str, 
        original_language: &str, title: &str, release_date: &str, 
        overview: &str, popularity: f64, poster_path: &str) -> Result<(), rusqlite::Error>{

        self.conn.execute(
            "INSERT INTO tvs (
                TvID,
                OriginalTitle,
                OriginalLanguage,
                Title,
                ReleaseDate,
                Overview,
                popularity,
                PosterPath,
                BackdropPath) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",

            &[
            &tv_id.to_string(),
            original_title,
            original_language,
            title,
            release_date,
            overview,
            &popularity.to_string(),
            poster_path],
        )?;

        Ok(())
    }

    pub fn create_tv_genre(&self ,genre_id: u64, genre_name: &str) -> Result<(), rusqlite::Error>{
        self.conn.execute(
            "INSERT OR IGNORE INTO tv_genres (
                GenreID,
                GenreName) values (?1, ?2)",

            &[
            &genre_id.to_string(),
            genre_name],
        )?;

        Ok(())
    }

    pub fn create_tv_cast(&self, tv_id: u64, actor_name: &str, character: &str, order: u32) -> Result<(), rusqlite::Error>{
        self.conn.execute(
            "INSERT INTO tv_casts (
                ActorName,
                TvID,
                Character,
                Order) values (?1, ?2, ?3, ?4)",

            &[
            actor_name,
            &tv_id.to_string(),
            character, 
            &order.to_string()],
        )?;

        Ok(())
    }

    pub fn create_season(&self, tv_id: u64, season: u32, episode_count: u32, name: &str, overview: &str, poster_path: &str) -> Result<(), rusqlite::Error>{
        self.conn.execute(
            "INSERT INTO seasons (
                TvID INTEGER NOT NULL,
                SeasonNumber INTEGER NOT NULL,
                EpisodeCount INTEGER,
                Name TEXT,
                Overview TEXT,
                PosterPath TEXT,) values (?1, ?2, ?3, ?4, ?5, ?6, ?7)",

            &[
            &tv_id.to_string(),
            &season.to_string(),
            &episode_count.to_string(),
            name,
            overview,
            poster_path],
        )?;

        Ok(())
    }

    pub fn create_episode(&self, tv_id: u64, season: u32, episode: u32) -> Result<u64, rusqlite::Error>{

        self.conn.execute(
            "INSERT INTO episodes (
                TvID,
                SeasonNumber,
                EpisodeNumber) values (?1, ?2, ?3)",

            &[
            &tv_id.to_string(),
            &season.to_string(),
            &episode.to_string()],
        )?;

        Ok(self.conn.last_insert_rowid() as u64)
    }

    pub fn link_tv_genre(&self, tv_id: u64, genre_id: u64) -> Result<(), rusqlite::Error>{
        self.conn.execute(
            "INSERT INTO tv_genre_links (
                GenreID,
                TvID) values (?1, ?2)",

            &[
            &genre_id.to_string(),
            &tv_id.to_string()],
        )?;

        Ok(())
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
    pub fn episode_exist(&self, tv_id: u64, season: u32, episode: u32) -> Result<Option<u64>, rusqlite::Error>{
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