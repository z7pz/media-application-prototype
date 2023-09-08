// use actix_web::{Error, Responder};
// 
// use crate::{structures::{Exam, Base, Grade}, utils::Snowflake};
// #[derive(Deserialize)]
// struct EditGrade {
//     pub user_id: Snowflake,
// 	pub grade: i32,
// 	pub paper: String,
// }
// 
// #[derive(Deserialize)]
// struct EditRequest {
// 	id: Snowflake,
// 	name: String,
// 	outof: i32,
// 	grades: Vec<EditGrade>,
// }
// 
// #[post("/edit")]
// async fn get_exams(req_body: String) -> Result<impl Responder, Error> {
// 	let EditRequest { id, name, outof, grades } = serde_json::from_str(&req_body).unwrap();
// 	let exam = Exam::find_by_id(id).await.expect("something went wrong");
// 
// 	Ok(actix_web::web::Json(exam))
// }