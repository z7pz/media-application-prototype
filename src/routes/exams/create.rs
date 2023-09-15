use crate::structures::*;
use actix_web::Error;

#[derive(Deserialize)]
struct CreateExamRequest {
    name: String,
    outof: i32,
}

#[post("/create")]
async fn create(req_body: String) -> Result<String, Error> {
    let json: CreateExamRequest = serde_json::from_str(&req_body)?;
    let exam = Exam::new(json.name, json.outof);
    exam.insert()
        .await
        .map_err(|res| actix_web::error::ErrorInternalServerError(res))?;
	Ok(exam.id.to_string())
}
