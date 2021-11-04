use super::SqlLibrary;


impl SqlLibrary{

    pub fn create_video(&self, path: &str, media_type: u8, duration: f32, 
        bit_rate: f32, codec: &str, width: u32, height: u32, size: usize) -> Result<u64, rusqlite::Error>{

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

        Ok(self.conn.last_insert_rowid() as u64)
    }

    pub fn create_video_subtitle(&self, video_id: u64, language: &str) -> Result<(), rusqlite::Error>{
        self.conn.execute(
            "INSERT INTO subtitles (
                SubtitleVideoID,
                SubtitleLanguage) values (?1, ?2)",
            &[&video_id.to_string(), language],
        )?;
        Ok(())
    }

    pub fn create_video_audio(&self, video_id: u64, language: &str) -> Result<(), rusqlite::Error>{
        self.conn.execute(
            "INSERT INTO audios (
                AudioVideoID,
                AudioLanguage) values (?1, ?2)",
            &[&video_id.to_string(), language],
        )?;
        Ok(())
    }

    pub fn edit_video_media_id(&self, video_id: u64, media_id: u64) -> Result<(), rusqlite::Error>{
        self.conn.execute(
            "UPDATE videos SET VideoMediaID = ?1 WHERE VideoID = ?2",
            &[
                &media_id.to_string(),
                &video_id.to_string()],
        )?;
        Ok(())
    }

    pub fn edit_video_path(&self, video_id: u64, path: &str) -> Result<(), rusqlite::Error>{
        self.conn.execute(
            "UPDATE videos SET VideoMediaID = ?1 WHERE VideoID = ?2",
            &[
                path,
                &video_id.to_string()],
        )?;
        Ok(())
    }


    pub fn edit_last_time(&self, video_id: u64, user_id: u64, last_time: u64) -> Result<(), rusqlite::Error>{
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

    pub fn get_video_id(&self, path: &str) -> Result<u64, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT VideoID from videos
             WHERE path = ?1",
        )?;
    
        let rows = stmt.query_map(&[path], |row| row.get(0))?;
        for row in rows{
            return Ok(row?)
        }
        Err(rusqlite::Error::QueryReturnedNoRows)
    }

    pub fn get_video_media_type(&self, video_id: u64) -> Result<Option<u8>, rusqlite::Error>{
        let mut stmt = self.conn.prepare(
            "SELECT MediaType from videos
             WHERE VideoID = ?1",
        )?;
    
        let rows = stmt.query_map(&[&video_id.to_string()], |row| row.get(0))?;
        for row in rows{
            return Ok(Some(row?))
        }
        Ok(None)
    }
    

    pub fn get_video_unknown(&self) -> Result<Vec<(u64, u8, String)>, rusqlite::Error>{
        let mut stmt = self.conn.prepare(
            "SELECT VideoID, VideoMediaType, Path from videos
             WHERE VideoMediaID = NULL",
        )?;
    
        let rows = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;

        let mut ret = Vec::new();
        for row in rows{
            ret.push(row?);
        }
        Ok(ret)
    }
    


}