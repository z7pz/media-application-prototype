mod login;
use login::*;
mod register;
use register::*;

use actix_web::{web::*, Scope};

pub fn init() -> Scope {
    scope("/auth").service(login).service(register)
}
