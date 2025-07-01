use crate::data_models::game::Game;
use crate::database_services::game_database::GameDataBase;
use crate::errors::servive_errors::ServiceError;
use actix_web::{
    delete, get, post, put,
    web::{scope, Json, Path},
    HttpResponse, Responder, Scope,
};

#[post("/new")]
async fn add_game(new_game: Json<Game>) -> Result<impl Responder, ServiceError> {
    let real_new_game = Game::new(
        new_game.title.clone(),
        &new_game.platform.string(),
        &new_game.rating.string(),
        new_game.number_of_players,
    );
    if let Some(game) = real_new_game {
        let did_insert: bool = GameDataBase::insert_game(game).await?;
        println!("successfully made game");
        return Ok(HttpResponse::Ok().json(did_insert));
    } else {
        println!("failed to make game");
        return Err(ServiceError::FailedToMakeGame);
    }
}

#[get("/all")]
async fn get_all_games() -> Result<impl Responder, ServiceError> {
    let resp = match GameDataBase::get_games().await? {
        Some(games) => Ok(HttpResponse::Ok().json(games)),
        None => Err(ServiceError::GameNotFound),
    };
    return resp;
}

#[get("/{id}")]
async fn get_games(path: Path<(String,)>) -> Result<impl Responder, ServiceError> {
    let id = path.into_inner().0;
    return match GameDataBase::get_game_with_id(id).await? {
        Some(game) => Ok(HttpResponse::Ok().json(game)),
        None => Err(ServiceError::GameNotFound),
    };
}

#[put("/update")]
async fn update_game_with(updated_game: Json<Game>) -> Result<impl Responder, ServiceError> {
    let was_updated: bool = GameDataBase::update_game(updated_game.into_inner()).await?;
    return Ok(HttpResponse::Ok().json(was_updated));
}

#[delete("/remove/{id}")]
async fn delete_game_with(path: Path<(String,)>) -> Result<impl Responder, ServiceError> {
    let id = path.into_inner().0;
    let was_deleted: bool = GameDataBase::delete_game(Some(id)).await?;
    return Ok(HttpResponse::Ok().json(was_deleted));
}

#[delete("/remove/all")]
async fn delete_all_games() -> Result<impl Responder, ServiceError> {
    let was_deleted: bool = GameDataBase::delete_game(None).await?;
    return Ok(HttpResponse::Ok().json(was_deleted));
}

pub fn game_scope() -> Scope {
    scope("/game")
        .service(add_game)
        .service(get_all_games)
        .service(get_games)
        .service(update_game_with)
        .service(delete_game_with)
        .service(delete_all_games)
}
