use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error as ThisError;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, ThisError)]
pub enum Error {
    #[error("Txn can't commit no open txn")]
    TxnCantCommitNoOpenTxn,
    #[error("Cannot begin txn with txn false")]
    CannotBeginTxnWithTxnFalse,
    #[error("Cannot commit txn with txn false")]
    CannotCommitTxnWithTxnFalse,
    #[error("No txn")]
    NoTxn,

    // -- Externals
    #[error("Sqlx Error {0:?}")]
    Sqlx(
        #[from]
        #[serde_as(as = "DisplayFromStr")]
        sqlx::Error,
    ),
}

// region:    --- Error Boilerplate
