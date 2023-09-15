use actix_web::{Error, HttpRequest, Responder};
use serde_json::json;

use crate::{
    structures::{Base, Session, User},
    utils::snowflake::Ref,
};

#[get("/@me")]
pub async fn me(req: HttpRequest) -> Result<impl Responder, Error> {
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

        let grades = id.grades_by_userid().await.unwrap_or(Default::default());

    Ok(actix_web::web::Json(json!({
        "display_name": display_name,
        "id": id,
        "username": username,
        "role": role,
        "grades": grades
    })))
}
