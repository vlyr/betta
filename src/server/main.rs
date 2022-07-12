use betta_core::error::Result;
use betta_core::event::Event;
//use betta_core::utils::download_from_youtube;
use rodio::{Decoder, OutputStream, Sink};
use std::env;
use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Server {
    main_sink: Sink,
}

impl Server {
    pub fn new(sink: Sink) -> Self {
        Self { main_sink: sink }
    }
}

fn main() -> Result<()> {
    let path = Path::new("/tmp/betta_channel");

    if path.exists() {
        fs::remove_file(path)?;
    }

    let listener = UnixListener::bind("/tmp/betta_channel")?;

    let (main_tx, main_rx) = mpsc::channel::<Event>();

    env::set_current_dir("/home/vlyr/media/music/betta")?;

    /*thread::spawn(move || {
        download_from_youtube("https://www.youtube.com/watch?v=tV6Oe7FkQJc")
    });*/

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let server = Arc::new(Mutex::new(Server::new(sink)));

    thread::spawn(move || {
        event_handler(main_rx);
    });

    for stream in listener.incoming().filter_map(|s| s.ok()) {
        let server = Arc::clone(&server);
        let main_tx = Sender::clone(&main_tx);

        thread::spawn(move || {
            handle_stream(stream, server, main_tx).map_err(|e| {
                eprintln!("Error occurred in handle_stream - {}", e);
            })
        });
    }

    Ok(())
}

fn handle_stream(
    mut stream: UnixStream,
    server: Arc<Mutex<Server>>,
    main_tx: Sender<Event>,
) -> Result<()> {
    let file = BufReader::new(File::open("/home/vlyr/media/music/bangers/file.wav").unwrap());

    let src = Decoder::new(file).unwrap();

    let server = server.lock().unwrap();

    server.main_sink.append(src);
    server.main_sink.set_volume(0.5);
    server.main_sink.play();

    loop {
        let mut buffer = vec![0; 1024];

        stream.read(&mut buffer)?;

        let message = String::from_utf8(buffer).unwrap();
        println!("{}", message);

        stream.write(b"ACK")?;
        stream.write(b"\r\n")?;
        stream.flush()?;
    }
}

fn event_handler(main_rx: Receiver<Event>) {
    while let Ok(ev) = main_rx.recv() {}
}
