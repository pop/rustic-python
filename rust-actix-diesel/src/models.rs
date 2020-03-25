use super::schema::gifs;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize, Debug)]
pub struct Gif {
    pub id: i32,
    pub url: String,
}

#[derive(Insertable)]
#[table_name = "gifs"]
pub struct NewGif<'a> {
    pub url: &'a str,
}
