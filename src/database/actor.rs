use super::SqlLibrary;


impl SqlLibrary{
    pub fn create_actor(&self, actor_id: i64, birthday: &str, deathday: &str, gender: u8, 
        name: &str, place_of_birth: &str, profile_path: &str) -> Result<(), rusqlite::Error>{

        self.conn.execute(
            "INSERT INTO actors (
                ActorID NTEGER PRIMARY KEY NOT NULL,
                Birthday  TEXT,
                Deathday  TEXT,
                Gender  INTEGER,
                Name  TEXT,
                PlaceOfBirth  TEXT,
                ProfilePath  TEXT) values (?1, ?2, ?3, ?4, ?5, ?6, ?7)",

            &[
            &actor_id.to_string(),
            birthday,
            deathday,
            &gender.to_string(),
            name,
            place_of_birth,
            profile_path],
        )?;

        Ok(())
    }
}