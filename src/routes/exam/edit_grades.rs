use actix_web::{Error, Responder};

use crate::{
    structures::{Base, Exam, Grade},
    utils::{Ref, Snowflake},
};

#[derive(Deserialize)]
struct GradeRequest {
    id: Snowflake,
    grade: i32,
    paper: String,
}

#[derive(Deserialize)]
struct EditGradeRequest {
    grades: Vec<GradeRequest>,
    exam_id: Snowflake,
}

#[post("/edit_grades")]
async fn edit_grades(req_body: String) -> Result<impl Responder, Error> {
    let json: EditGradeRequest = serde_json::from_str(&req_body)?;
    let mut grades = json
        .exam_id
        .grades_by_exam()
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("couldn't fetch grades"))?;
    let clone = &grades.clone();
    let filtered = clone
        .iter()
        .filter(|grade| !json.grades.iter().any(|c| c.id == grade.id));
    for grade in filtered.clone() {
        grade.clone().delete().await.map_err(|_| {
            actix_web::error::ErrorInternalServerError(format!(
                "error while deleting {}",
                *grade.id
            ))
        })?;
    }
    grades.retain(|grade| json.grades.iter().any(|c| c.id == grade.id));
    for gradee in grades.iter_mut() {
        let GradeRequest { grade, paper, .. } =
            json.grades.iter().find(|c| c.id == gradee.id).unwrap();
        gradee.grade = *grade;
        gradee.paper = paper.clone();
        gradee.update().await.map_err(|_| {
            actix_web::error::ErrorInternalServerError(format!("{} couldn't save", *gradee.id))
        })?
    }

    Ok(actix_web::web::Json(grades))
}
