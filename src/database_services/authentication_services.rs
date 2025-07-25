use crate::data_models::auth_models::{LoginRequest, User};
use crate::database_services::database_utilities::get_connection;
use crate::errors::auth_errors::AuthServiceError;
use crate::service_endpoints::auth_servce::register_user;
use actix_web::guard::Put;
use std::usize;
use uuid::Uuid;

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
    let res = conn.query_row("SELECT us.id, u.username, us.expiry FROM user_sessions as us INNER JOIN users as u on us.user_id = u.id WHERE us.id == 1?", 
        [session_id],
        |row| {
        let id: String = row.get(0)?;
        let user_name: String = row.get(1)?;
        let is_expired = conn.query_row("SELECT is_expired(1?)", [id.clone()], |row| {
            if let Ok(is_expired) = row.get::<usize, bool>(0) {
                return Ok(is_expired);
            }
            return Ok(true)
        })?;
        if !is_expired {
            return Ok(User{ username: user_name, user_session: id });
        } else {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
    });

    return match res {
        Ok(user) => Ok(user),
        Err(_) => Err(AuthServiceError::InvalidSessionToken),
    };
}

pub fn create_user(request_body: LoginRequest) -> Result<User, AuthServiceError> {
    let user_id = Uuid::new_v4().to_string();
    let conn = get_connection().or(Err(AuthServiceError::FailedToRegister))?;
    let statement_result = conn.execute(
        "INSERT INTO users (id, username, passHash) 
                                             VALUES (?1,?2,?3);",
        [
            user_id.clone(),
            request_body.username.clone(),
            request_body.pass_hash,
        ],
    );
    if statement_result.is_err() {
        return Err(AuthServiceError::FailedToRegister);
    }
    let session_id = create_session_id(user_id.clone(), request_body.username);
    if let Ok(user) = session_id {
        return Ok(user);
    } else {
        return Err(AuthServiceError::FailedToRegister);
    }
}

pub fn create_session_id(user_id: String, name: String) -> Result<User, AuthServiceError> {
    let session_id = Uuid::new_v4().to_string();
    let conn = get_connection().or(Err(AuthServiceError::FailedToRegister))?;
    let expiry = conn
        .query_row("SELECT get_expiry()", [], |row| {
            return Ok(row.get::<usize, String>(0)?);
        })
        .or(Err(AuthServiceError::FailedToRegister))?;
    let res = conn.execute(
        "INSERT INTO user_sessions(id, user_id, expiry) VALUES(1?, 2?, 3?)",
        [session_id.clone(), user_id, expiry],
    );
    match res {
        Ok(_) => Ok(User {
            username: name,
            user_session: session_id,
        }),
        Err(_) => Err(AuthServiceError::FailedToAuthenticate),
    }
}

pub fn validate_request(request: actix_web::dev::ServiceRequest) -> Result<(), AuthServiceError> {
    let session_id = request.headers().get("session_id");
    if let Some(session_id) = session_id {
        let conn = get_connection().or(Err(AuthServiceError::FailedToRegister))?;
        let res = conn.query_row(
            "SELECT is_expired(1?)",
            [session_id
                .to_str()
                .or(Err(AuthServiceError::FailedToAuthenticate))?],
            |row| Ok(row.get::<usize, bool>(0)?),
        );
        return match res {
            Ok(res) => {
                if res {
                    return Ok(());
                } else {
                    return Err(AuthServiceError::InvalidSessionToken);
                }
            }
            Err(_) => Err(AuthServiceError::FailedToRegister),
        };
    }
    return Err(AuthServiceError::GenerallyForbiden);
}
