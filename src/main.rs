use std::usize;
use actix_web::{delete, error, get,
                http::{header::ContentType, StatusCode}, 
                middleware::{from_fn, Next}, post, put, 
                web::{ self, Json, Path}, App, HttpResponse, HttpServer, Responder, 
                dev::{ServiceRequest, ServiceResponse}, body::MessageBody, Error};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use derive_more::derive::{Display, Error};
use rusqlite::{Connection};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
enum PlatformType {
   Playstation1, Playstation2, Playstation3, Playstation4, Playstation5,
   NES, SNES, N64, GameCube, Wii, WiiU, Switch, Switch2,
   NintendoDS, Nintendo3DS, Computer
}

impl PlatformType {
    fn platform_from_string(type_string: &str) -> PlatformType {
        return match type_string {
            "NES" => PlatformType::NES,
            "SNES" => PlatformType::SNES,
            "N64" => PlatformType::N64,
            "GameCube" => PlatformType::GameCube,
            "Wii" => PlatformType::Wii,
            "WiiU" => PlatformType::WiiU,
            "Switch" => PlatformType::Switch, 
            "Switch2" => PlatformType::Switch2,
            "NintendoDS" => PlatformType::NintendoDS,
            "Nintendo3DS" => PlatformType::Nintendo3DS,
            "Computer" => PlatformType::Computer,
            "Playstation1" => PlatformType::Playstation1,
            "Playstation2" => PlatformType::Playstation2, 
            "Playstation3" => PlatformType::Playstation3, 
            "Playstation4" => PlatformType::Playstation4,
            "Playstation5" => PlatformType::Playstation5,
            _ => todo!("make a non platform")
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
    fn rating_from_string(rating_string: &str) -> ESRBRating {
        return match rating_string {
            "Everyone" => ESRBRating::Everyone,
            "Everyone10" => ESRBRating::Everyone10,
            "Teen" => ESRBRating::Teen,
            "Mature" => ESRBRating::Mature,
            "AdultOnly" => ESRBRating::AdultOnly,
            _ => todo!("make a non rating")
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
   fn new(title: String, platform: &str, rating: &str, number_of_players: u8) -> Game {
       let platform = PlatformType::platform_from_string(platform);
       let rating = ESRBRating::rating_from_string(rating);
       return Game { id: Uuid::new_v4().to_string(), title: title, platform: platform, rating: rating, number_of_players: number_of_players };
   }
}

#[derive(Debug, Display, Error)]
enum ServiceError {
    #[display("failed to connect to DB.")]
    ConnectionFailure,
    #[display("Failed to find a game for specified id")]
    GameNotFound,
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
        };
    }
}
struct GameDataBase;
impl GameDataBase {
    
    async fn get_connection() -> Result<rusqlite::Connection, ServiceError> {
        println!("getting connection");
        return match Connection::open("kellum_library.db") {
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
                                    return Ok(Game{id,
                                                   title: title.to_string(),
                                                   platform: PlatformType::platform_from_string(&platform_string),
                                                   rating: ESRBRating::rating_from_string(&rating_string),
                                                   number_of_players: number});
                                }
                            }
                        }
                    }
                }
                return Err(rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Real,
                        Box::new(ServiceError::GameNotFound)));
        });
        println!("getting game");
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
            return Ok(Game{
                id: row.get::<usize, String>(0)?,
                title: row.get::<usize, String>(1)?,
                platform: PlatformType::platform_from_string(&row.get::<usize, String>(2)?),
                rating: ESRBRating::rating_from_string(&row.get::<usize, String>(3)?),
                number_of_players: row.get::<usize, u8>(4)?
            });
        });
        println!("getting game");
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
        println!("deleting game");
        return match result { Ok(_) => Ok(true), Err(_) => Ok(false) };
    }

    async fn update_game(updated_game: Game) -> Result<bool, ServiceError> {
        let conn = GameDataBase::get_connection().await?;
        let statement_result = conn.execute("UPDATE games 
                                             SET title=?1, platform=?2, rating=?3, number_of_players=?4 
                                             WHERE id=?5", 
                                             [updated_game.title, updated_game.platform.string(),
                                             updated_game.number_of_players.to_string()]);
        println!("updating game new");
        return match statement_result { Ok(_) => Ok(true), Err(_) => Ok(false) };
    }

    async fn insert_game(new_game: Game) -> Result<bool, ServiceError> {
        let conn = GameDataBase::get_connection().await?;
        let statement_result = conn.execute("INSERT INTO games (id, title, platform, rating, number_of_players) 
                                             VALUES (?1,?2,?3,?4,?5);", 
                                            [new_game.id, new_game.title, new_game.platform.string(),
                                            new_game.rating.string(),new_game.number_of_players.to_string()]);
        println!("receving game new");
        return match statement_result { Ok(_) => Ok(true), Err(_) => Ok(false) };
    }
}

#[post("/new")]
async fn add_game(new_game: Json<Game>) -> Result<impl Responder, ServiceError> {
    let real_new_game = Game::new(new_game.title.clone(), &new_game.platform.string(), 
                                  &new_game.rating.string(), new_game.number_of_players);
    let did_insert = GameDataBase::insert_game(real_new_game).await?;
    println!("receving game new");
    return Ok(HttpResponse::Ok().json(did_insert));
}

#[get("/all")]
async fn get_all_games() -> Result<impl Responder, ServiceError> {
    let resp = match GameDataBase::get_games().await? {
        Some(games) => HttpResponse::Ok().json(games),
        None => HttpResponse::NotFound().body("games not Found")
    };
    println!("game responding");
    return Ok(resp);
}

#[get("/{id}")]
async fn get_games(path: Path<(String,)>) -> Result<impl Responder, ServiceError> {
    let id = path.into_inner().0;
    let resp = match GameDataBase::get_game_with_id(id).await? {
        Some(game) => HttpResponse::Ok().json(game),
        None => HttpResponse::NotFound().body("games not Found")
    };
    println!("game responding");
    return Ok(resp);
}

#[put("/update")]
async fn update_game_with(updated_game: Json<Game>) -> Result<impl Responder, ServiceError> {
    let was_updated = GameDataBase::update_game(updated_game.into_inner()).await?;
    println!("game responding {}", was_updated);
    return Ok(HttpResponse::Ok().json(was_updated));
}

#[delete("/remove/{id}")]
async fn delete_game_with(path: Path<(String,)>) -> Result<impl Responder, ServiceError> {
    let id = path.into_inner().0;
    let was_deleted = GameDataBase::delete_game(Some(id)).await?;
    println!("game responding {}", was_deleted);
    return Ok(HttpResponse::Ok().json(was_deleted));
}

#[delete("/remove/all")]
async fn delete_all_games() -> Result<impl Responder, ServiceError> {
    let was_deleted = GameDataBase::delete_game(None).await?;
    println!("game responding {}", was_deleted);
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
        let scope = web::scope("/game")
            .service(add_game)
            .service(get_all_games)
            .service(get_games)
            .service(update_game_with)
            .service(delete_game_with)
            .service(delete_all_games);
        App::new().service(scope).wrap(from_fn(my_middleware))
    }).bind(("127.0.0.1", 8080))?.run().await
}
