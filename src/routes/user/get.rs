use actix_web::{Error, HttpRequest, Responder};
use serde_json::json;
use serde_json::Value;

use crate::{
    structures::{Base, Session, User},
    utils::Ref,
};

#[get("/get")]
pub async fn get_user(req: HttpRequest) -> Result<impl Responder, Error> {
    let session_id = req
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap();
    println!("{session_id}");
    let session = Session::find_one("token = $1", vec![session_id])
        .await
        .unwrap();

    let User {
        display_name,
        id,
        username,
        role,
        ..
    } = User::find_one("id = $1", vec![session.user_id])
        .await
        .unwrap();

    Ok(actix_web::web::Json(json!({
        "display_name": display_name,
        "id": id,
        "username": username,
        "role": role
    })))
}
