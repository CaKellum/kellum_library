use actix_web::{
    error::ResponseError,
    get, post,
    web::{scope, Json, Path},
    HttpResponse, Responder, Scope,
};
use rusqlite::{self, auto_extension::register_auto_extension};
use serde::{Deserialize, Serialize};
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

impl error::ResponseError for AuthServiceError {}

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
    // check to make sure user exist
    // if exists:
    //      validagte no current session has user_id equal to the user loging in
    //      if they do return old seession else:
    //          create session id and insert it in to users_sessions
    // else:
    //  return error
}

#[post("/register")]
async fn register_user() -> Result<impl Responder, AuthServiceError> {
    // validate new user doesn't exist
    // if does return error else:
    //      create user then log in the user
}

pub async fn validate_user_session(session_id: String) -> Result<User, AuthServiceError> {
    // get connection
    // look for session_id
    // if no session:
    //     return error
    // else:
    //     return User information
}

fn create_session_id(user: User) -> String {
    return "".to_string();
}

pub fn validate_request() -> Result<(), AuthServiceError> {
    // recive sha from header & generated sha from the request
    // if they match return Ok else return error
    return Ok(());
}

pub fn auth_scope() -> Scope {
    scope("/auth").service(login_user).service(register_user)
}
