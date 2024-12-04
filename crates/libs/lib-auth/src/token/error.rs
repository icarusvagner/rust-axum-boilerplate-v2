use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    HmacFailNewFromSlice,

    InvalidFormat,
    CannotDecodeIdent,
    CannotDecodeExp,
    SignatureNotMatching,
    ExpNotIso,
    Expired,
}

// region: --- Error boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error boilerplate
