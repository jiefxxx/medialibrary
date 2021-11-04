use super::SqlLibrary;


impl SqlLibrary{
    pub fn create_movie(&self ,movie_id: u64, original_title: &str, 
        original_language: &str, title: &str, release_date: &str, 
        overview: &str, popularity: f64, poster_path: &str) -> Result<(), rusqlite::Error>{

        self.conn.execute(
            "INSERT INTO movies (
                MovieID,
                OriginalTitle,
                OriginalLanguage,
                Title,
                ReleaseDate,
                Overview,
                Popularity,
                PosterPath,
                BackdropPath) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",

            &[
            &movie_id.to_string(),
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

    pub fn create_movie_genre(&self ,genre_id: u64, genre_name: &str) -> Result<(), rusqlite::Error>{
        self.conn.execute(
            "INSERT OR IGNORE INTO movie_genres (
                GenreID,
                GenreName) values (?1, ?2)",

            &[
            &genre_id.to_string(),
            genre_name],
        )?;

        Ok(())
    }

    pub fn create_movie_cast(&self, movie_id: u64, actor_name: &str, character: &str, order: u32) -> Result<(), rusqlite::Error>{
        self.conn.execute(
            "INSERT INTO movie_casts (
                ActorName,
                MovieID,
                Character,
                Order) values (?1, ?2, ?3, ?4)",

            &[
            actor_name,
            &movie_id.to_string(),
            character, 
            &order.to_string()],
        )?;

        Ok(())
    }

    pub fn link_movie_genre(&self, movie_id: u64, genre_id: u64) -> Result<(), rusqlite::Error>{
        self.conn.execute(
            "INSERT INTO movie_genre_links (
                GenreID,
                MovieID) values (?1, ?2)",

            &[
            &genre_id.to_string(),
            &movie_id.to_string()],
        )?;

        Ok(())
    }

    pub fn movie_exist(&self, movie_id: u64) -> Result<bool, rusqlite::Error>{
        let mut stmt = self.conn.prepare(
            "SELECT OriginalTitle from movies
             WHERE MediaID = ?1",
        )?;
    
        let rows = stmt.query_map(&[&movie_id.to_string()], |row| row.get(0))?;
        for row in rows{
            let _unused: String = row?;
            return Ok(true)
        }
        Ok(false)
    }
}