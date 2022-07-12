use crate::error::Error;

pub enum Event {
    FileDownloaded,
    Err(Error),
}
