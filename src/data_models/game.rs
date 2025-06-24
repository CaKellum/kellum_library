use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum PlatformType {
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
    pub fn platform_from_string(type_string: &str) -> Option<PlatformType> {
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

    pub fn string(self) -> String {
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum ESRBRating {
    Everyone,
    Everyone10,
    Teen,
    Mature,
    AdultOnly,
}

impl ESRBRating {
    pub fn rating_from_string(rating_string: &str) -> Option<ESRBRating> {
        return match rating_string {
            "Everyone" => Some(ESRBRating::Everyone),
            "Everyone10" => Some(ESRBRating::Everyone10),
            "Teen" => Some(ESRBRating::Teen),
            "Mature" => Some(ESRBRating::Mature),
            "AdultOnly" => Some(ESRBRating::AdultOnly),
            _ => None,
        };
    }

    pub fn string(self) -> String {
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
pub struct Game {
    pub id: String,
    pub title: String,
    pub platform: PlatformType,
    pub rating: ESRBRating,
    pub number_of_players: u8,
}

impl Game {
    pub fn new(title: String, platform: &str, rating: &str, number_of_players: u8) -> Option<Game> {
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
