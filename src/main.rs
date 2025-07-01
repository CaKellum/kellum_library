use crate::database_services::database_utilities::ServiceError;
use crate::service_endpoints::{
    auth_servce::auth_scope, game_service::game_scope, movie_service::movie_scope,
};
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::{from_fn, Next},
    App, Error, HttpMessage, HttpServer,
};

pub mod data_models;
pub mod database_services;
pub mod service_endpoints;

//TODO: Move to own module
async fn my_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    println!("{}", req.path());
    println!("{}", req.content_type());
    println!("{}", req.method());
    let res = next.call(req).await;
    return res;
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .service(game_scope())
            .service(movie_scope())
            .service(auth_scope())
            .wrap(from_fn(my_middleware))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// TODO: Move to test directory
#[cfg(test)]
mod tests {
    use crate::data_models::{game::*, movie::*};
    use serde_json;

    #[test]
    fn test_movie_encoding() {
        let str_data = "{
            \"id\": \"\",
            \"title\": \"Troy\",
            \"format\": \"DVD\",
            \"rating\": \"Restricted\"
        }";
        let expected_movie = Movie {
            id: "".to_string(),
            title: "Troy".to_string(),
            format: MotionPictureFormat::DVD,
            rating: MPAARating::Restricted,
        };
        let movie: Movie = serde_json::from_str(str_data).unwrap();
        assert_eq!(expected_movie.id, movie.id);
        assert_eq!(expected_movie.title, movie.title);
        assert_eq!(expected_movie.format, movie.format);
        assert_eq!(expected_movie.rating, movie.rating);
    }

    #[test]
    fn test_game_encoding() {
        let str_data = "{
        \"id\":\"\",
        \"title\":\"syphon filter 2\",
        \"platform\":\"Playstation1\",
        \"rating\":\"Mature\"
            ,\"number_of_players\":1
        }";

        let expected_game = Game {
            id: "".to_string(),
            title: "syphon filter 2".to_string(),
            rating: ESRBRating::Mature,
            platform: PlatformType::Playstation1,
            number_of_players: 1,
        };

        let game: Game = serde_json::from_str(str_data).unwrap();

        assert_eq!(expected_game.id, game.id);
        assert_eq!(expected_game.title, game.title);
        assert_eq!(expected_game.platform, game.platform);
        assert_eq!(expected_game.rating, game.rating);
        assert_eq!(expected_game.number_of_players, game.number_of_players);
    }
}
