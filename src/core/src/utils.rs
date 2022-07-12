use crate::error::{Error, Result};
use crate::event::Event;
use std::process::Command;
use std::sync::mpsc::Sender;

pub fn download_from_youtube<U>(url: U, sender: Sender<Event>) -> Result<()>
where
    U: AsRef<str>,
{
    let output = match Command::new("youtube-dl")
        .args(["-f", "bestaudio", "-x", url.as_ref()])
        .output()
    {
        Ok(_) => Event::FileDownloaded,
        Err(e) => Event::Err(Error::FileDownloadError(e.to_string())),
    };

    sender.send(output).unwrap();

    Ok(())
}
