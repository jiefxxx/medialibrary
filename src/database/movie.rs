use crate::rustmdb::Movie;

use super::SqlLibrary;


impl SqlLibrary{
    pub fn create_movie(&mut self, movie: &Movie) -> Result<(Vec<u64>, Vec<String>), rusqlite::Error>{
        let tx = self.conn.transaction()?;

        let mut person_ids = Vec::new();
        let mut rsc_path = Vec::new();

        tx.execute(
            "INSERT INTO movies (
                MovieID,
                OriginalTitle,
                OriginalLanguage,
                Title,
                ReleaseDate,
                Overview,
                Popularity,
                PosterPath,
                BackdropPath,
                VoteAverage,
                VoteCount,
                Tagline,
                Status,
                Adult) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",

            &[
                &movie.id.to_string(),
                &movie.original_title,
                &movie.original_language,
                &movie.title,
                &movie.release_date,
                &movie.overview.as_ref().unwrap_or(&"".to_string()),
                &movie.popularity.to_string(),
                &movie.poster_path.as_ref().unwrap_or(&"".to_string()),
                &movie.backdrop_path.as_ref().unwrap_or(&"".to_string()),
                &movie.vote_average.to_string(),
                &movie.vote_count.to_string(),
                &movie.tagline.as_ref().unwrap_or(&"".to_string()),
                &movie.status,
                &movie.adult.to_string()],
        )?;

        if let Some(backdrop_path) = &movie.backdrop_path{
            rsc_path.push(backdrop_path.clone())
        }

        if let Some(poster_path) = &movie.poster_path{
            rsc_path.push(poster_path.clone())
        }

        for genre in &movie.genres{
            tx.execute(
                "INSERT OR IGNORE INTO movie_genres (
                    GenreID,
                    GenreName) values (?1, ?2)",
    
                &[
                &genre.id.to_string(),
                &genre.name],
            )?;

            tx.execute(
                "INSERT INTO movie_genre_links (
                    GenreID,
                    MovieID) values (?1, ?2)",
    
                &[
                &genre.id.to_string(),
                &movie.id.to_string()],
            )?;
        }

        for cast in &movie.credits.cast{
            if cast.cast_id.is_none(){
                continue
            }
    
            tx.execute(
                "INSERT INTO movie_casts (
                    PersonID,
                    MovieID,
                    Character,
                    Order) values (?1, ?2, ?3, ?4)",
    
                &[
                &cast.cast_id.unwrap().to_string(),
                &movie.id.to_string(),
                &cast.character, 
                &cast.order.to_string()],
            )?;

            person_ids.push(cast.cast_id.unwrap())
        }

        tx.commit()?;

        Ok((person_ids, rsc_path))
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