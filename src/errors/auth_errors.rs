use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::derive::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum AuthServiceError {
    #[display("Failed to register user")]
    FailedToRegister,
    #[display("Failed to authenticate user")]
    FailedToAuthenticate,
    #[display("Invalid session token")]
    InvalidSessionToken,
    #[display("User is just not allowed to do this")]
    GenerallyForbiden,
    #[display("There might be an attacker in the middle")]
    SuspiciousRequest,
}

impl error::ResponseError for AuthServiceError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        return HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(self.to_string());
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::FailedToRegister => StatusCode::PRECONDITION_FAILED,
            Self::GenerallyForbiden => StatusCode::FORBIDDEN,
            Self::FailedToAuthenticate => StatusCode::UNAUTHORIZED,
            Self::SuspiciousRequest => StatusCode::UNAUTHORIZED,
            Self::InvalidSessionToken => StatusCode::UNAUTHORIZED,
        }
    }
}
