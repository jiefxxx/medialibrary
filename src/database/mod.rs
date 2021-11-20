use std::fmt;

use rusqlite::Connection;

mod video;
mod movie;
mod tv;
mod person;

pub struct SqlLibrary{
    conn: Connection
}

impl SqlLibrary{
    pub fn new(path: &str) ->  Result<SqlLibrary, rusqlite::Error>{
        Ok(
            SqlLibrary{
                conn: Connection::open(path)?
            }
        )
    }

    //video part

    pub fn init_db(&self) -> Result<(), rusqlite::Error>{
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Videos (
                id INTEGER PRIMARY KEY NOT NULL,
                path TEXT NOT NULL UNIQUE,
                media_type INTEGER,
                media_id INTEGER,
                duration INTEGER,
                bit_rate INTEGER,
                codec TEXT,
                width INTEGER,
                height INTEGER,
                size INTEGER,
                adding TEXT)",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS LastTime (
                video_id INTEGER NOT NULL,
                user_id INTEGER NOT NULL,
                last_time INTEGER,
                unique(video_id, user_id))",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Audios (
                video_id INTEGER NOT NULL,
                language TEXT,
                unique(video_id, language))",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Subtitles (
                video_id INTEGER NOT NULL,
                language TEXT,
                unique(video_id, language))",
            [],
        )?;

        self.conn.execute(
            "CREATE VIEW IF NOT EXISTS VideosView
                AS 
                SELECT
                    Videos.id as id,
                    path,
                    media_type,
                    media_id,
                    duration,
                    bit_rate,
                    codec,
                    width,
                    height,
                    Movies.title as m_title,
                    Tvs.title as t_title,
                    episode_number,
                    season_number,
                    Movies.release_date as release_date,
                    size,
                    adding,
                    GROUP_CONCAT(Subtitles.language) as subtitles,
                    GROUP_CONCAT(Audios.language) as audios
                FROM
                    Videos
                LEFT OUTER JOIN Audios ON Videos.id = Audios.video_id
                LEFT OUTER JOIN Subtitles ON Videos.id = Subtitles.video_id
                LEFT OUTER JOIN Movies ON Videos.media_type = 0 AND Videos.media_id = Movies.id
                LEFT OUTER JOIN Episodes ON Videos.media_type = 1 AND Videos.media_id = Episodes.id
                LEFT OUTER JOIN Tvs ON Episodes.tv_id = Tvs.id
                GROUP BY videos.id",
                []
        )?;

        // Movie Part

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Movies (
                id INTEGER PRIMARY KEY NOT NULL,
                original_title TEXT,
                original_language TEXT,
                title TEXT,
                release_date TEXT,
                overview TEXT,
                popularity FLOAT,
                poster_path TEXT,
                backdrop_path TEXT,
                vote_average FLOAT,
                vote_count INTEGER,
                tagline TEXT,
                status TEXT,
                adult BOOL)",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS MovieGenres (
                id INTEGER PRIMARY KEY NOT NULL,
                name TEXT)",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS MovieGenreLinks (
                movie_id INTEGER NOT NULL,
                genre_id INTEGER NOT NULL,
                unique(movie_id,genre_id))",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS MovieCasts (
                movie_id INTEGER NOT NULL,
                person_id TEXT,
                character TEXT,
                ord INTEGER,
                unique(movie_id,person_id,character))",
            [],
        )?;

        self.conn.execute(
            "CREATE VIEW IF NOT EXISTS MoviesView
                AS 
                SELECT
                    Movies.id as id,
                    original_title,
                    original_language,
                    title,
                    release_date,
                    overview,
                    popularity,
                    poster_path,
                    backdrop_path,
                    vote_average,
                    vote_count,
                    tagline,
                    status,
                    adult,
                    GROUP_CONCAT(MovieGenres.name) as genres,
                    MAX(Videos.adding) as adding,
                    GROUP_CONCAT(Videos.id) as video_ids
                FROM
                    Movies
                INNER JOIN Videos ON Movies.id == Videos.media_id AND Videos.media_type == 0
                LEFT OUTER JOIN MovieGenres ON Movies.id = MovieGenres.movie_id

                GROUP BY Movies.id",
                []
        )?;

        // Tv Part

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Tvs (
                id INTEGER PRIMARY KEY NOT NULL,
                original_title TEXT,
                original_language TEXT,
                title TEXT,
                release_date TEXT,
                overview TEXT,
                popularity FLOAT,
                poster_path TEXT,
                backdrop_path TEXT,
                status TEXT,
                vote_average FLOAT,
                vote_count INTEGER,
                in_production BOOL, 
                number_of_episodes INTEGER,
                number_of_seasons INTEGER,
                episode_run_time INTEGER)",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS TvGenres (
                id INTEGER PRIMARY KEY NOT NULL,
                name TEXT)",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS TvGenreLinks (
                tv_id INTEGER NOT NULL,
                genre_id INTEGER NOT NULL,
                unique(tv_id,genre_id))",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS TvCasts (
                tv_id INTEGER NOT NULL,
                person_id TEXT,
                character TEXT,
                ord INTEGER,
                unique(tv_id, person_id, character))",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Seasons (
                id INTEGER PRIMARY KEY NOT NULL,
                tv_id INTEGER NOT NULL,
                season_number INTEGER NOT NULL,
                episode_count INTEGER,
                title TEXT,
                overview TEXT,
                poster_path TEXT,
                release_date TEXT)",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Episodes (
                id INTEGER PRIMARY KEY NOT NULL,
                season_id INTEGER NOT NULL,
                tv_id INTEGER NOT NULL,
                season_number INTEGER NOT NULL,
                episode_number INTEGER NOT NULL,
                release_date TEXT,
                title TEXT,
                overview TEXT,
                vote_average FLOAT,
                vote_count INTEGER)",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS EpisodeCasts (
                episode_id INTEGER NOT NULL,
                person_id INTEGER NOT NULL,
                character TEXT,
                ord INTEGER,
                unique(episode_id,person_id,character))",
            [],
        )?;


        //Person Part
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Persons (
                id INTEGER PRIMARY KEY NOT NULL,
                birthday TEXT,
                known_for_department TEXT,
                deathday TEXT,
                name TEXT,
                gender INTEGER,
                biography TEXT,
                popularity FLOAT,
                place_of_Birth TEXT,
                profile_path TEXT)",
            []
        )?;

        Ok(())
    }
}

pub fn parse_concat( row: Option<String>) -> Option<Vec<String>>{
    if let Some(row)= row{
        return Some(row.split(",").map(|s| s.to_string()).collect())
    }
    None
}

#[derive(Debug)]
pub enum ErrorKind{
    Unknwon,
}

#[derive(Debug)]
pub struct Error{
    kind: ErrorKind,
    description: String,
    location: String,
}

impl Error{
    pub fn new(kind: ErrorKind, description: String, location: &str) -> Error{
        Error{
            kind,
            description,
            location: location.to_string(), 
        }
    }

    pub fn from_reqwest(e: rusqlite::Error, location: &str) -> Error{
        Error::new(ErrorKind::Unknwon, e.to_string(), location)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {:?} at {} {}", &self.kind, &self.location, &self.description)
    }
}

impl std::convert::From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Error {
        Error::from_reqwest(err, "Undefined")
    }
}