use actix_web::{
    error::ResponseError,
    http::StatusCode,
    post,
    web::{scope, Json},
    HttpResponse, Responder, Scope,
};
use derive_more::{Display, Error};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::env;
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

impl ResponseError for AuthServiceError {}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub user_session: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub pash_hash: String,
}

#[post("/login")]
async fn login_user(body: Json<LoginRequest>) -> Result<impl Responder, AuthServiceError> {
    let login_req = body.into_inner();
    let id = get_user_id(login_req.username.clone(), login_req.pash_hash)?;
    let user = create_session_id(id, login_req.username)?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/register")]
async fn register_user() -> Result<impl Responder, AuthServiceError> {
    // validate new user doesn't exist
    // if does return error else:
    //      create user then log in the user
    Ok(HttpResponse::Ok()
        .status(StatusCode::IM_A_TEAPOT)
        .body("teapot"))
}

fn get_connection() -> Result<Connection, AuthServiceError> {
    let db_path = env::var("DB_PATH").unwrap_or("kellum_library.db".to_string());
    return match Connection::open(db_path) {
        Ok(conn) => Ok(conn),
        Err(_) => Err(AuthServiceError::FailedToRegister),
    };
}

fn get_user_id(name: String, pass: String) -> Result<String, AuthServiceError> {
    let conn = get_connection()?;
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
    let conn = get_connection()?;
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

fn create_session_id(user_id: String, name: String) -> Result<User, AuthServiceError> {
    let session_id = Uuid::new_v4().to_string();
    let res = get_connection()?.execute(
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
    return Ok(());
}

pub fn auth_scope() -> Scope {
    scope("/auth").service(login_user).service(register_user)
}
