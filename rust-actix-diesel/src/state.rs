use diesel::prelude::*;
use diesel_migrations::{run_pending_migrations, RunMigrationsError};
use dotenv::dotenv;
use std::env;

use crate::models::{NewGif, Gif};
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::r2d2::PooledConnection;
use r2d2::Error;

pub type DbStatePool = Pool<ConnectionManager<SqliteConnection>>;

#[derive(Clone)]
pub struct State {
    pub pool: DbStatePool,
}

impl State {
    pub fn new() -> Self {
        State {
            pool: State::establish_connection_pool().expect("Umm."),
        }
    }

    pub fn establish_connection_pool() -> Result<DbStatePool, r2d2::Error> {
        let manager = State::establish_connection_manager();
        Pool::builder().build(manager)
    }

    pub fn establish_connection_manager() -> ConnectionManager<SqliteConnection> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("Please set the DATABASE_URL env variable");

        ConnectionManager::<SqliteConnection>::new(&database_url)
    }

    pub fn run_migrations(&self) -> Result<(), RunMigrationsError> {
        run_pending_migrations(&self.pool.get().expect("Umm...")) 
    }

    pub fn create_gif(&self, url: &str) -> Result<Gif, diesel::result::Error> {
        use crate::schema::gifs::table;

        let new_gif = NewGif {
            url: url,
        };

        diesel::insert_into(table)
            .values(new_gif)
            .execute(&self.pool.get().expect("Umm..."));

        self.get_latest_gif()
    }

    pub fn get_latest_gif(&self) -> Result<Gif, diesel::result::Error> {
        use crate::schema::gifs::dsl::*;

        match gifs.filter(url.eq(&url)).load::<Gif>(&self.pool.get().expect("Umm...")) {
            Ok(mut gif) => Ok(gif.pop().expect("What?")),
            Err(e) => Err(e)
        }
    }

    pub fn get_gif(&self, gif_id: i32) -> Result<Gif, diesel::result::Error> {
        use crate::schema::gifs::dsl::*;
        
        gifs 
            .find(gif_id)
            .first::<Gif>(&self.pool.get().expect("Umm..."))
    }

    pub fn get_all_gifs(&self) -> Result<Vec<Gif>, diesel::result::Error> {
        use crate::schema::gifs::dsl::*;
        
        gifs 
            .load::<Gif>(&self.pool.get().expect("Umm..."))
    }
}
