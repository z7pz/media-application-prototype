mod get;
pub use get::*;

use actix_web::{web::*, Scope};
pub fn init() -> Scope {
    scope("/users").service(me) // get::me
}
