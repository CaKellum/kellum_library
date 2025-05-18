use actix_web::{
    body::MessageBody,
    delete,
    dev::{ServiceRequest, ServiceResponse},
    error, get,
    http::{header::ContentType, StatusCode},
    middleware::{from_fn, Next},
    post, put,
    web::{self, Json, Path},
    App, Error, HttpResponse, HttpServer, Responder,
};
use derive_more::derive::{Display, Error};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::{env, usize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
enum PlatformType {
    Playstation1,
    Playstation2,
    Playstation3,
    Playstation4,
    Playstation5,
    NES,
    SNES,
    N64,
    GameCube,
    Wii,
    WiiU,
    Switch,
    Switch2,
    NintendoDS,
    Nintendo3DS,
    Computer,
}

impl PlatformType {
    fn platform_from_string(type_string: &str) -> Option<PlatformType> {
        return match type_string {
            "NES" => Some(PlatformType::NES),
            "SNES" => Some(PlatformType::SNES),
            "N64" => Some(PlatformType::N64),
            "GameCube" => Some(PlatformType::GameCube),
            "Wii" => Some(PlatformType::Wii),
            "WiiU" => Some(PlatformType::WiiU),
            "Switch" => Some(PlatformType::Switch),
            "Switch2" => Some(PlatformType::Switch2),
            "NintendoDS" => Some(PlatformType::NintendoDS),
            "Nintendo3DS" => Some(PlatformType::Nintendo3DS),
            "Computer" => Some(PlatformType::Computer),
            "Playstation1" => Some(PlatformType::Playstation1),
            "Playstation2" => Some(PlatformType::Playstation2),
            "Playstation3" => Some(PlatformType::Playstation3),
            "Playstation4" => Some(PlatformType::Playstation4),
            "Playstation5" => Some(PlatformType::Playstation5),
            _ => None,
        };
    }

    fn string(self) -> String {
        return match self {
            PlatformType::NES => "NES",
            PlatformType::SNES => "SNES",
            PlatformType::N64 => "N64",
            PlatformType::GameCube => "GameCube",
            PlatformType::Wii => "Wii",
            PlatformType::WiiU => "WiiU",
            PlatformType::Switch => "Switch",
            PlatformType::Switch2 => "Switch2",
            PlatformType::NintendoDS => "NintendoDS",
            PlatformType::Nintendo3DS => "Nintendo3DS",
            PlatformType::Computer => "Computer",
            PlatformType::Playstation1 => "Playstation1",
            PlatformType::Playstation2 => "Playstation2",
            PlatformType::Playstation3 => "Playstation3",
            PlatformType::Playstation4 => "Playstation4",
            PlatformType::Playstation5 => "Playstation5",
        }
        .to_string();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
enum ESRBRating {
    Everyone,
    Everyone10,
    Teen,
    Mature,
    AdultOnly,
}

impl ESRBRating {
    fn rating_from_string(rating_string: &str) -> Option<ESRBRating> {
        return match rating_string {
            "Everyone" => Some(ESRBRating::Everyone),
            "Everyone10" => Some(ESRBRating::Everyone10),
            "Teen" => Some(ESRBRating::Teen),
            "Mature" => Some(ESRBRating::Mature),
            "AdultOnly" => Some(ESRBRating::AdultOnly),
            _ => None,
        };
    }

    fn string(self) -> String {
        return match self {
            ESRBRating::Everyone => "Everyone",
            ESRBRating::Everyone10 => "Everyone10",
            ESRBRating::Teen => "Teen",
            ESRBRating::Mature => "Mature",
            ESRBRating::AdultOnly => "AdultOnly",
        }
        .to_string();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Game {
    id: String,
    title: String,
    platform: PlatformType,
    rating: ESRBRating,
    number_of_players: u8,
}

impl Game {
    fn new(title: String, platform: &str, rating: &str, number_of_players: u8) -> Option<Game> {
        let platform = PlatformType::platform_from_string(platform)?;
        let rating = ESRBRating::rating_from_string(rating)?;
        return Some(Game {
            id: Uuid::new_v4().to_string(),
            title,
            platform,
            rating,
            number_of_players,
        });
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum MPAARating {
    GeneralAudiences,
    ParentalGuidance,
    ParentsStronglyCautioned,
    Restricted,
    AdultsOnly,
}

impl MPAARating {
    fn from_string(rating_string: &str) -> Option<Self> {
        match rating_string {
            "GeneralAudiences" => Some(Self::GeneralAudiences),
            "ParentalGuidance" => Some(Self::ParentalGuidance),
            "ParentsStronglyCautioned" => Some(Self::ParentsStronglyCautioned),
            "Restricted" => Some(Self::Restricted),
            "AdultsOnly" => Some(Self::AdultsOnly),
            _ => None,
        }
    }

    fn string(&self) -> String {
        match self {
            Self::GeneralAudiences => "GeneralAudiences".to_string(),
            Self::ParentalGuidance => "ParentalGuidance".to_string(),
            Self::ParentsStronglyCautioned => "ParentsStronglyCautioned".to_string(),
            Self::Restricted => "Restricted".to_string(),
            Self::AdultsOnly => "AdultsOnly".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum MotionPictureFormat {
    BluRay,
    UltraHD,
    DVD,
    VHS,
}

impl MotionPictureFormat {
    fn from_string(rating_string: &str) -> Option<Self> {
        match rating_string {
            "BluRay" => Some(Self::BluRay),
            "UltraHD" => Some(Self::UltraHD),
            "DVD" => Some(Self::DVD),
            "VHS" => Some(Self::VHS),
            _ => None,
        }
    }

    fn string(&self) -> String {
        match self {
            Self::BluRay => "BluRay".to_string(),
            Self::DVD => "DVD".to_string(),
            Self::UltraHD => "UltraHD".to_string(),
            Self::VHS => "VHS".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Movie {
    id: String,
    title: String,
    format: MotionPictureFormat,
    rating: MPAARating,
}

impl Movie {
    fn new(title: &str, format: &str, rating: &str) -> Option<Self> {
        let format = MotionPictureFormat::from_string(format)?;
        let rating = MPAARating::from_string(rating)?;
        return Some(Movie {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            format,
            rating,
        });
    }
}

#[derive(Debug, Display, Error)]
enum ServiceError {
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

struct MovieDataBase;
impl MovieDataBase {
    async fn get_connection() -> Result<rusqlite::Connection, ServiceError> {
        let db_path = env::var("DB_PATH").unwrap_or("kellum_library.db".to_string());
        return match Connection::open(db_path) {
            Ok(conn) => Ok(conn),
            Err(_) => Err(ServiceError::ConnectionFailure),
        };
    }

    async fn new_movie_with(new_movie: Movie) -> Result<bool, ServiceError> {
        let conn = Self::get_connection().await?;
        let res = conn.execute(
            "INSERT INTO(id, title, format, rating) VALUES( 1?, 2?, 3?, 4?)",
            [
                new_movie.id,
                new_movie.title,
                new_movie.format.string(),
                new_movie.rating.string(),
            ],
        );
        return match res {
            Ok(rows_altered) => {
                if rows_altered > 0 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(_) => Err(ServiceError::ConnectionFailure),
        };
    }

    async fn get_movie_with_id(id: String) -> Result<Option<Movie>, ServiceError> {
        let conn = Self::get_connection().await?;
        let res = conn.query_row_and_then(
            "SELECT id, title, format, rating FROM movies WHERE id=1?",
            [id],
            |row| {
                if let Some(format) =
                    MotionPictureFormat::from_string(&row.get::<usize, String>(2)?)
                {
                    if let Some(rating) = MPAARating::from_string(&row.get::<usize, String>(3)?) {
                        return Ok(Movie {
                            id: row.get::<usize, String>(0)?,
                            title: row.get::<usize, String>(1)?,
                            format,
                            rating,
                        });
                    }
                }
                return Err(rusqlite::Error::FromSqlConversionFailure(
                    1,
                    rusqlite::types::Type::Real,
                    Box::new(ServiceError::FailedToMakeMovie),
                ));
            },
        );
        return match res {
            Ok(movie) => Ok(Some(movie)),
            Err(_) => Err(ServiceError::MovieNotFound),
        };
    }
    async fn get_all_movies() -> Result<Option<Vec<Movie>>, ServiceError> {
        let conn = Self::get_connection().await?;
        let mut stmnt = conn
            .prepare("SELECT id, title, format, rating FROM movies")
            .unwrap();
        let res = stmnt.query_map([], |row| {
            if let Some(format) = MotionPictureFormat::from_string(&row.get::<usize, String>(2)?) {
                if let Some(rating) = MPAARating::from_string(&row.get::<usize, String>(3)?) {
                    return Ok(Movie {
                        id: row.get::<usize, String>(0)?,
                        title: row.get::<usize, String>(1)?,
                        format,
                        rating,
                    });
                }
            }
            return Err(rusqlite::Error::FromSqlConversionFailure(
                1,
                rusqlite::types::Type::Real,
                Box::new(ServiceError::FailedToMakeMovie),
            ));
        });
        return match res {
            Ok(movies) => {
                let mut movie_list = vec![];
                movies.for_each(|movie| match movie {
                    Ok(movie) => movie_list.push(movie),
                    Err(_) => {}
                });
                return Ok(Some(movie_list));
            }
            Err(_) => Err(ServiceError::MovieNotFound),
        };
    }

    async fn update_movie_with(new_movie: Movie) -> Result<bool, ServiceError> {
        let conn = Self::get_connection().await?;
        let res = conn.execute(
            "UPDATE movies SET title=1?, format=2?, rating=3? WHERE id=4?",
            [
                new_movie.title,
                new_movie.format.string(),
                new_movie.rating.string(),
                new_movie.id,
            ],
        );
        return match res {
            Ok(row_count) => {
                if row_count > 0 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(_) => Err(ServiceError::FailedToUpdateMovie),
        };
    }

    async fn delete_movie(id: Option<String>) -> Result<bool, ServiceError> {
        let conn = Self::get_connection().await?;
        let res = match id {
            Some(id) => conn.execute("DELETE FROM movies WHERE id=1?", [id]),
            None => conn.execute("DROP TABLE movies", []),
        };

        return match res {
            Ok(_) => Ok(true),
            Err(_) => Err(ServiceError::MovieNotFound),
        };
    }
}

struct GameDataBase;
impl GameDataBase {
    async fn get_connection() -> Result<rusqlite::Connection, ServiceError> {
        let db_path = env::var("DB_PATH").unwrap_or("kellum_library.db".to_string());
        return match Connection::open(db_path) {
            Ok(conn) => Ok(conn),
            Err(_) => Err(ServiceError::ConnectionFailure),
        };
    }

    async fn get_game_with_id(id: String) -> Result<Option<Game>, ServiceError> {
        let conn = GameDataBase::get_connection().await?;
        let res = conn.query_row_and_then(
            "SELECT id, title, platform, rating, number_of_players FROM games WHERE id=1?",
            [id],
            |row| {
                if let Some(platform) =
                    PlatformType::platform_from_string(&row.get::<usize, String>(2)?)
                {
                    if let Some(rating) =
                        ESRBRating::rating_from_string(&row.get::<usize, String>(3)?)
                    {
                        return Ok(Game {
                            id: row.get::<usize, String>(0)?,
                            title: row.get::<usize, String>(1)?.to_string(),
                            platform,
                            rating,
                            number_of_players: row.get::<usize, u8>(4)?,
                        });
                    }
                }
                return Err(rusqlite::Error::FromSqlConversionFailure(
                    1,
                    rusqlite::types::Type::Real,
                    Box::new(ServiceError::FailedToMakeGame),
                ));
            },
        );
        let interpreted_res = match res {
            Ok(game) => Ok(Some(game)),
            Err(_) => Err(ServiceError::GameNotFound),
        };
        return Ok(interpreted_res?);
    }

    async fn get_games() -> Result<Option<Vec<Game>>, ServiceError> {
        let conn = GameDataBase::get_connection().await?;
        let mut statment = conn
            .prepare("SELECT id, title, platform, rating, number_of_players FROM games")
            .unwrap();
        let games_res = statment.query_map([], |row| {
            if let Some(platform) =
                PlatformType::platform_from_string(&row.get::<usize, String>(2)?)
            {
                if let Some(rating) = ESRBRating::rating_from_string(&row.get::<usize, String>(3)?)
                {
                    return Ok(Game {
                        id: row.get::<usize, String>(0)?,
                        title: row.get::<usize, String>(1)?,
                        platform,
                        rating,
                        number_of_players: row.get::<usize, u8>(4)?,
                    });
                }
            }
            return Err(rusqlite::Error::FromSqlConversionFailure(
                1,
                rusqlite::types::Type::Real,
                Box::new(ServiceError::FailedToMakeGame),
            ));
        });
        return match games_res {
            Ok(game_map) => {
                let mut game_list = vec![];
                game_map.for_each(|game| match game {
                    Ok(game) => game_list.push(game),
                    Err(_) => {}
                });
                return Ok(Some(game_list));
            }
            Err(_) => Err(ServiceError::GameNotFound),
        };
    }

    async fn delete_game(id: Option<String>) -> Result<bool, ServiceError> {
        let conn = GameDataBase::get_connection().await?;
        let result = match id {
            Some(id) => conn.execute("DELETE FROM games WHERE id=?1", [id]),
            None => conn.execute("DROP TABLE games", []),
        };
        return match result {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        };
    }

    async fn update_game(updated_game: Game) -> Result<bool, ServiceError> {
        let conn = GameDataBase::get_connection().await?;
        let statement_result = conn.execute("UPDATE games 
                                             SET title=?1, platform=?2, rating=?3, number_of_players=?4 
                                             WHERE id=?5", 
                                             [updated_game.title, updated_game.platform.string(),
                                             updated_game.number_of_players.to_string()]);
        return match statement_result {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        };
    }

    async fn insert_game(new_game: Game) -> Result<bool, ServiceError> {
        let conn = GameDataBase::get_connection().await?;
        let statement_result = conn.execute(
            "INSERT INTO games (id, title, platform, rating, number_of_players) 
                                             VALUES (?1,?2,?3,?4,?5);",
            [
                new_game.id,
                new_game.title,
                new_game.platform.string(),
                new_game.rating.string(),
                new_game.number_of_players.to_string(),
            ],
        );
        return match statement_result {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        };
    }
}

#[post("/new")]
async fn add_movie(new_movie: Json<Movie>) -> Result<impl Responder, ServiceError> {
    if let Some(movie) = Movie::new(
        &new_movie.title,
        &new_movie.format.string(),
        &new_movie.rating.string(),
    ) {
        let did_insert = MovieDataBase::new_movie_with(movie).await?;
        return Ok(HttpResponse::Ok().json(did_insert));
    } else {
        return Err(ServiceError::ConnectionFailure);
    }
}

#[post("/new")]
async fn add_game(new_game: Json<Game>) -> Result<impl Responder, ServiceError> {
    let real_new_game = Game::new(
        new_game.title.clone(),
        &new_game.platform.string(),
        &new_game.rating.string(),
        new_game.number_of_players,
    );
    if let Some(game) = real_new_game {
        let did_insert = GameDataBase::insert_game(game).await?;
        return Ok(HttpResponse::Ok().json(did_insert));
    } else {
        return Err(ServiceError::FailedToMakeGame);
    }
}

#[get("/all")]
async fn get_all_movies() -> Result<impl Responder, ServiceError> {
    return match MovieDataBase::get_all_movies().await? {
        Some(movies) => Ok(HttpResponse::Ok().json(movies)),
        None => Err(ServiceError::MovieNotFound),
    };
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
async fn get_movie(path: Path<(String,)>) -> Result<impl Responder, ServiceError> {
    let id = path.into_inner().0;
    return match MovieDataBase::get_movie_with_id(id).await? {
        Some(movie) => Ok(HttpResponse::Ok().json(movie)),
        None => Err(ServiceError::MovieNotFound),
    };
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
async fn update_movie_with(updated_movie: Json<Movie>) -> Result<impl Responder, ServiceError> {
    let was_updated = MovieDataBase::update_movie_with(updated_movie.into_inner()).await?;
    return Ok(HttpResponse::Ok().json(was_updated));
}

#[put("/update")]
async fn update_game_with(updated_game: Json<Game>) -> Result<impl Responder, ServiceError> {
    let was_updated = GameDataBase::update_game(updated_game.into_inner()).await?;
    return Ok(HttpResponse::Ok().json(was_updated));
}

#[delete("/remove/{id}")]
async fn delete_movie_with(path: Path<(String,)>) -> Result<impl Responder, ServiceError> {
    let id = path.into_inner().0;
    let was_deleted = MovieDataBase::delete_movie(Some(id)).await?;
    return Ok(HttpResponse::Ok().json(was_deleted));
}

#[delete("/remove/{id}")]
async fn delete_game_with(path: Path<(String,)>) -> Result<impl Responder, ServiceError> {
    let id = path.into_inner().0;
    let was_deleted = GameDataBase::delete_game(Some(id)).await?;
    return Ok(HttpResponse::Ok().json(was_deleted));
}

#[delete("/remove/all")]
async fn delete_all_movies() -> Result<impl Responder, ServiceError> {
    let was_deleted = MovieDataBase::delete_movie(None).await?;
    return Ok(HttpResponse::Ok().json(was_deleted));
}

#[delete("/remove/all")]
async fn delete_all_games() -> Result<impl Responder, ServiceError> {
    let was_deleted = GameDataBase::delete_game(None).await?;
    return Ok(HttpResponse::Ok().json(was_deleted));
}

async fn my_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // pre-processing
    next.call(req).await
    // post-processing
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        let game_scope = web::scope("/game")
            .service(add_game)
            .service(get_all_games)
            .service(get_games)
            .service(update_game_with)
            .service(delete_game_with)
            .service(delete_all_games);
        let movie_scope = web::scope("/movie")
            .service(add_movie)
            .service(get_all_movies)
            .service(get_movie)
            .service(update_movie_with)
            .service(delete_all_movies)
            .service(delete_movie_with);
        App::new()
            .service(game_scope)
            .service(movie_scope)
            .wrap(from_fn(my_middleware))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
