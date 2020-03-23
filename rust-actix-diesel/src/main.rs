#[macro_use]
extern crate diesel;

extern crate diesel_migrations;

extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel_migrations::{run_pending_migrations, RunMigrationsError};

use dotenv::dotenv;

use std::env;

use models::{NewGif, Gif};


fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


fn run_migrations(conn: &SqliteConnection) -> Result<(), RunMigrationsError> {
    run_pending_migrations(conn) 
}


fn create_gif(conn: &SqliteConnection, url: &str) -> usize {
    use schema::gifs;

    let new_gif = NewGif {
        url: url,
    };

    diesel::insert_into(gifs::table)
        .values(new_gif)
        .execute(conn)
        .expect("Error saving new gif")
}


fn get_gif(conn: &SqliteConnection, _id: usize) -> Vec<Gif> {
    use schema::gifs::dsl::*;

    gifs 
        .filter(url.eq("http://foo.bar.baz"))
        .limit(1)
        .load::<Gif>(conn)
        .expect("Error loading gif")
}


fn main() {
    let conn = establish_connection();
    match run_migrations(&conn) {
        Err(_) => panic!("Oh no migrations failed!"),
        _ => (),
    };
    let result = create_gif(&conn, "http://foo.bar.baz");
    println!("{}", result);
    let gifs = get_gif(&conn, result);
    for gif in gifs {
        println!("{:?}", gif.url);
    }
}
