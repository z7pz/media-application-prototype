use actix_web::Error;

#[get("/user")]
pub async fn get_user() -> Result<String, Error> {
    println!("test");
    Ok("test".into())
}
