pub fn b32(content: impl AsRef<[u8]>) -> String {
    data_encoding::BASE32.encode(content.as_ref())
}

pub fn b32u(content: impl AsRef<[u8]>) -> String {
    data_encoding::BASE32_NOPAD.encode(content.as_ref())
}

pub fn b32dnssec(content: impl AsRef<[u8]>) -> String {
    data_encoding::BASE32_DNSSEC.encode(content.as_ref())
}

pub fn b32dncurve(content: impl AsRef<[u8]>) -> String {
    data_encoding::BASE32_DNSCURVE.encode(content.as_ref())
}

pub fn b32hex(content: impl AsRef<[u8]>) -> String {
    data_encoding::BASE32HEX.encode(content.as_ref())
}

pub fn b32hex_u(content: impl AsRef<[u8]>) -> String {
    data_encoding::BASE32HEX_NOPAD.encode(content.as_ref())
}

// region: --- Error

pub type Result<T> = core::result::Result<T, Error>;

// region: --- Error boilerplate

#[derive(Debug)]
pub enum Error {
    FailToDecodeB32,
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error boilerplate

// endregion: --- Error
