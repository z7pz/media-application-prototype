use actix_web::Error;

use crate::structures::{Base, Session, User};

#[derive(Deserialize)]
struct LoginReq {
    username: String,
    password: String,
}

#[post("/login")]
async fn login(req_body: String) -> Result<String, Error> {
    let json: LoginReq = serde_json::from_str(&req_body)?;
    let user = User::find_one("username = $1", vec![json.username])
        .await
        .map_err(|_| actix_web::error::ErrorUnauthorized("Unauthorized"))?;
    if user.password_hash == json.password {
        let session = Session::new(user.id);
        session
            .insert()
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("Something went worng!"))?;
        return Ok(session.token);
    }
    Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
}
