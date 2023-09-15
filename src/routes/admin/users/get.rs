use actix_web::web::Json;
use actix_web::{Error, Responder};
use serde_json::json;
use serde_json::Value;

use crate::structures::{Base, User};

#[get("/all")]
async fn all() -> Result<impl Responder, Error> {
    let users = User::find_all().await.map_err(|_| {
        actix_web::error::ErrorInternalServerError("Something went wrong while getting users data.")
    })?;

    let vec: Vec<Value> = users
        .iter()
        .map(|c| {
            json!({
                "id": c.id,
                "username": c.username,
                "display_name": c.display_name,
                "role": c.role
            })
        })
        .collect();

    Ok(Json(vec))
}
