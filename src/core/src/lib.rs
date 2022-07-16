use std::path::PathBuf;

pub struct Song {
    inner: PathBuf,
}

impl Song {
    pub fn new(inner: PathBuf) -> Self {
        Self { inner }
    }

    pub fn name(&self) -> String {
        self.inner
            .with_extension("")
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn path(&self) -> String {
        self.inner.display().to_string()
    }
}

#[cfg(test)]
mod tests {}

pub mod command;
pub mod error;
pub mod event;
pub mod utils;
