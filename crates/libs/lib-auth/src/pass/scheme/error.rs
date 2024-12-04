// region: Error

use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error as ThisError;

pub type Result<T> = core::result::Result<T, Error>;

// region: --- Error boilerplate

#[serde_as]
#[derive(Debug, Serialize, ThisError)]
pub enum Error {
    #[error("Error with key")]
    Key,
    #[error("Error with salt")]
    Salt,
    #[error("Error with hash")]
    Hash,
    #[error("Error with password validation")]
    PassValidate,
    #[error("Error with scheme not found - cause: {0}")]
    SchemeNotFound(String),

    // --- IO error
    #[error("IO error")]
    Io(
        #[from]
        #[serde_as(as = "DisplayFromStr")]
        std::io::Error,
    ),
}

// endregion: --- Error boilerplate

// endregion: Error
