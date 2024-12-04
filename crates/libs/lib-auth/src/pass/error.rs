// region: --- Error

use super::scheme;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error as ThisError;

pub type Result<T> = core::result::Result<T, Error>;

// region: --- Error boilerplate

#[serde_as]
#[derive(Debug, Serialize, ThisError)]
pub enum Error {
    #[error("Password with scheme failed to parse")]
    PassWithSchemeFailedToParse,

    #[error("Failed to spawn block for validation")]
    FailSpawnBlockForValidate,
    #[error("Failed to spawn block for hasing")]
    FailSpawnBlockForHash,

    // --- Modules
    #[error("Password scheme error")]
    Scheme(#[from] scheme::Error),

    // --- IO error
    #[error("IO error")]
    Io(
        #[from]
        #[serde_as(as = "DisplayFromStr")]
        std::io::Error,
    ),

    // --- Uuid Error
    #[error("UUID error")]
    UUIDError(
        #[from]
        #[serde_as(as = "DisplayFromStr")]
        uuid::Error,
    ),
}

// endregion: --- Error boilerplate

// endregion: --- Error
