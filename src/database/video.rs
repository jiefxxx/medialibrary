use std::collections::HashMap;

use rusqlite::{ToSql, types::Null};

use super::SqlLibrary;

use crate::library::video::{Video, VideoResult};

impl SqlLibrary{

    pub fn create_video(&self, video: Video) -> Result<u64, rusqlite::Error>{
        self.conn.execute(
            "INSERT INTO Videos (
                path,
                media_type,
                duration,
                bit_rate,
                codec,
                width,
                height,
                size,
                adding) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, datetime('now'))",
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
                "INSERT INTO Subtitles (
                    video_id,
                    language) values (?1, ?2)",
                &[&video_id.to_string(), &language],
            )?;
        }

        for language in video.audios{
            self.conn.execute(
                "INSERT INTO Audios (
                    video_id,
                    language) values (?1, ?2)",
                &[&video_id.to_string(), &language],
            )?;
        }

        Ok(video_id)
    }

    pub fn get_videos(&self, parameters: HashMap<&str, Option<(String, String)>>) -> Result<Vec<VideoResult>, rusqlite::Error>{
        let mut param: Vec<&dyn ToSql> = Vec::new();
        let mut sql = String::new();
        sql += "SELECT id, path, media_type, media_id, adding FROM Videos ";
        if parameters.len() > 0{
            sql += "WHERE ";
            let mut counter = 1;
            for (name, value) in &parameters{
                if counter > 1{
                    sql += "AND "
                }
                
                if let Some((operator, value)) = value{
                    println!("param {} ", &value);
                    param.push(value);
                    sql += &format!("{} {} ?{} ", name, operator, &counter);
                    counter += 1;
                }
                else{
                    sql += &format!("{} IS NULL ", name);
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
            "UPDATE Videos SET media_id = ?1 WHERE id = ?2",
            &[
                &media_id.to_string(),
                &video_id.to_string()],
        )?;
        Ok(())
    }

    pub fn edit_video_path(&self, video_id: u64, path: &str) -> Result<(), rusqlite::Error>{
        self.conn.execute(
            "UPDATE Videos SET path = ?1 WHERE id = ?2",
            &[
                path,
                &video_id.to_string()],
        )?;
        Ok(())
    }


    pub fn edit_last_time(&self, video_id: u64, user_id: u64, last_time: u64) -> Result<(), rusqlite::Error>{
        self.conn.execute(
            "INSERT OR REPLACE INTO LastTime (
                video_id,
                user_id,
                last_time) values (?1, ?2, ?3)",
            &[
                &video_id.to_string(),
                &user_id.to_string(),
                &last_time.to_string()],
        )?;
        Ok(())
    }

    pub fn get_video_id(&self, path: &str) -> Result<u64, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id from Videos
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
            "SELECT media_type from Videos
             WHERE id = ?1",
        )?;
    
        let rows = stmt.query_map(&[&video_id.to_string()], |row| row.get(0))?;
        for row in rows{
            return Ok(Some(row?))
        }
        Ok(None)
    }
}