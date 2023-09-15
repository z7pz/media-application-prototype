use actix_web::{Error, Responder};

use crate::structures::{Base, Exam};

#[get("/all")]
pub async fn get_all() -> Result<impl Responder, Error> {
    let exams = Exam::find_all()
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Something went worng."))?;
    Ok(actix_web::web::Json(exams))
}
