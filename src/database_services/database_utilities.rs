use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use chrono::DateTime;
use chrono::{format::strftime, TimeZone, Utc};
use derive_more::derive::{Display, Error};
use rusqlite::functions::FunctionFlags;
use rusqlite::Connection;
use std::{env, error::Error, sync::Arc};

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
pub fn get_connection() -> Result<rusqlite::Connection, ServiceError> {
    let db_path = env::var("DB_PATH").unwrap_or("kellum_library.db".to_string());
    let conn = match Connection::open(db_path) {
        Ok(conn) => Ok(conn),
        Err(_) => Err(ServiceError::ConnectionFailure),
    }?;
    add_auth_functions(&conn)?;
    Ok(conn)
}

type BoxedError = Box<dyn Error + Send + Sync + 'static>;

fn add_auth_functions(conn: &Connection) -> Result<(), ServiceError> {
    conn.create_scalar_function(
        "is_expired",
        1,
        FunctionFlags::SQLITE_DETERMINISTIC | FunctionFlags::SQLITE_UTF8,
        move |ctx| {
            let date: Arc<DateTime<Utc>> = ctx.get_or_create_aux(0, |value| {
                Ok(DateTime::parse_from_str(value.as_str()?, "")?)
            })?;
            return Ok(true);
        },
    )
    .or(Err(ServiceError::ConnectionFailure))
}
