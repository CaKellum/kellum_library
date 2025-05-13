use actix_web::{delete, get, post, put, web::{ self, Path, Json}, App, HttpResponse, HttpServer, Responder};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
enum PlatformType {
   Playstation1, Playstation2, Playstation3, Playstation4, Playstation5,
   NES, SNES, N64, GameCube, Wii, WiiU, Switch, Switch2,
   NintendoDS, Nintendo3DS,
   Computer
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

#[post("/new")]
async fn add_game(new_game: Json<Game>) -> impl Responder {
    let real_new_game = Game::new(new_game.title.clone(), &new_game.platform.string(), 
                                  &new_game.rating.string(), new_game.number_of_players);
    return HttpResponse::Ok().body("added Game {real_new_game.id}");
}

#[get("/all")]
async fn get_all_games() -> impl Responder {
    return HttpResponse::Ok().body("all games");
}

#[get("/{id}")]
async fn get_games(path: Path<(String,)>) -> impl Responder {
    let id = path.into_inner().0;
    return HttpResponse::Ok().body("get game with {id}");
}

#[put("/update")]
async fn update_game_with(updated_game: Json<Game>) -> impl Responder {
    return HttpResponse::Ok().body("update game {update_game.id}");
}

#[delete("/remove/{id}")]
async fn delete_game_with(path: Path<(String,)>) -> impl Responder {
    return HttpResponse::Ok().body("delete game");
}

#[delete("/remove/all")]
async fn delete_all_games() -> impl Responder {
    return HttpResponse::Ok().body("delete all games");
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
        App::new().service(scope)
    }).bind(("127.0.0.1", 8080))?.run().await
}
