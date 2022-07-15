use betta_core::command::Command;
use betta_core::error::Result;
use std::env;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;
use std::process;

fn main() -> Result<()> {
    let stream = match UnixStream::connect("/tmp/betta_channel") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to connect - {}", e);
            process::exit(1);
        }
    };

    let mut args = env::args();
    args.next();

    let command = match Command::from_args(args.collect::<Vec<_>>().iter()) {
        Ok(cmd) => cmd,
        Err(e) => {
            println!("Error when creating command - {}", e);
            process::exit(1);
        }
    };

    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    print!("{}", command.to_string());

    writer.write(format!("{}\n", command.to_string()).as_bytes())?;
    writer.flush()?;

    let mut buffer = vec![];
    reader.read_until(0x06, &mut buffer).unwrap();

    println!("{}", String::from_utf8(buffer).unwrap());

    reader.into_inner().shutdown(Shutdown::Both).unwrap();

    Ok(())
}
