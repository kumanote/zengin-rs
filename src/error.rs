use std::io;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("IO error: {cause:?}")]
    Io { cause: io::Error },

    #[error("Deserialize error: {cause:?}")]
    Deserialize { cause: serde_json::Error },
}

impl From<io::Error> for Error {
    fn from(cause: io::Error) -> Error {
        Error::Io { cause }
    }
}

impl From<serde_json::Error> for Error {
    fn from(cause: serde_json::Error) -> Error {
        Error::Deserialize { cause }
    }
}
