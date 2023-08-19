use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::basic::BasicAuth;

pub async fn validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    if credentials.user_id() == "user" && credentials.password() == Some("password") {
        Ok(req)
    } else {
       Err((Error::from(actix_web::error::ErrorUnauthorized("Unauthorized")), req))
    }
}
