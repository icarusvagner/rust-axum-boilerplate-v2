use std::borrow::Cow;

use lib_auth::pass;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use sqlx::error::DatabaseError;
use thiserror::Error as ThisError;

use super::store::dbx;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, ThisError)]
pub enum Error {
    #[error("OffsetDateTime TimeErrorComponentRange - cause: {0}")]
    TimeErrorComponentRange(
        #[from]
        #[serde_as(as = "DisplayFromStr")]
        time::error::ComponentRange,
    ),
    #[error("Chrono parsing error - cause: {0}")]
    ChronoError(
        #[from]
        #[serde_as(as = "DisplayFromStr")]
        chrono::ParseError,
    ),
    #[error("Birth date is in invalid format")]
    InvalidBirthdateFormat,
    #[error("Database entity not found - entity: {entity:?} - id: {id:?}")]
    EntityNotFound { entity: &'static str, id: i64 },
    #[error("List limit over max - max: {max:?} - actual: {actual:?}")]
    ListLimitOverMax { max: i64, actual: i64 },
    #[error("Count failed")]
    CountFail,
    #[error("Admin already exist in our records - username: {username:?}")]
    AdminAlreadyExists { username: String },
    #[error("Database unique violation occur - table: {table:?} - constraint: {constraint:?}")]
    UniqueViolation { table: String, constraint: String },
    #[error("Cannot create model manager provider: {0}")]
    CantCreateModelManagerProvider(String),
    #[error("Password error fail")]
    Pwd(#[from] pass::Error),

    // -- Externals
    #[error("Sqlx error - cause: {0:?}")]
    DatabaseError(
        #[from]
        #[serde_as(as = "DisplayFromStr")]
        sqlx::Error,
    ),
    #[error("SEA Query error - cause: {0:?}")]
    SeaQueryError(
        #[from]
        #[serde_as(as = "DisplayFromStr")]
        sea_query::error::Error,
    ),
    #[error("IO error - cause: {0:#?}")]
    Io(
        #[from]
        #[serde_as(as = "DisplayFromStr")]
        std::io::Error,
    ),
    #[error("Dbx error - cause: {0:?}")]
    Dbx(#[from] dbx::Error),

    // CRUD Models error
    #[error("Account removed: {uname} email: {email} id: {id}")]
    AccountIsRemoved {
        uname: String,
        email: String,
        id: i64,
    },
    #[error("Admin is already removed: admin_id: {admin_id}")]
    AdminAlreadyRemoved { admin_id: i64 },
    #[error("Insert failed: {entity} cause: {cause:?}")]
    InsertionFailed { entity: String, cause: String },
    #[error("Selection failed: {entity} cause: {cause:?}")]
    SelectionFailed { entity: String, cause: String },
    #[error("Deletion failed: {entity} cause: {cause:?}")]
    DeletionFailed { entity: String, cause: String },
    #[error("Updating failed: {entity} cause: {cause:?}")]
    UpdatingFailed { entity: String, cause: String },
    #[error("Username not found: {entity} cause: {cause:?}")]
    UsernameNotFound { entity: String, cause: String },
    #[error("Username is already in the record: {uname}")]
    UsernameAlreadyExists { uname: String },

    // General CID and MID
    #[error("Updating cid and mid failed - entity: {entity}, gen_id: {gen_id}")]
    CannotUpdateCidMid { entity: String, gen_id: i64 },
}

impl Error {
    /// This function will transform the error into a more precise variant if it is an SQLX or PGError Unique Violation.
    /// The resolver can contain a function (table_name: &str, constraint: &str) that may return a specific Error if desired.
    /// If the resolver is None, or if the resolver function returns None, it will default to Error::UniqueViolation {table, constraint}.
    pub fn resolve_unique_violation<F>(self, resolver: Option<F>) -> Self
    where
        F: FnOnce(&str, &str) -> Option<Self>,
    {
        match self
            .as_database_error()
            .map(|db_error| (db_error.code(), db_error.table(), db_error.constraint()))
        {
            // "23505" => postgresql "unique violation"
            Some((Some(Cow::Borrowed("23505")), Some(table), Some(constraint))) => resolver
                .and_then(|fun| fun(table, constraint))
                .unwrap_or_else(|| Error::UniqueViolation {
                    table: table.to_string(),
                    constraint: constraint.to_string(),
                }),
            _ => self,
        }
    }

    /// A convenient function to return the eventual database error (Postgres)
    /// if this Error is an SQLX Error that contains a database error.
    pub fn as_database_error(&self) -> Option<&(dyn DatabaseError + 'static)> {
        match self {
            Error::Dbx(dbx::Error::Sqlx(sqlx_error)) => sqlx_error.as_database_error(),
            _ => None,
        }
    }
}
