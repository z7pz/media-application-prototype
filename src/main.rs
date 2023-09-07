#![feature(io_error_other)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_with;
#[macro_use]
extern crate actix_web;

use actix_web_lab::middleware::{from_fn, Next};

use actix_web::{
    body::MessageBody,
    dev::{Service, ServiceRequest, ServiceResponse},
    web, App, Error, HttpServer,
};
use sqlx::{Pool, Postgres};
use structures::Session;

use crate::structures::Base;

mod appstate;
mod routes;
mod structures;
mod utils;
static POOL: std::sync::OnceLock<Pool<Postgres>> = std::sync::OnceLock::new();

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let options = sqlx::postgres::PgPoolOptions::new()
        .max_lifetime(None)
        .idle_timeout(None);
    let pool = options
        .connect("postgres://postgres:postgres@localhost/idk")
        .await
        .expect("couldn't connect to database!");
    sqlx::migrate!("db/migrations")
        .run(&pool)
        .await
        .expect("Failed to run the migration");
    POOL.set(pool).expect("couldn't asign the pool to global");
    println!("server is running!");
    use routes::*;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(appstate::new()))
            .service(web::scope("/auth").service(login).service(register))
            .service(
                web::scope("/exams")
                    .wrap(from_fn(authorization))
                    .service(create_exam)
                    .service(get_exams)
                    .service(add_grade),
            )
            .service(
                web::scope("/user")
                .wrap(from_fn(authorization))
                .service(get_user)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn authorization(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth = req.headers().get("Authorization");
    if let Some(session) = auth.map(|c| c.to_str().unwrap()) {
        Session::find_one("token = $1", vec![session])
            .await
            .map_err(|_| actix_web::error::ErrorUnauthorized("Unauthorized."))?;
        let res = next.call(req).await?;
        return Ok(res);
    }
    Err(actix_web::error::ErrorUnauthorized("Unauthorized."))
}
