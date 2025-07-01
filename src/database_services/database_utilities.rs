use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use chrono::{DateTime, TimeDelta, Utc};
use derive_more::derive::{Display, Error};
use rusqlite::{functions::FunctionFlags, Connection};
use std::{env, sync::Arc};

// TODO: Move to own module
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
    add_is_expired(&conn)?;
    add_get_expiry(&conn)?;
    Ok(conn)
}

fn add_is_expired(conn: &Connection) -> Result<(), ServiceError> {
    conn.create_scalar_function(
        "is_expired",
        1,
        FunctionFlags::SQLITE_DETERMINISTIC | FunctionFlags::SQLITE_UTF8,
        move |ctx| {
            let date: Result<Arc<chrono::DateTime<Utc>>, rusqlite::Error> =
                ctx.get_or_create_aux(0, |value| {
                    let str_val = value.as_str().unwrap();
                    let fixed_date = DateTime::parse_from_str(str_val, "").unwrap();
                    return Ok::<chrono::DateTime<Utc>, rusqlite::Error>(fixed_date.to_utc());
                });
            let date: chrono::DateTime<Utc> = (*date.unwrap()).into();
            return Ok(date < Utc::now());
        },
    )
    .or(Err(ServiceError::ConnectionFailure))
}

fn add_get_expiry(conn: &Connection) -> Result<(), ServiceError> {
    conn.create_scalar_function(
        "get_expiry",
        0,
        FunctionFlags::SQLITE_DETERMINISTIC | FunctionFlags::SQLITE_UTF8,
        move |_ctx| {
            let now = Utc::now();
            let duration = TimeDelta::hours(2);
            let expiry_time = now.time().overflowing_add_signed(duration).0;
            let expires_at: DateTime<Utc> = match now.with_time(expiry_time) {
                chrono::offset::LocalResult::Single(time) => time,
                chrono::offset::LocalResult::None => now,
                chrono::offset::LocalResult::Ambiguous(early_time, _) => early_time,
            };
            let str_val = expires_at.to_rfc3339();
            return Ok(str_val);
        },
    )
    .or(Err(ServiceError::ConnectionFailure))
}
