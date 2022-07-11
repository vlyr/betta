use betta_core::error::Result;
use cpal::traits::HostTrait;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;

pub struct Server {}

fn main() -> Result<()> {
    let listener = UnixListener::bind("/tmp/betta_channel")?;

    for stream in listener.incoming().filter_map(|s| s.ok()) {
        thread::spawn(move || {
            handle_stream(stream).map_err(|e| {
                eprintln!("Error occurred in handle_stream - {}", e);
            })
        });
    }

    Ok(())
}

fn handle_stream(mut stream: UnixStream) -> Result<()> {
    let host = cpal::default_host();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = BufReader::new(File::open("/home/vlyr/downloads/spa_milton.mp3").unwrap());

    let src = Decoder::new(file).unwrap();

    sink.append(src);

    sink.play();

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
