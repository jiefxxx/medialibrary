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
            "CREATE TABLE IF NOT EXISTS videos (
                VideoID INTEGER PRIMARY KEY NOT NULL,
                Path TEXT NOT NULL UNIQUE,
                VideoMediaType INTEGER,
                VideoMediaID INTEGER,
                Duration FLOAT,
                BitRate FLOAT,
                Codec TEXT,
                Width INTEGER,
                Height INTEGER,
                Size INTEGER,
                Adding TEXT)",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS lastTime (
                LastTimeVideoID INTEGER NOT NULL,
                LastTimeUserID INTEGER NOT NULL,
                LastTimeValue INTEGER,
                unique(LastTimeVideoID, LastTimeUserID))",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS audios (
                AudioVideoID INTEGER NOT NULL,
                AudioLanguage TEXT,
                unique(AudioVideoID, AudioLanguage))",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS subtitles (
                SubtitleVideoID INTEGER NOT NULL,
                SubtitleLanguage TEXT,
                unique(SubtitleVideoID, SubtitleLanguage))",
            [],
        )?;

        // Movie Part

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS movies (
                MovieID INTEGER PRIMARY KEY NOT NULL,
                OriginalTitle TEXT,
                OriginalLanguage TEXT,
                Title TEXT,
                ReleaseDate TEXT,
                Overview TEXT,
                Popularity FLOAT,
                PosterPath TEXT,
                BackdropPath TEXT,
                VoteAverage FLOAT,
                VoteCount INTEGER,
                Tagline TEXT,
                Status TEXT,
                Adult BOOL)",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS movie_genres (
                GenreID INTEGER PRIMARY KEY NOT NULL,
                GenreName TEXT)",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS movie_genre_links (
                MovieID INTEGER NOT NULL,
                GenreID INTEGER NOT NULL,
                UNIQUE(MovieID,GenreID))",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS movie_casts (
                MovieID INTEGER NOT NULL,
                PersonID TEXT,
                Character TEXT,
                Order INTEGER,
                UNIQUE(MovieID,PersonID,Character))",
            [],
        )?;

        // Tv Part

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tvs (
                TvID INTEGER PRIMARY KEY NOT NULL,
                OriginalTitle TEXT,
                OriginalLanguage TEXT,
                Title TEXT,
                ReleaseDate TEXT,
                Overview TEXT,
                Popularity FLOAT,
                PosterPath TEXT,
                BackdropPath TEXT,
                Status TEXT,
                VoteAverage FLOAT,
                VoteCount INTEGER,
                InProduction BOOL, 
                NumberOfEpisodes INTEGER,
                NumberOfSeasons INTEGER,
                EpisodeRunTime INTEGER)",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tv_genres (
                GenreID INTEGER PRIMARY KEY NOT NULL,
                GenreName TEXT)",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tv_genre_links (
                TvID INTEGER NOT NULL,
                GenreID INTEGER NOT NULL,
                UNIQUE(TvID,GenreID))",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tv_casts (
                TvID INTEGER NOT NULL,
                PersonID TEXT,
                Character TEXT,
                Order INTEGER,
                UNIQUE(TvID,PersonID,Character))",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS seasons (
                SeasonID INTEGER PRIMARY KEY NOT NULL,
                TvID INTEGER NOT NULL,
                SeasonNumber INTEGER NOT NULL,
                EpisodeCount INTEGER,
                Title TEXT,
                Overview TEXT,
                PosterPath TEXT,
                ReleaseDate TEXT)",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS episodes (
                EpisodeID INTEGER PRIMARY KEY NOT NULL,
                SeasonID INTEGER NOT NULL,
                TvID INTEGER NOT NULL,
                SeasonNumber INTEGER NOT NULL,
                EpisodeNumber INTEGER NOT NULL,
                ReleaseDate TEXT,
                Title TEXT,
                Overview TEXT,
                VoteAverage FLOAT,
                VoteCount INTEGER)",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS episode_casts (
                EpisodeID INTEGER NOT NULL,
                PersonID INTEGER NOT NULL,
                Character TEXT,
                Order INTEGER,
                UNIQUE(EpisodeID,PersonID,Character))",
            [],
        )?;


        //Person Part
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS persons (
                PersonID INTEGER PRIMARY KEY NOT NULL,
                Birthday TEXT,
                KnownForDepartment TEXT,
                Deathday TEXT,
                Name TEXT,
                Gender INTEGER,
                Biography TEXT,
                Popularity FLOAT,
                PlaceOfBirth TEXT,
                ProfilePath TEXT)",
            []
        )?;

        Ok(())
    }
}