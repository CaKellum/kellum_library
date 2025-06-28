use crate::data_models::auth_models::LoginRequest;
use crate::database_services::authentication_services::{
    create_session_id, get_user_id, AuthServiceError,
};
use actix_web::{
    http::StatusCode,
    post,
    web::{scope, Json},
    HttpResponse, Responder, Scope,
};

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

pub fn auth_scope() -> Scope {
    scope("/auth").service(login_user).service(register_user)
}
