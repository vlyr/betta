use crate::command::Command;
use crate::error::Error;
use std::path::PathBuf;

pub enum Event {
    FileDownloaded(PathBuf),
    Command(Command),
    Err(Error),
}
