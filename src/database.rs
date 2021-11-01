use rusqlite::Connection;

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
                Adding TEXT
            )",
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
        Ok(())
    }

    pub fn create_video(&self, path: &str, media_type: u8, duration: f32, 
        bit_rate: f32, codec: &str, width: u32, height: u32, size: usize) -> Result<i64, rusqlite::Error>{

        self.conn.execute(
            "INSERT INTO videos (
                Path,
                VideoMediaType,
                Duration,
                BitRate,
                Codec,
                Width,
                Height,
                Size,
                Adding) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, datetime('now'))",
            &[path, 
            &media_type.to_string(),
            &duration.to_string(),
            &bit_rate.to_string(),
            &codec,
            &width.to_string(),
            &height.to_string(),
            &size.to_string()],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn create_video_subtitle(&self, video_id: i64, language: &str) -> Result<(), rusqlite::Error>{
        self.conn.execute(
            "INSERT INTO subtitles (
                SubtitleVideoID,
                SubtitleLanguage) values (?1, ?2)",
            &[&video_id.to_string(), language],
        )?;
        Ok(())
    }

    pub fn create_video_audio(&self, video_id: i64, language: &str) -> Result<(), rusqlite::Error>{
        self.conn.execute(
            "INSERT INTO audios (
                AudioVideoID,
                AudioLanguage) values (?1, ?2)",
            &[&video_id.to_string(), language],
        )?;
        Ok(())
    }

    pub fn edit_video_media_id(&self, video_id: i64, media_id: i64) -> Result<(), rusqlite::Error>{
        self.conn.execute(
            "UPDATE videos SET VideoMediaID = ?1 WHERE VideoID = ?2",
            &[
                &media_id.to_string(),
                &video_id.to_string()],
        )?;
        Ok(())
    }

    pub fn edit_video_path(&self, video_id: i64, path: &str) -> Result<(), rusqlite::Error>{
        self.conn.execute(
            "UPDATE videos SET VideoMediaID = ?1 WHERE VideoID = ?2",
            &[
                path,
                &video_id.to_string()],
        )?;
        Ok(())
    }


    pub fn edit_last_time(&self, video_id: i64, user_id: i64, last_time: i64) -> Result<(), rusqlite::Error>{
        self.conn.execute(
            "INSERT OR REPLACE INTO lastTime (
                LastTimeVideoID,
                LastTimeUserID,
                LastTimeValue) values (?1, ?2, ?3)",
            &[
                &video_id.to_string(),
                &user_id.to_string(),
                &last_time.to_string()],
        )?;
        Ok(())
    }

    pub fn get_video_id(&self, path: &str) -> Result<i64, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT videoID from videos
             WHERE path = ?1",
        )?;
    
        let rows = stmt.query_map(&[path], |row| row.get(0))?;
        for row in rows{
            return Ok(row?)
        }
        Err(rusqlite::Error::QueryReturnedNoRows)
    }


}

