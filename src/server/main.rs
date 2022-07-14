use betta_core::command::Command;
use betta_core::error::Result;
use betta_core::event::Event;
use betta_core::utils::download_from_youtube;
use rodio::queue::SourcesQueueInput;
use rodio::source::Source;
use rodio::{Decoder, OutputStream};
use std::env;
use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Server {
    input_stream: Arc<SourcesQueueInput<f32>>,
}

impl Server {
    pub fn new(input: Arc<SourcesQueueInput<f32>>) -> Self {
        Self {
            input_stream: input,
        }
    }
}

fn main() -> Result<()> {
    let path = Path::new("/tmp/betta_channel");

    if path.exists() {
        fs::remove_file(path)?
    }

    let listener = UnixListener::bind("/tmp/betta_channel")?;

    let (main_tx, main_rx) = mpsc::channel::<Event>();
    let main_tx_clone = main_tx.clone();

    env::set_current_dir("/home/vlyr/media/music/betta")?;

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let (input, output) = rodio::queue::queue(true);

    // TODO: fix .unwrap()
    stream_handle.play_raw(output).unwrap();

    let server = Arc::new(Mutex::new(Server::new(input)));
    let server_clone = Arc::clone(&server);

    thread::spawn(move || {
        event_handler(main_rx, main_tx_clone, server_clone);
    });

    for stream in listener.incoming().filter_map(|s| s.ok()) {
        let main_tx = Sender::clone(&main_tx);

        thread::spawn(move || {
            handle_stream(stream, main_tx).map_err(|e| {
                eprintln!("Error occurred in handle_stream - {}", e);
            })
        });
    }

    Ok(())
}

fn handle_stream(mut stream: UnixStream, main_tx: Sender<Event>) -> Result<()> {
    main_tx
        .send(Event::Command(Command::SetVolume(50)))
        .unwrap();

    loop {
        let mut buffer = vec![0; 1024];
        stream.read(&mut buffer)?;
        buffer.retain(|byte| *byte != u8::MIN);

        let message = String::from_utf8(buffer).unwrap();
        let cmd = Command::from_args(message.split(' '))?;
        main_tx.send(Event::Command(cmd)).unwrap();

        stream.write(b"ACK")?;
        stream.flush()?;
    }
}

fn event_handler(main_rx: Receiver<Event>, main_tx: Sender<Event>, server: Arc<Mutex<Server>>) {
    let mut song_control_tx: Option<Sender<Command>> = None;
    let mut stop_signal: Option<Receiver<()>> = None;

    loop {
        if let Some(ref rx) = stop_signal {
            if let Ok(_sig) = rx.try_recv() {}
        }

        if let Ok(event) = main_rx.recv() {
            let server = server.lock().unwrap();

            match event {
                Event::Command(cmd) => match cmd {
                    Command::Play(path) => {
                        let file = BufReader::new(File::open(path).unwrap());

                        let (tx, rx) = mpsc::channel();
                        song_control_tx = Some(tx);

                        let src = Decoder::new(file)
                            .unwrap()
                            .amplify(0.5)
                            .pausable(false)
                            .stoppable()
                            .periodic_access(Duration::from_millis(200), move |src| {
                                if let Ok(cmd) = rx.try_recv() {
                                    println!("{:#?}", src.inner_mut().inner_mut().total_duration());

                                    match cmd {
                                        Command::Pause => src.inner_mut().set_paused(true),
                                        Command::Resume => src.inner_mut().set_paused(false),
                                        Command::SetVolume(vol) => {
                                            src.inner_mut()
                                                .inner_mut()
                                                .set_factor(vol as f32 / 100.0);
                                        }
                                        _ => (),
                                    }
                                }
                            })
                            .convert_samples();

                        stop_signal = Some(server.input_stream.append_with_signal(src));
                    }

                    Command::Download(url) => {
                        let sender = main_tx.clone();

                        thread::spawn(move || match download_from_youtube(url) {
                            Ok(file_path) => sender.send(Event::FileDownloaded(file_path)),
                            Err(e) => sender.send(Event::Err(e)),
                        });
                    }

                    cmd => {
                        if let Some(ref tx) = song_control_tx {
                            tx.send(cmd).unwrap();
                        };
                    }
                },
                _ => (),
            }
        }
    }
}
