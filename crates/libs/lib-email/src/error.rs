use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use derive_more::derive::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use std::sync::Arc;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- Email smtp error
    EmailProviderMissing(String),
    EmailPortParsingError,

    SomethingWentWrong,
    SendingEmailFailed(String),
    SMTPServerConnectFailed(String),

    // -- External Modules
    #[from]
    SerdeJson(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
}

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("{:<12} - model::Error {self:?}", "INTO_RES");

        // Create a placeholder Axum reponse.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the reponse.
        response.extensions_mut().insert(Arc::new(self));

        response
    }
}
// endregion: --- Axum IntoResponse

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

// region:    --- Client Error

/// From the root error to the http status code and ClientError
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use Error::*; // TODO: should change to `use web::Error as E`

        match self {
            // -- Auth
            SomethingWentWrong => (StatusCode::FORBIDDEN, ClientError::SOMETHING_WENT_WRONG),

            // -- Fallback.
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
    SOMETHING_WENT_WRONG,
    SERVICE_ERROR,
}
// endregion: --- Client Error
