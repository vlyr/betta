use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub enum Command {
    Play(String),
    Download(String),
    Pause,
    Resume,
    VolumeUp(u32),
    VolumeDown(u32),
    SetVolume(u32),
    GetVolume,
    Overview,
}

impl ToString for Command {
    fn to_string(&self) -> String {
        use Command::*;

        match self {
            Pause => "pause".into(),
            Play(path) => format!("play {}", path),
            Download(url) => format!("download {}", url),
            Resume => "resume".into(),
            GetVolume => "vol".into(),
            SetVolume(num) => format!("vol {}", num),
            _ => "overview".into(),
        }
    }
}

impl Command {
    pub fn from_args<S, I>(mut data: I) -> Result<Self>
    where
        S: AsRef<str>,
        I: Iterator<Item = S>,
    {
        use Command::*;

        match data.next() {
            Some(arg) => match arg.as_ref() {
                "play" => match data.next() {
                    Some(p) => Ok(Play(p.as_ref().to_string())),
                    None => Err(Error::InvalidCommandArguments),
                },

                "download" => match data.next() {
                    Some(url) => Ok(Download(url.as_ref().to_string())),
                    None => Err(Error::InvalidCommandArguments),
                },

                "pause" => Ok(Pause),
                "resume" => Ok(Resume),

                "vol" => match data.next() {
                    Some(vol) => match vol.as_ref().parse::<u32>() {
                        Ok(num) => Ok(SetVolume(num)),
                        Err(_) => Err(Error::InvalidCommandArguments),
                    },

                    None => Ok(GetVolume),
                },
                _ => Ok(Overview),
            },
            _ => Ok(Overview),
        }
    }
}
