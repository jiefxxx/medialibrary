use std::collections::HashMap;

use rusqlite::ToSql;

use super::{SqlLibrary, parse_concat};

use crate::library::video::{MediaInfo, Video, VideoResult, EpisodeMinimal, MovieMinimal};

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
            &video.codec.as_ref().unwrap_or(&"".to_string()),
            &video.width.to_string(),
            &video.height.to_string(),
            &video.size.to_string()],
        )?;

        let video_id = self.conn.last_insert_rowid() as u64;
        for language in video.subtitles{
            self.conn.execute(
                "INSERT OR IGNORE INTO Subtitles (
                    video_id,
                    language) values (?1, ?2)",
                &[&video_id.to_string(), &language],
            )?;
        }

        for language in video.audios{
            self.conn.execute(
                "INSERT OR IGNORE INTO Audios (
                    video_id,
                    language) values (?1, ?2)",
                &[&video_id.to_string(), &language],
            )?;
        }

        Ok(video_id)
    }

    pub fn get_video(&self,video_id: u64) -> Result<Option<Video>, rusqlite::Error>{
        let sql = "SELECT
                            id,
                            path,
                            media_type,
                            media_id,
                            duration,
                            bit_rate,
                            codec,
                            width ,
                            height,
                            size,
                            adding,
                            subtitles,
                            audios, 
                            m_title, 
                            t_title, 
                            release_date, 
                            episode_number, 
                            season_number FROM VideosView
                        WHERE id = ?1";
        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map(&[&video_id.to_string()], |row| {
            let media_id: Option<u64> = row.get(4)?;
            let info = match media_id{
                None => MediaInfo::Unknown,
                Some(media_id) => match row.get(3)?{
                    0 => MediaInfo::Movie(MovieMinimal{
                        id: media_id,
                        title: row.get(14)?,
                        release_date: row.get(16)?,
                    }),
                    1 => MediaInfo::Tv(EpisodeMinimal{
                        id: media_id,
                        title: row.get(15)?,
                        season_number: row.get(17)?,
                        episode_number: row.get(18)?,
                    }),
                    _ => MediaInfo::Unknown,
                }
            };
            Ok(Video{
                id: row.get(1)?,
                path:row.get(2)?,
                media_type: row.get(3)?,
                media_id,
                bit_rate: row.get(6)?,
                duration: row.get(5)?,
                size: row.get(10)?,
                adding: row.get(11)?,
                codec: row.get(7)?,
                width: row.get(8)?,
                height: row.get(9)?,
                subtitles: parse_concat(row.get(12)?).unwrap_or_default(),
                audios: parse_concat(row.get(13)?).unwrap_or_default(),
                info,
            })
        })?;

        for row in rows{
            return Ok(Some(row?));
        }

        Ok(None)
    }

    pub fn get_videos(&self, parameters: HashMap<&str, Option<(String, String)>>) -> Result<Vec<VideoResult>, rusqlite::Error>{
        let mut param: Vec<&dyn ToSql> = Vec::new();
        let mut sql = String::new();
        sql += "SELECT id, path, media_type, media_id, adding, m_title, t_title, release_date, episode_number, season_number FROM VideosView ";
        if parameters.len() > 0{
            sql += "WHERE ";
            let mut counter = 1;
            for (name, value) in &parameters{
                if counter > 1{
                    sql += "AND "
                }
                
                if let Some((operator, value)) = value{
                    param.push(value);
                    sql += &format!("{} {} ?{} ", name, operator, &counter);
                    counter += 1;
                }
                else{
                    sql += &format!("{} IS NULL ", name);
                }
                
            }
        }

        let mut stmt = self.conn.prepare(&sql)?;
    
        let rows = stmt.query_map(param.as_slice(), |row| {
            let media_id: Option<u64> = row.get(3)?;
            let info = match media_id{
                None => MediaInfo::Unknown,
                Some(media_id) => match row.get(2)?{
                    0 => MediaInfo::Movie(MovieMinimal{
                        id: media_id,
                        title: row.get(5)?,
                        release_date: row.get(7)?,
                    }),
                    1 => MediaInfo::Tv(EpisodeMinimal{
                        id: media_id,
                        title: row.get(6)?,
                        season_number: row.get(8)?,
                        episode_number: row.get(9)?,
                    }),
                    _ => MediaInfo::Unknown,
                }
            };
            Ok(VideoResult{
                id: row.get(0)?,
                path: row.get(1)?,
                media_type: row.get(2)?,
                adding: row.get(4)?,
                info,
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