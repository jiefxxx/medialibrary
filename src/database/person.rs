use crate::rustmdb::Person;

use super::SqlLibrary;


impl SqlLibrary{

    pub fn create_person(&mut self, person: &Person) -> Result<(Vec<u64>, Vec<String>), rusqlite::Error>{

        let tx = self.conn.transaction()?;

        let person_ids = Vec::new();
        let mut rsc_path = Vec::new();

        tx.execute(
            "INSERT INTO Persons (
                id,
                birthday,
                known_for_department,
                deathday,
                name,
                gender,
                biography,
                popularity,
                place_of_birth,
                profile_path) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",

            &[
            &person.id.to_string(),
            &person.birthday.as_ref().unwrap_or(&"".to_string()),
            &person.known_for_department.as_ref().unwrap_or(&"".to_string()),
            &person.deathday.as_ref().unwrap_or(&"".to_string()),
            &person.name,
            &person.gender.to_string(),
            &person.biography,
            &person.popularity.to_string(),
            &person.place_of_birth.as_ref().unwrap_or(&"".to_string()),
            &person.profile_path.as_ref().unwrap_or(&"".to_string())],
        )?;

        if let Some(profile_path) = &person.profile_path{
            rsc_path.push(profile_path.clone())
        }

        tx.commit()?;

        Ok((person_ids, rsc_path))
    }

    pub fn person_exist(&self, person_id: u64) -> Result<bool, rusqlite::Error>{
        let mut stmt = self.conn.prepare(
            "SELECT name from Persons
             WHERE id = ?1",
        )?;
    
        let rows = stmt.query_map(&[&person_id.to_string()], |row| row.get(0))?;
        for row in rows{
            let _unused: String = row?;
            return Ok(true)
        }
        Ok(false)
    }
}