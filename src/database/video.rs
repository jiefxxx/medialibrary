use std::collections::HashMap;

use rusqlite::{ToSql, types::Null};

use super::SqlLibrary;

use crate::library::video::{Video, VideoResult};

impl SqlLibrary{

    pub fn create_video(&self, video: Video) -> Result<u64, rusqlite::Error>{
        self.conn.execute(
            "INSERT INTO videos (
                Path,
                MediaType,
                Duration,
                BitRate,
                Codec,
                Width,
                Height,
                Size,
                Adding) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, datetime('now'))",
            &[&video.path, 
            &video.media_type.to_string(),
            &video.duration.to_string(),
            &video.bit_rate.to_string(),
            &video.codec,
            &video.width.to_string(),
            &video.height.to_string(),
            &video.size.to_string()],
        )?;

        let video_id = self.conn.last_insert_rowid() as u64;
        for language in video.subtitles{
            self.conn.execute(
                "INSERT INTO subtitles (
                    SubtitleVideoID,
                    SubtitleLanguage) values (?1, ?2)",
                &[&video_id.to_string(), &language],
            )?;
        }

        for language in video.audios{
            self.conn.execute(
                "INSERT INTO audios (
                    AudioVideoID,
                    AudioLanguage) values (?1, ?2)",
                &[&video_id.to_string(), &language],
            )?;
        }

        Ok(video_id)
    }

    pub fn get_videos(&self, parameters: HashMap<&str, Option<String>>) -> Result<Vec<VideoResult>, rusqlite::Error>{
        let mut param: Vec<&dyn ToSql> = Vec::new();
        let mut sql = String::new();
        sql += "SELECT VideoID, Path, MediaType, MediaID, Adding FROM videos ";
        if parameters.len() > 0{
            sql += "WHERE ";
            let mut counter = 1;
            for (name, value) in &parameters{
                    if counter > 1{
                        sql += "AND "
                    }
                sql += name;
                
                if let Some(value) = value{
                    println!("param {} ", &value);
                    param.push(value);
                    sql += " = ?";
                    sql += &counter.to_string();
                    sql += " ";
                    counter += 1;
                }
                else{
                    sql += " IS NULL ";
                    println!("param NULL");
                }
                
            }
        }
        println!("sql {} ", &sql);
        let mut stmt = self.conn.prepare(&sql)?;
    
        let rows = stmt.query_map(param.as_slice(), |row| {
            Ok(VideoResult{
                id: row.get(0)?,
                path: row.get(1)?,
                media_type: row.get(2)?,
                media_id: row.get(3)?,
                adding: row.get(4)?,
            })
        })?;

        let mut result = Vec::new();
        for row in rows{
            result.push(row?);
        }
        Ok(result)
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