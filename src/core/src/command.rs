#[derive(Debug, Clone)]
pub enum Command {
    Play,
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
            Play => "play".into(),
            Resume => "resume".into(),
            GetVolume => "vol".into(),
            SetVolume(num) => format!("vol {}", num),
            _ => "overview".into(),
        }
    }
}

impl<S, I> From<I> for Command
where
    S: AsRef<str>,
    I: Iterator<Item = S>,
{
    fn from(mut data: I) -> Self {
        use Command::*;
        match data.next() {
            Some(arg) => match arg.as_ref() {
                "play" => Play,
                "pause" => Pause,
                "resume" => Resume,
                "vol" => match data.next() {
                    Some(vol) => match vol.as_ref().parse::<u32>() {
                        Ok(num) => SetVolume(num),
                        Err(_) => GetVolume,
                    },

                    None => GetVolume,
                },
                _ => Overview,
            },
            _ => Overview,
        }
    }
}
