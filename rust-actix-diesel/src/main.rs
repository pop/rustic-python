#[macro_use]
extern crate diesel;
extern crate diesel_migrations;
extern crate dotenv;

extern crate actix;
pub mod schema;
pub mod models;
pub mod state;

use std::sync::Mutex;

use actix_web::{web, App, Error, HttpServer, HttpRequest, HttpResponse};

use crate::state::{State, DbStatePool};
use crate::models::Gif;


async fn index(
    path: web::Path<String>,
    db: web::Data<DbStatePool>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(Gif { id: 1, url: String::from("bogus") }))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let state = State::new();

    HttpServer::new(move || {
        App::new()
            .data(state.pool.clone())
            .service(
                web::resource("/{thing}").route(web::get().to(index)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
