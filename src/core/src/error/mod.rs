use crate::event::Event;
use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, self::Error>;

#[derive(Debug)]
pub enum Error {
    // Error when downloading a song from YouTube
    // String - youtube-dl command output
    FileDownloadError(String),

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
            FileDownloadError(desc) => {
                format!("Error when downloading file from YouTube - {}", desc)
            }
            IO(e) => format!("{}", e),
        };
        write!(f, "{}", output)
    }
}

impl std::error::Error for Error {}
