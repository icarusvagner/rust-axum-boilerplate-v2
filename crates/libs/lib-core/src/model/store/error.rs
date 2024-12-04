use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error as ThisError;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, ThisError)]
pub enum Error {
    #[error("Cannot create database postgres pool - cause: {0}")]
    CannotCreateDatabasePool(String),
    #[error("Database error occured: {0:?}")]
    DatabaseError(
        #[from]
        #[serde_as(as = "DisplayFromStr")]
        sqlx::Error,
    ),
    #[error(transparent)]
    Io(
        #[from]
        #[serde_as(as = "DisplayFromStr")]
        std::io::Error,
    ),
}
