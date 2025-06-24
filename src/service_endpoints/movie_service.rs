use crate::data_models::movie::Movie;
use crate::database_services::{database_utilities::ServiceError, movie_database::MovieDataBase};
use actix_web::{
    delete, get, post, put,
    web::{scope, Json, Path},
    HttpResponse, Responder, Scope,
};

#[post("/new")]
async fn add_movie(new_movie: Json<Movie>) -> Result<impl Responder, ServiceError> {
    if let Some(movie) = Movie::new(
        &new_movie.title,
        &new_movie.format.string(),
        &new_movie.rating.string(),
    ) {
        let did_insert: bool = MovieDataBase::new_movie_with(movie).await?;
        return Ok(HttpResponse::Ok().json(did_insert));
    } else {
        return Err(ServiceError::ConnectionFailure);
    }
}

#[get("/all")]
async fn get_all_movies() -> Result<impl Responder, ServiceError> {
    return match MovieDataBase::get_all_movies().await? {
        Some(movies) => Ok(HttpResponse::Ok().json(movies)),
        None => Err(ServiceError::MovieNotFound),
    };
}

#[get("/{id}")]
async fn get_movie(path: Path<(String,)>) -> Result<impl Responder, ServiceError> {
    let id = path.into_inner().0;
    return match MovieDataBase::get_movie_with_id(id).await? {
        Some(movie) => Ok(HttpResponse::Ok().json(movie)),
        None => Err(ServiceError::MovieNotFound),
    };
}

#[put("/update")]
async fn update_movie_with(updated_movie: Json<Movie>) -> Result<impl Responder, ServiceError> {
    let was_updated: bool = MovieDataBase::update_movie_with(updated_movie.into_inner()).await?;
    return Ok(HttpResponse::Ok().json(was_updated));
}

#[delete("/remove/{id}")]
async fn delete_movie_with(path: Path<(String,)>) -> Result<impl Responder, ServiceError> {
    let id = path.into_inner().0;
    let was_deleted: bool = MovieDataBase::delete_movie(Some(id)).await?;
    return Ok(HttpResponse::Ok().json(was_deleted));
}

#[delete("/remove/all")]
async fn delete_all_movies() -> Result<impl Responder, ServiceError> {
    let was_deleted: bool = MovieDataBase::delete_movie(None).await?;
    return Ok(HttpResponse::Ok().json(was_deleted));
}

pub fn movie_scope() -> Scope {
    scope("/movie")
        .service(add_movie)
        .service(get_all_movies)
        .service(get_movie)
        .service(update_movie_with)
        .service(delete_all_movies)
        .service(delete_movie_with)
}
