use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, self::Error>;

#[derive(Debug)]
pub enum Error {
    // Invalid download URL provided for a song
    InvalidURL,

    // IO error
    IO(io::Error),
}

impl From<io::Error> for Error {
    fn from(data: io::Error) -> Self {
        Self::IO(data)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        let output = match self {
            InvalidURL => "Invalid URL".into(),
            IO(e) => format!("{}", e),
        };
        write!(f, "{}", output)
    }
}

impl std::error::Error for Error {}
