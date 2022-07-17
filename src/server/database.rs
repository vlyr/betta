use betta_core::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Playlist {
    // Vec of paths
    songs: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Database {
    playlists: Vec<Playlist>,
}

impl Database {
    pub fn read_from_file<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut content = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(path.as_ref())?;

        let mut buffer = String::new();

        content.read_to_string(&mut buffer)?;

        Ok(serde_json::from_str(&buffer).unwrap())
    }
}
