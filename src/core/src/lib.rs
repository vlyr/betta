pub struct Song {
    name: String,
    artist: Option<String>,
}

#[cfg(test)]
mod tests {}

pub mod command;
pub mod error;
pub mod event;
pub mod utils;
