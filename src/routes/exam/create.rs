use crate::structures::*;
use actix_web::Error;

#[derive(Deserialize)]
struct CreateExamRequest {
    name: String,
    outof: i8,
}

#[post("/")]
async fn create_exam(req_body: String) -> Result<String, Error> {
    let json: CreateExamRequest = serde_json::from_str(&req_body)?;
    let exam = Exam::new(json.name, json.outof);
    exam.insert()
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Something went wrong."))?;
	Ok(exam.id.to_string())
}
