use crate::data_models::auth_models::LoginRequest;
use crate::database_services::authentication_services::{
    create_session_id, create_user, get_user_id,
};
use crate::errors::auth_errors::AuthServiceError;
use actix_web::{
    post,
    web::{scope, Json},
    HttpResponse, Responder, Scope,
};

#[post("/login")]
async fn login_user(body: Json<LoginRequest>) -> Result<impl Responder, AuthServiceError> {
    let login_req = body.into_inner();
    let id = get_user_id(login_req.username.clone(), login_req.pass_hash)?;
    let user = create_session_id(id, login_req.username)?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/register")]
async fn register_user(body: Json<LoginRequest>) -> Result<impl Responder, AuthServiceError> {
    let request = body.into_inner();
    let user = create_user(request)?;
    Ok(HttpResponse::Ok().json(user))
}

pub fn auth_scope() -> Scope {
    scope("/auth").service(login_user).service(register_user)
}
