use crate::data_models::auth_models::User;
use crate::database_services::database_utilities::get_connection;
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use actix_web::{http::header::ContentType, HttpResponse};
use derive_more::{Display, Error};
use uuid::Uuid;

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

impl ResponseError for AuthServiceError {
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

pub fn get_user_id(name: String, pass: String) -> Result<String, AuthServiceError> {
    let conn = get_connection().or(Err(AuthServiceError::FailedToRegister))?;
    let res = conn.query_row(
        "SELECT id FROM users WHERE username == 1? && passHash == 2?",
        [name, pass],
        |row| {
            let id = row.get::<usize, String>(0)?;
            return Ok(id);
        },
    );
    return match res {
        Ok(id) => Ok(id),
        Err(_) => Err(AuthServiceError::FailedToAuthenticate),
    };
}

pub async fn validate_user_session(session_id: String) -> Result<User, AuthServiceError> {
    let conn = get_connection().or(Err(AuthServiceError::FailedToRegister))?;
    // TODO: Deal with the expiry in the session
    let res = conn.query_row("SELECT us.id, u.username FROM user_sessions as us INNER JOIN users as u on us.user_id = u.id WHERE us.id == 1?", 
        [session_id],
        |row| {
        let id = row.get(0)?;
        let user_name= row.get(1)?;
        return Ok(User{ username: user_name, user_session: id });
    });

    return match res {
        Ok(user) => Ok(user),
        Err(_) => Err(AuthServiceError::InvalidSessionToken),
    };
}

pub fn create_session_id(user_id: String, name: String) -> Result<User, AuthServiceError> {
    let session_id = Uuid::new_v4().to_string();
    let conn = get_connection().or(Err(AuthServiceError::FailedToRegister))?;
    let res = conn.execute(
        "INSERT INTO user_sessions(id, user_id, expiry) VALUES(1?, 2?, 3?)",
        [session_id.clone(), user_id, "".to_string()],
    );
    match res {
        Ok(_) => Ok(User {
            username: name,
            user_session: session_id,
        }),
        Err(_) => Err(AuthServiceError::FailedToAuthenticate),
    }
}

pub fn validate_request() -> Result<(), AuthServiceError> {
    // recive sha from header & generated sha from the request
    // if they match return Ok else return error
    return Err(AuthServiceError::SuspiciousRequest);
}
