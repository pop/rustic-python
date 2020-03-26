#[macro_use]
extern crate diesel;

extern crate actix;
extern crate actix_threadpool;
extern crate diesel_migrations;
extern crate dotenv;

pub mod schema;
pub mod models;
pub mod state;

use actix_web::{web, guard, App, HttpServer, HttpResponse, Result, error};
use serde::{Serialize};
use serde_json::to_string;

use crate::state::State;
use crate::models::GifForm;

#[derive(Serialize)]
struct ErrorMsg {
    error: String
}

impl ErrorMsg {
    fn new(msg: &str) -> ErrorMsg {
        ErrorMsg {
            error: String::from(msg)
        }
    }
}

#[derive(Serialize)]
struct StatusMsg {
    status: String
}

impl StatusMsg {
    fn new(msg: &str) -> StatusMsg {
        StatusMsg {
            status: String::from(msg)
        }
    }
}


async fn get_all_gif(
    db: web::Data<State>,
) -> Result<HttpResponse> {

    match web::block(
        move || {
            db.get_all_gifs()
        }
    ).await {
        Ok(gifs) => Ok(
            HttpResponse::Ok().json(
                gifs
            )
        ),
        Err(_) => Err(
            error::ErrorNotFound(
                to_string(
                    &ErrorMsg::new("Could not load all gifs" )
                ).unwrap()
            )
        ),
    }
}

async fn post_gif(
    db: web::Data<State>,
    new_gif: web::Json<GifForm>
) -> Result<HttpResponse> {

    match web::block(
        move || {
            db.create_gif(&new_gif.url)
        }
    ).await {
        Ok(gif) => Ok(
            HttpResponse::Ok().json(
                gif
            )
        ),
        Err(_) => Err(
            error::ErrorInternalServerError(
                to_string(
                    &ErrorMsg::new("Could not post gif")
                ).unwrap()
            )
        )
    }

}

async fn get_gif(
    path: web::Path<String>,
    db: web::Data<State>,
) -> Result<HttpResponse> {
    let gif_id: i32 = path.parse().unwrap(); // FIXME

    match web::block(
        move || {
            db.get_gif(gif_id)
        }
    ).await {
        Ok(gif) => Ok(
            HttpResponse::Ok().json(
                gif
            )
        ),
        Err(_) => Err(
            error::ErrorNotFound(
                to_string(
                    &ErrorMsg::new("Could not find that gif")
                ).unwrap()
            )
        )
    }
}

async fn initialize_database(
    db: web::Data<State>
) -> Result<HttpResponse> {

    match db.run_migrations() {
        Ok(_) => Ok(
            HttpResponse::Ok().json(
                StatusMsg::new("Success!")
            )
        ),
        Err(_) => Err(
            error::ErrorInternalServerError(
                to_string(
                    &ErrorMsg::new("Failed to migrate database")
                ).unwrap()
            )
        ),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let state = State::new();

    let serving_on = "127.0.0.1:5000";
    println!("Serving on {}", serving_on);

    HttpServer::new(move || {
        App::new()
            .data(
                state.clone()
            )
            .service(
                web::resource("/gif/{gif_id}/")
                    .guard(guard::Get())
                    .guard(guard::Header("Content-Type", "application/json"))
                    .to(get_gif)
            )
            .service(
                web::resource("/gif/")
                    .guard(guard::Post())
                    .guard(guard::Header("Content-Type", "application/json"))
                    .guard(guard::Header("Authorization", "cloudbolt"))
                    .to(post_gif)
            )
            .service(
                web::resource("/gif/")
                    .guard(guard::Get())
                    .guard(guard::Header("Content-Type", "application/json"))
                    .to(get_all_gif)
            )
            .service(
                web::resource("/")
                    .guard(guard::Post())
                    .guard(guard::Header("Content-Type", "application/json"))
                    .guard(guard::Header("Authorization", "cloudbolt"))
                    .to(initialize_database)
            )
            .default_service(
                web::resource("")
                    // 404 for GET request
                    .route(
                        web::route()
                            .guard(guard::Get())
                            .to(HttpResponse::NotFound),
                    )
                    .route(
                        web::route()
                            .guard(guard::Post())
                            .to(HttpResponse::NotFound),
                    )
                    // all that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .guard(guard::Not(guard::Post()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
    .bind(serving_on)?
    .run()
    .await
}
