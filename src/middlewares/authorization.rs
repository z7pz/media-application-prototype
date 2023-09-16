use crate::prelude::*;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
};
use actix_web_lab::middleware::Next;

use crate::structures::{Base, Session};

pub async fn authorization(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::error::Error> {
    let auth = req.headers().get("Authorization");
    if let Some(session) = auth.map(|c| c.to_str().unwrap()) {
        Session::find_one("token = $1", vec![session])
            .await
            .map_err(|_| actix_web::error::ErrorUnauthorized("Unauthorized."))?;
        let res = next.call(req).await?;
        return Ok(res);
    }
    Err(actix_web::error::ErrorUnauthorized("Unauthorized."))
}
