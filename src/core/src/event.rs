use crate::command::Command;
use crate::error::Error;

pub enum Event {
    FileDownloaded,
    Command(Command),
    Err(Error),
}
