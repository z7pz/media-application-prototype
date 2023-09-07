use actix_web::{Error, HttpRequest, Responder};

use crate::{
    structures::{Base, Session, User},
    utils::Ref,
};

#[get("/get_user")]
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

    let user = User::find_one("id = $1", vec![session.user_id]).await.unwrap();

    Ok(actix_web::web::Json(user))
}
