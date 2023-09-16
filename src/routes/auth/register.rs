use crate::prelude::*;
use crate::structures::{Base, User};
use serde::Deserialize;

#[derive(Deserialize)]
struct RegisterReq {
    display_name: String,
    username: String,
    password: String,
}

#[post("/register")]
async fn register(req_body: String) -> Result<String> {
    let json: RegisterReq = serde_json::from_str(&req_body)?;
    let user = User::new(json.display_name, json.username, json.password);
    let res = user.insert().await?;
//     if let Err(err) = res {
//         match err {
//             Error::Sqlx(err) => match err {
//                 sqlx::Error::Database(err) if err.code().unwrap_or_default() == "23505" => {
//                     return Err(actix_web::error::ErrorBadRequest("User already exists."))?
//                 },
//                 _ => {}
//             },
// 
//             _ => {}
//         }
//         return Err(actix_web::error::ErrorBadRequest("Something went wrong!"))?;
//     }
    Ok("User created!".into())
}
