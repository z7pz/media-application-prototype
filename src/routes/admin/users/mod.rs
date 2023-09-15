mod get;
use actix_web::{web::*, Scope};
use get::*;

pub fn init() -> Scope {
    scope("/users").service(all)
}
