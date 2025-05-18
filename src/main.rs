use std::{usize,env};
use actix_web::{body::MessageBody, delete, dev::{ServiceRequest, ServiceResponse}, error, get, http::{header::ContentType, StatusCode}, middleware::{from_fn, Next}, post, put, web::{ self, Json, Path}, App, Error, HttpResponse, HttpServer, Responder};
use derive_more::derive::{Display, Error};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
enum PlatformType {
   Playstation1, Playstation2, Playstation3, Playstation4, Playstation5,
   NES, SNES, N64, GameCube, Wii, WiiU, Switch, Switch2,
   NintendoDS, Nintendo3DS, Computer
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
            _ => None 
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
            PlatformType::Playstation5 => "Playstation5"
        }.to_string();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
enum ESRBRating { 
    Everyone, Everyone10, Teen, Mature, AdultOnly 
}

impl ESRBRating {
    fn rating_from_string(rating_string: &str) -> Option<ESRBRating> {
        return match rating_string {
            "Everyone" => Some(ESRBRating::Everyone),
            "Everyone10" => Some(ESRBRating::Everyone10),
            "Teen" => Some(ESRBRating::Teen),
            "Mature" => Some(ESRBRating::Mature),
            "AdultOnly" => Some(ESRBRating::AdultOnly),
            _ => None
        };
    }

    fn string(self) -> String {
        return match self {
            ESRBRating::Everyone => "Everyone",
            ESRBRating::Everyone10 => "Everyone10",
            ESRBRating::Teen => "Teen",
            ESRBRating::Mature => "Mature",
            ESRBRating::AdultOnly => "AdultOnly"
        }.to_string();
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
       return Some(Game{ id: Uuid::new_v4().to_string(), title, platform, rating, number_of_players });
   }
}

enum MPAARating {
    GeneralAudiences,
    ParentalGuidance,
    ParentsStronglyCautioned,
    Restricted,
    AdultsOnly
}

impl MPAARating {
   fn from_string(rating_string: &str) -> Option<Self> {
       match rating_string {
           _ => None
       }
   }
}

