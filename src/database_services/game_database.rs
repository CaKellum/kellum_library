use crate::data_models::game::{ESRBRating, Game, PlatformType};
use crate::database_services::database_utilities::get_connection;
use crate::ServiceError;

pub struct GameDataBase;
impl GameDataBase {
    pub async fn get_game_with_id(id: String) -> Result<Option<Game>, ServiceError> {
        let conn = get_connection()?;
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

    pub async fn get_games() -> Result<Option<Vec<Game>>, ServiceError> {
        let conn = get_connection()?;
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

    pub async fn delete_game(id: Option<String>) -> Result<bool, ServiceError> {
        let conn = get_connection()?;
        let result = match id {
            Some(id) => conn.execute("DELETE FROM games WHERE id=?1", [id]),
            None => conn.execute("DROP TABLE games", []),
        };
        return match result {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        };
    }

    pub async fn update_game(updated_game: Game) -> Result<bool, ServiceError> {
        let conn = get_connection()?;
        let statement_result = conn.execute("UPDATE games 
                                             SET title=?1, platform=?2, rating=?3, number_of_players=?4 
                                             WHERE id=?5", 
                                             [updated_game.title, updated_game.platform.string(),
                                             updated_game.number_of_players.to_string()]);
        return match statement_result {
            Ok(rows_updated) => {
                if rows_updated > 0 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(_) => Err(ServiceError::FailedToUpdateGame),
        };
    }

    pub async fn insert_game(new_game: Game) -> Result<bool, ServiceError> {
        let conn = get_connection()?;
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
