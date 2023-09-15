use actix_web::{Error, HttpRequest, Responder};

use crate::{
    structures::{Base, Exam, Session, User, UserRoles},
    utils::snowflake::Snowflake,
};

#[derive(Deserialize)]
struct AddGradeReq {
    exam_id: Snowflake,
    grade_id: Snowflake,
}

#[delete("/delete")]
async fn delete(req_body: String, req: HttpRequest) -> Result<impl Responder, Error> {
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
            let mut exam = Exam::find_by_id(json.exam_id)
                .await
                .map_err(|_| actix_web::error::ErrorNotFound("Exam not found."))?;
            exam.grades.retain_mut(|&mut x| x != json.grade_id);
            exam.update().await.map_err(|_| {
                actix_web::error::ErrorInternalServerError("Exam couldn't be updated.")
            })?;
            Ok(actix_web::web::Json(exam))
        }
        _ => Err(actix_web::error::ErrorForbidden("Access denied.")),
    }
}
