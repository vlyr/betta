use betta_core::command::Command;
use betta_core::error::Result;
use std::env;
use std::io::Read;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::process;

fn main() -> Result<()> {
    let mut stream = match UnixStream::connect("/tmp/betta_channel") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to connect - {}", e);
            process::exit(1);
        }
    };

    let mut args = env::args();
    args.next();

    let command = Command::from(args);

    //loop {
    stream.write(command.to_string().as_bytes())?;
    stream.flush()?;

    let mut buf = vec![0; 1024];
    stream.read(&mut buf)?;

    println!("{}", String::from_utf8(buf).unwrap());
    //}
    Ok(())
}
