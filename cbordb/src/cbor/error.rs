
#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    IncorrectType,
    TooShort,
    NumberTooBig,
    InvalidCode,
    Infinite,
    NonUFT8String,
    FormatError(std::fmt::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::fmt::Error> for Error {
    fn from(e: std::fmt::Error) -> Self {
        Error::FormatError(e)
    }
}