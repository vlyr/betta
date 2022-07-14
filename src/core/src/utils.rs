use crate::error::{Error, Result};
use crate::event::Event;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn download_from_youtube<U>(url: U) -> Result<PathBuf>
where
    U: AsRef<str>,
{
    // Check current directory contents for checking the filename of the recently downloaded file
    // later
    let current_dir = std::env::current_dir()?;
    let dir_read = fs::read_dir(&current_dir)?;

    let prev_dir_contents: Vec<_> = dir_read
        .filter_map(|result| result.ok())
        .map(|file| file.path())
        .collect();

    if let Err(e) = Command::new("youtube-dl")
        .args(["-f", "bestaudio", "-x", url.as_ref()])
        .output()
    {
        return Err(Error::FileDownloadError(e.to_string()));
    };

    let new_dir_read = fs::read_dir(current_dir)?;

    let new_file_path = new_dir_read
        .filter_map(|f| f.ok())
        .filter(|f| !prev_dir_contents.contains(&f.path()))
        .next();

    match new_file_path {
        Some(p) => {
            println!("{}", p.path().to_str().unwrap());
            Ok(p.path())
        }

        None => Err(Error::FileDownloadError(
            "Downloaded file does not exist in target directory".into(),
        )),
    }
}
