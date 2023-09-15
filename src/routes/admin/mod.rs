mod users;
use actix_web::{web::*, Scope};
pub fn init() -> Scope {
    scope("admin").service(users::init())
}