enum MotionPictureFormat {
   BluRay,
   UltraHD,
   DVD,
   VHS
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Movie {
    id: String,
    title: String,
    format: MotionPictureFormat,
    rating: MPAARating
}

impl Movie { 
}

#[derive(Debug, Display, Error)]
enum ServiceError {
    #[display("failed to connect to DB.")]
    ConnectionFailure,
    #[display("Failed to find a game for specified id")]
    GameNotFound,
    #[display("Failed to make new game")]
    FailedToMakeGame,
}

impl error::ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        return HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(self.to_string());
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        return match *self {
           Self::ConnectionFailure => StatusCode::INTERNAL_SERVER_ERROR, 
           Self::GameNotFound => StatusCode::NOT_FOUND,
           Self::FailedToMakeGame => StatusCode::IM_A_TEAPOT
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
        let res = conn.query_row_and_then("SELECT id, title, platform, rating, number_of_players FROM games WHERE id=1?", [id],
            |row| {
                if let Ok(id) = row.get::<usize, String>(0) {
                    if let Ok(title) = row.get::<usize, String>(1) {
                        if let Ok(platform_string) = row.get::<usize, String>(2) { 
                            if let Ok(rating_string) = row.get::<usize, String>(3) { 
                                if let Ok(number) = row.get::<usize, u8>(4) {
                                    if let Some(platform) = PlatformType::platform_from_string(&platform_string) {
                                        if let Some(rating) = ESRBRating::rating_from_string(&rating_string) {
                                            return Ok(Game{id, title: title.to_string(), platform, rating,
                                                   number_of_players: number});
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                return Err(rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Real,
                        Box::new(ServiceError::GameNotFound)));
        });
        let interpreted_res = match res {
            Ok(game) => Ok(Some(game)),
            Err(_) => Err(ServiceError::GameNotFound)
        };
        return Ok(interpreted_res?);
    }

    async fn get_games() -> Result<Option<Vec<Game>>, ServiceError> {
        let conn = GameDataBase::get_connection().await?;
        let mut statment = conn.prepare("SELECT id, title, platform, rating, number_of_players FROM games").unwrap();
        let games_res = statment.query_map([], |row| {
                if let Some(platform) = PlatformType::platform_from_string(&row.get::<usize, String>(2)?) {
                    if let Some(rating) = ESRBRating::rating_from_string(&row.get::<usize, String>(3)?) {
                        return Ok(Game{ id: row.get::<usize, String>(0)?,
                                        title: row.get::<usize, String>(1)?, 
                                        platform, rating, 
                                        number_of_players: row.get::<usize, u8>(4)?})
                    }
                }
                return Err(rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Real,
                        Box::new(ServiceError::GameNotFound)));
        });
        return match games_res {
            Ok(game_map) => {
                let mut game_list = vec![];
                game_map.for_each(|game| {
                    match game {
                        Ok(game) => game_list.push(game),
                        Err(_) => {}
                    }
                });
                return Ok(Some(game_list));
            },
            Err(_) => Err(ServiceError::GameNotFound)
        };
    }

    async fn delete_game(id: Option<String>) -> Result<bool, ServiceError> {
        let conn = GameDataBase::get_connection().await?;
        let result = match id {
            Some(id) => conn.execute("DELETE FROM games WHERE id=?1",[id]),
            None => conn.execute("DROP TABLE games", []) 
        };
        return match result { Ok(_) => Ok(true), Err(_) => Ok(false) };
    }

    async fn update_game(updated_game: Game) -> Result<bool, ServiceError> {
        let conn = GameDataBase::get_connection().await?;
        let statement_result = conn.execute("UPDATE games 
                                             SET title=?1, platform=?2, rating=?3, number_of_players=?4 
                                             WHERE id=?5", 
                                             [updated_game.title, updated_game.platform.string(),
                                             updated_game.number_of_players.to_string()]);
        return match statement_result { Ok(_) => Ok(true), Err(_) => Ok(false) };
    }

    async fn insert_game(new_game: Game) -> Result<bool, ServiceError> {
        let conn = GameDataBase::get_connection().await?;
        let statement_result = conn.execute("INSERT INTO games (id, title, platform, rating, number_of_players) 
                                             VALUES (?1,?2,?3,?4,?5);", 
                                            [new_game.id, new_game.title, new_game.platform.string(),
                                            new_game.rating.string(),new_game.number_of_players.to_string()]);
        return match statement_result { Ok(_) => Ok(true), Err(_) => Ok(false) };
    }
}

#[post("/new")]
async fn add_game(new_game: Json<Game>) -> Result<impl Responder, ServiceError> {
    let real_new_game = Game::new(new_game.title.clone(), &new_game.platform.string(), 
                                  &new_game.rating.string(), new_game.number_of_players);
    if let Some(game) = real_new_game {
        let did_insert = GameDataBase::insert_game(game).await?;
        println!("receving game new");
        return Ok(HttpResponse::Ok().json(did_insert));
    } else {
        return Err(ServiceError::FailedToMakeGame);
    }
}

#[get("/all")]
async fn get_all_games() -> Result<impl Responder, ServiceError> {
    let resp = match GameDataBase::get_games().await? {
        Some(games) => HttpResponse::Ok().json(games),
        None => HttpResponse::NotFound().body("games not Found")
    };
    return Ok(resp);
}

#[get("/{id}")]
async fn get_games(path: Path<(String,)>) -> Result<impl Responder, ServiceError> {
    let id = path.into_inner().0;
    let resp = match GameDataBase::get_game_with_id(id).await? {
        Some(game) => HttpResponse::Ok().json(game),
        None => HttpResponse::NotFound().body("games not Found")
    };
    return Ok(resp);
}

#[put("/update")]
async fn update_game_with(updated_game: Json<Game>) -> Result<impl Responder, ServiceError> {
    let was_updated = GameDataBase::update_game(updated_game.into_inner()).await?;
    return Ok(HttpResponse::Ok().json(was_updated));
}

#[delete("/remove/{id}")]
async fn delete_game_with(path: Path<(String,)>) -> Result<impl Responder, ServiceError> {
    let id = path.into_inner().0;
    let was_deleted = GameDataBase::delete_game(Some(id)).await?;
    return Ok(HttpResponse::Ok().json(was_deleted));
}

#[delete("/remove/all")]
async fn delete_all_games() -> Result<impl Responder, ServiceError> {
    let was_deleted = GameDataBase::delete_game(None).await?;
    return Ok(HttpResponse::Ok().json(was_deleted));
}

async fn my_middleware(req: ServiceRequest, next: Next<impl MessageBody>,) -> Result<ServiceResponse<impl MessageBody>, Error> {
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
        let movie_scope = web::scope("/movie");
        App::new()
            .service(game_scope)
            .service(movie_scope)
            .wrap(from_fn(my_middleware))
    }).bind(("127.0.0.1", 8080))?.run().await
}
