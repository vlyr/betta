use betta_core::command::Command;
use betta_core::error::Result;
use betta_core::event::Event;
use betta_core::utils::download_from_youtube;
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

mod server;
use server::Server;

mod database;
use database::Database;

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
        event_handler(main_rx, main_tx_clone, server_clone)
            .map_err(|e| panic!("{}", e))
            .unwrap();
    });

    for stream in listener.incoming().filter_map(|s| s.ok()) {
        let main_tx = Sender::clone(&main_tx);

        let server_clone = server.clone();

        thread::spawn(move || {
            handle_stream(stream, main_tx, server_clone).map_err(|e| {
                eprintln!("Error occurred in handle_stream - {}", e);
            })
        });
    }

    Ok(())
}

fn handle_stream(
    mut stream: UnixStream,
    main_tx: Sender<Event>,
    server: Arc<Mutex<Server>>,
) -> Result<()> {
    loop {
        let mut buffer = vec![0; 1024];
        stream.read(&mut buffer)?;
        buffer.retain(|byte| *byte != u8::MIN);

        let message = String::from_utf8(buffer).unwrap();

        if message.is_empty() {
            continue;
        }

        let cmd = Command::from_args(message.split(' '))?;

        match cmd {
            Command::Overview => {
                let server_lock = server.lock().unwrap();

                let queue = server_lock.queue();

                match queue.is_empty() {
                    true => stream.write(b"Queue is empty"),

                    false => stream.write(
                        queue
                            .iter()
                            .map(|p| p.name())
                            .collect::<Vec<_>>()
                            .join("\n")
                            .as_bytes(),
                    ),
                }
                .unwrap();
            }

            cmd => {
                main_tx.send(Event::Command(cmd)).unwrap();
            }
        }

        stream.write(&[0x06])?;

        stream.flush()?;
    }
}

fn event_handler(
    main_rx: Receiver<Event>,
    main_tx: Sender<Event>,
    server: Arc<Mutex<Server>>,
) -> Result<()> {
    let mut song_control_tx: Option<Sender<Command>> = None;

    loop {
        if let Ok(event) = main_rx.recv() {
            let mut server = server.lock().unwrap();

            match event {
                Event::SongFinished => {
                    if let Some(next_song) = server.queue_mut().pop_front() {
                        main_tx
                            .send(Event::Command(Command::Play(next_song.path())))
                            .unwrap();
                    }
                }

                Event::Command(cmd) => match cmd {
                    Command::Play(ref path) => {
                        if let Some(ref tx) = song_control_tx {
                            if let Err(e) = tx.send(Command::Stop) {
                                eprintln!("{}", e);
                            }
                        }
                        let file = match Path::new(&path).is_dir() {
                            true => {
                                server.queue_directory(path)?;
                                let first_song = server.queue_mut().pop_front().unwrap();

                                BufReader::new(File::open(first_song.path())?)
                            }

                            false => BufReader::new(File::open(path)?),
                        };

                        let (tx, rx) = mpsc::channel();
                        song_control_tx = Some(tx);

                        let src = Decoder::new(file)
                            .unwrap()
                            .amplify(0.5)
                            .pausable(false)
                            .stoppable()
                            .periodic_access(Duration::from_millis(200), move |src| {
                                if let Ok(cmd) = rx.try_recv() {
                                    match cmd {
                                        Command::Pause => src.inner_mut().set_paused(true),
                                        Command::Resume => src.inner_mut().set_paused(false),
                                        Command::Stop => src.stop(),
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

                        let stop_signal_rx = server.audio_input().append_with_signal(src);
                        let stop_signal_tx = main_tx.clone();

                        thread::spawn(move || {
                            stop_signal_rx.recv().ok();
                            stop_signal_tx.send(Event::SongFinished).unwrap();
                        });
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
