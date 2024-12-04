use serde::Serialize;
use thiserror::Error as ThisError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, ThisError)]
pub enum Error {
    #[error("Context cannot new root context")]
    CtxCannotNewRootCtx,
}
