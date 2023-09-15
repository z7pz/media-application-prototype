mod get;
use get::*;
mod users {
    use super::*;
    use actix_web::{web::*, Scope};

    pub fn init() -> Scope {
        scope("/users").service(all)
    }
}

pub use users::*;
