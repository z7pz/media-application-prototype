pub mod edit;
pub use edit::*;
pub mod create;
pub use create::*;
pub mod get;
pub use get::*;
pub mod grades;

use crate::middlewares;
use crate::prelude::Error;
use actix_web::{
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse, Transform},
    *,
};
use actix_web_lab::middleware::from_fn;

fn exams() -> Scope {
    web::scope("")
        .service(get_all)
        .service(create)
        .service(edit)
}
fn grades() -> Scope {
    web::scope("/grades")
        .service(grades::add)
        .service(grades::delete)
        .service(grades::get_all)
        .service(grades::edit)
}

pub fn init() -> actix_web::Scope<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<impl MessageBody>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let f = from_fn(middlewares::authorization);
    web::scope("/exams")
        .wrap(f)
        .service(self::exams())
        .service(self::grades())
}
