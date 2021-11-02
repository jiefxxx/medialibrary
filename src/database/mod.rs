use rusqlite::Connection;

mod video;
mod movie;
mod actor;
mod tv;

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
                VoteAverage FLOAT,
                PosterPath TEXT,
                BackdropPath TEXT)",
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
                ActorID INTEGER NOT NULL,
                Character TEXT,
                UNIQUE(MovieID,ActorID,Character))",
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
                VoteAverage FLOAT,
                PosterPath TEXT,
                BackdropPath TEXT)",
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
                ActorID INTEGER NOT NULL,
                Character TEXT,
                UNIQUE(TvID,ActorID,Character))",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tv_episode_links (
                TvID INTEGER NOT NULL,
                EpisodeID INTEGER NOT NULL,
                UNIQUE(TvID,EpisodeID))",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS episodes (
                EpisodeID INTEGER PRIMARY KEY NOT NULL,
                EpisodeTitle TEXT,
                EpisodeReleaseDate TEXT,
                EpisodeOverview TEXT,
                EpisodeVoteAverage FLOAT,
                SeasonNumber INTEGER NOT NULL,
                EpisodeNumber INTEGER NOT NULL)",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS episode_casts (
                EpisodeID INTEGER NOT NULL,
                ActorID INTEGER NOT NULL,
                Character TEXT,
                UNIQUE(EpisodeID,ActorID,Character))",
            [],
        )?;

        //actor part

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS actors (
                ActorID NTEGER PRIMARY KEY NOT NULL,
                Birthday  TEXT,
                Deathday  TEXT,
                Gender  INTEGER,
                Name  TEXT,
                PlaceOfBirth  TEXT,
                ProfilePath  TEXT)",
            [],
        )?;

        Ok(())
    }
}