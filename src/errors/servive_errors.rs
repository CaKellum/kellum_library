use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::derive::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ServiceError {
    #[display("failed to connect to DB.")]
    ConnectionFailure,
    #[display("Failed to find a game for specified id")]
    GameNotFound,
    #[display("Failed to find a movie for specified id")]
    MovieNotFound,
    #[display("Failed to make new game")]
    FailedToMakeGame,
    #[display("Failed to make new movie")]
    FailedToMakeMovie,
    #[display("Failed to update game")]
    FailedToUpdateGame,
    #[display("Failed to update movie")]
    FailedToUpdateMovie,
}

impl error::ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        return HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(self.to_string());
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        return match self {
            Self::ConnectionFailure => StatusCode::INTERNAL_SERVER_ERROR,
            Self::GameNotFound => StatusCode::NOT_FOUND,
            Self::MovieNotFound => StatusCode::NOT_FOUND,
            Self::FailedToMakeGame => StatusCode::IM_A_TEAPOT,
            Self::FailedToMakeMovie => StatusCode::IM_A_TEAPOT,
            Self::FailedToUpdateGame => StatusCode::INTERNAL_SERVER_ERROR,
            Self::FailedToUpdateMovie => StatusCode::INTERNAL_SERVER_ERROR,
        };
    }
}
