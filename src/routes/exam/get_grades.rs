use actix_web::{Error, Responder};

use rayon::prelude::*;

use crate::{
    structures::{Base, Exam, Grade},
    utils::{Snowflake, Ref},
};

#[derive(Deserialize)]
struct GetGradesRequest {
    exam_id: Snowflake,
}

#[post("/grades")]
async fn get_grades(req_body: String) -> Result<impl Responder, Error> {
    let GetGradesRequest { exam_id } = serde_json::from_str(&req_body)?;
	// checking if this exam_id is found in the database
    exam_id.exam().await.map_err(|_| actix_web::error::ErrorNotFound("Exam by this id not found."))?;
	// getting all grades by exam_id
	let grades = exam_id.grades_by_exam().await.map_err(|_| actix_web::error::ErrorInternalServerError("Something went wrong while getting grades of this exam."))?;

    Ok(actix_web::web::Json(grades))
}
