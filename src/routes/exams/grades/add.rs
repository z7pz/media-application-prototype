use actix_web::{Error, HttpRequest, Responder};

use crate::{
    structures::{Base, Exam, Grade, Session, User, UserRoles},
    utils::snowflake::Snowflake,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct AddGradeReq {
    exam_id: Snowflake,
    user_id: Snowflake,
    mark: i32,
    // paper: String,
}

#[post("/add")]
async fn add(req_body: String, req: HttpRequest) -> Result<impl Responder, Error> {
    let json: AddGradeReq = serde_json::from_str(&req_body)?;
    let session_id = req
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap();
    let session = Session::find_one("token = $1", vec![session_id])
        .await
        .unwrap();
    let user = User::find_one("id = $1", vec![session.user_id])
        .await
        .unwrap();
    match user.role {
        UserRoles::Admin | UserRoles::Teacher => {
            let mut exam = Exam::find_one("id = $1", vec![json.exam_id])
                .await
                .map_err(|_| actix_web::error::ErrorNotFound("Exam not found"))?;
            let grade = Grade::new(json.user_id, exam.id, json.mark);
            grade.insert().await.unwrap();
            exam.grades.push(grade.id);
            println!("updating...");
            exam.update().await.unwrap();
            Ok(actix_web::web::Json(grade))
        }
        _ => Err(actix_web::error::ErrorForbidden("Access denied.")),
    }
}
