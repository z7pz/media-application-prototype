#![feature(io_error_other)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_with;

use sqlx::{Pool, Postgres,error::Error as sqlxError};
use actix_web::{post, web, App, Error, HttpServer};
use serde::Deserialize;
use structures::{Base, User, Session};

mod structures;
mod utils;

static POOL: std::sync::OnceLock<Pool<Postgres>> = std::sync::OnceLock::new();

#[derive(Deserialize)]
struct RegisterReq {
    display_name: String,
    username: String,
    password: String,
}

#[post("/register")]
async fn register(req_body: String) -> Result<String, Error> {
    let json: RegisterReq = serde_json::from_str(&req_body)?;
    let user = User::new(json.display_name, json.username, json.password);
    let res = user.insert().await;
    if let Err(err) = res {
        match err {
            sqlxError::Database(err) if err.code().unwrap_or_default() == "23505" => {
                return Err(actix_web::error::ErrorBadRequest("User already exists."))
            }
            _ => {}
        }
        return Err(actix_web::error::ErrorBadRequest("Something went wrong!"));
    }
    Ok("User created!".into())
}

#[derive(Deserialize)]
struct LoginReq {
    username: String,
    password: String,
}

#[post("/login")]
async fn login(req_body: String) -> Result<String, Error> {
    let json: LoginReq = serde_json::from_str(&req_body)?;
    let user = User::find_one("username = $1", vec![json.username]).await.map_err(|_| actix_web::error::ErrorUnauthorized("Unauthorized"))?;
    if user.password_hash == json.password {
        let session = Session::new(user.id);
        return Ok(session.token);
    }
    Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
}

struct AppState {}
//
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
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {}))
            .service(register)
            .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
