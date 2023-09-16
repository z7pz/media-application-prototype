use actix_web::{Error, Responder};
use serde::Deserialize;

use crate::{
    structures::{Base, Exam},
    utils::snowflake::Snowflake,
};
#[derive(Deserialize)]
struct EditGrade {
    pub user_id: Snowflake,
    pub grade: i32,
    pub paper: String,
}

#[derive(Deserialize)]
struct EditRequest {
    id: Snowflake,
    name: String,
    outof: i32,
}

#[post("/edit")]
async fn edit(req_body: String) -> Result<impl Responder, Error> {
    let EditRequest { id, name, outof } = serde_json::from_str(&req_body)?;
    let mut exam = Exam::find_by_id(id).await.expect("something went wrong");
    exam.name = name;
    exam.outof = outof;
    exam.update().await.expect("something went wrong");
    Ok(actix_web::web::Json(exam))
}
