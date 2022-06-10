use rocket::serde::{Deserialize, Serialize};


pub struct Musics {
    pub id: i32,
    pub name: String,
    pub classification: i32,
    pub link: String
}
pub struct MusicNavs {
    pub id: i32,
    pub name: String,
    pub state:i32
}