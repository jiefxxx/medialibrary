mod database;

use pyo3::prelude::*;
use database::SqlLibrary;

#[pyclass]
struct Library {
    conn: SqlLibrary,
}

#[pymethods]
impl Library {
    #[new]
    fn new(database_path: &str) -> Self {
        let conn = SqlLibrary::new(database_path).unwrap();
        conn.init_db().unwrap();
        Library{ conn}
    }

    pub fn create_video(&self, path: &str, media_type: u8, duration: f32, bit_rate: f32, 
        codec: &str, width: u32, height: u32, size: usize,
        subtitles: Vec<&str>, audios: Vec<&str>) -> PyResult<i64> {

        let video_id = self.conn.create_video(path, 
            media_type, 
            duration, 
            bit_rate, 
            codec, 
            width, 
            height, 
            size).unwrap();
        
        for language in subtitles{
            self.conn.create_video_subtitle(video_id, language).unwrap();
        }

        for language in audios{
            self.conn.create_video_audio(video_id, language).unwrap();
        }

        Ok(video_id)
    }

    pub fn edit_video_media_id(&self, video_id: i64, media_id :i64){
        self.conn.edit_video_media_id(video_id, media_id).unwrap();
    }

    pub fn get_video_id(&self, path: &str) ->PyResult<i64>{
        Ok(self.conn.get_video_id(path).unwrap())
    }

}

#[pyfunction]
fn say_hello(path: &str) {
    println!("Hello world! {:?}", path)
}

#[pymodule]
fn medialibrary(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(say_hello, module)?)?;
    module.add_class::<Library>()?;
    Ok(())
}
