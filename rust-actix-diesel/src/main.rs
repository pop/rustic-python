#[macro_use]
extern crate diesel;
extern crate diesel_migrations;
extern crate dotenv;

extern crate actix;
pub mod schema;
pub mod models;
pub mod state;

use actix_web::{web, guard, App, Error, HttpServer, HttpResponse};

use crate::state::State;
use crate::models::GifForm;


async fn get_all_gif(
    db: web::Data<State>,
) -> Result<HttpResponse, Error> {

    let gifs = web::block(move || {
       db.get_all_gifs()
    }).await.unwrap(); // FIXME

    Ok(HttpResponse::Ok().json(gifs))
}

async fn post_gif(
    db: web::Data<State>,
    new_gif: web::Json<GifForm>
) -> Result<HttpResponse, Error> {

    let gif = web::block(move || {
       db.create_gif(&new_gif.url)
    }).await.unwrap(); // FIXME

    Ok(HttpResponse::Ok().json(gif))
}

async fn get_gif(
    path: web::Path<String>,
    db: web::Data<State>,
) -> Result<HttpResponse, Error> {
    let gif_id: i32 = path.parse().unwrap(); // FIXME
    println!("{:?}", gif_id);

    let gif = web::block(move || {
        db.get_gif(gif_id)
    }).await.unwrap(); // FIXME

    Ok(HttpResponse::Ok().json(gif))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let state = State::new();

    state.run_migrations().unwrap(); // FIXME

    let serving_on = "127.0.0.1:5000";
    println!("Serving on {}", serving_on);

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
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
    })
    .bind(serving_on)?
    .run()
    .await
}
