use crate::data_models::movie::{MPAARating, MotionPictureFormat, Movie};
use crate::database_services::database_utilities::get_connection;
use crate::ServiceError;
use std::usize;

pub struct MovieDataBase;
impl MovieDataBase {
    pub async fn new_movie_with(new_movie: Movie) -> Result<bool, ServiceError> {
        let conn = get_connection()?;
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

    pub async fn get_movie_with_id(id: String) -> Result<Option<Movie>, ServiceError> {
        let conn = get_connection()?;
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

    pub async fn get_all_movies() -> Result<Option<Vec<Movie>>, ServiceError> {
        let conn = get_connection()?;
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

    pub async fn update_movie_with(new_movie: Movie) -> Result<bool, ServiceError> {
        let conn = get_connection()?;
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

    pub async fn delete_movie(id: Option<String>) -> Result<bool, ServiceError> {
        let conn = get_connection()?;
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
