use super::SqlLibrary;


impl SqlLibrary{
    pub fn create_tv(&self ,tv_id: i64, original_title: &str, 
        original_language: &str, title: &str, release_date: &str, 
        overview: &str, vote_average: f32, poster_path: &str) -> Result<(), rusqlite::Error>{

        self.conn.execute(
            "INSERT INTO tvs (
                TvID,
                OriginalTitle,
                OriginalLanguage,
                Title,
                ReleaseDate,
                Overview,
                VoteAverage,
                PosterPath,
                BackdropPath) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",

            &[
            &tv_id.to_string(),
            original_title,
            original_language,
            title,
            release_date,
            overview,
            &vote_average.to_string(),
            poster_path],
        )?;

        Ok(())
    }

    pub fn create_tv_genre(&self ,genre_id: i64, genre_name: &str) -> Result<(), rusqlite::Error>{
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

    pub fn create_tv_cast(&self, tv_id: i64, actor_id: i64, character: &str) -> Result<(), rusqlite::Error>{
        self.conn.execute(
            "INSERT INTO tv_casts (
                ActorID,
                TvID,
                Character) values (?1, ?2, ?3)",

            &[
            &actor_id.to_string(),
            &tv_id.to_string(),
            character],
        )?;

        Ok(())
    }

    pub fn create_episode(&self, episode_id: i64, title: &str, release_date: &str, overview: &str, vote_average: f32, season: i32, episode: i32) -> Result<(), rusqlite::Error>{

        self.conn.execute(
            "INSERT INTO episodes (
                EpisodeID INTEGER PRIMARY KEY NOT NULL,
                EpisodeTitle TEXT,
                EpisodeReleaseDate TEXT,
                EpisodeOverview TEXT,
                EpisodeVoteAverage FLOAT,
                SeasonNumber INTEGER NOT NULL,
                EpisodeNumber INTEGER NOT NULL) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",

            &[
            &episode_id.to_string(),
            title,
            release_date,
            overview,
            &vote_average.to_string(),
            &season.to_string(),
            &episode.to_string()],
        )?;

        Ok(())
    }

    pub fn create_episode_cast(&self, episode_id: i64, actor_id: i64, character: &str) -> Result<(), rusqlite::Error>{
        self.conn.execute(
            "INSERT INTO episode_casts (
                ActorID,
                EpisodeID,
                Character) values (?1, ?2, ?3)",

            &[
            &actor_id.to_string(),
            &episode_id.to_string(),
            character],
        )?;

        Ok(())
    }

    pub fn link_tv_genre(&self, tv_id: i64, genre_id: i64) -> Result<(), rusqlite::Error>{
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

    pub fn link_tv_episode(&self, tv_id: i64, episode_id: i64) -> Result<(), rusqlite::Error>{
        self.conn.execute(
            "INSERT INTO tv_episode_links (
                EpisodeID,
                TvID) values (?1, ?2)",

            &[
            &episode_id.to_string(),
            &tv_id.to_string()],
        )?;

        Ok(())
    }

    pub fn tv_exist(&self, tv_id: i64) -> Result<bool, rusqlite::Error>{
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
    pub fn episode_exist(&self, episode_id: i64) -> Result<bool, rusqlite::Error>{
        let mut stmt = self.conn.prepare(
            "SELECT Title from episodes
             WHERE EpisodeID = ?1",
        )?;
    
        let rows = stmt.query_map(&[&episode_id.to_string()], |row| row.get(0))?;
        for row in rows{
            let _unused: String = row?;
            return Ok(true)
        }
        Ok(false)
    }
}