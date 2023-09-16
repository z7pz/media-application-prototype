#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_with;
#[macro_use]
extern crate actix_web;

use actix_web::{web, App, HttpServer};
use sqlx::{Pool, Postgres};

pub mod appstate;
pub mod config;
pub mod database;
pub mod middlewares;
pub mod routes;
pub mod structures;
pub mod utils;
pub use config::*;
mod error;
mod prelude;

pub use prelude::*;
static POOL: std::sync::OnceLock<Pool<Postgres>> = std::sync::OnceLock::new();

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    database::init_db().await;
    HttpServer::new(move || {
        use routes::*;
        App::new()
            .app_data(web::Data::new(appstate::new()))
            .service(auth::init())
            .service(exams::init())
            .service(user::init())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}
