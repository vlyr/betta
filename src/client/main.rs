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

    let command = match Command::from_args(args) {
        Ok(cmd) => cmd,
        Err(e) => {
            println!("Error when creating command - {}", e);
            process::exit(1);
        }
    };

    //loop {
    stream.write(command.to_string().as_bytes())?;
    stream.flush()?;

    let mut buf = vec![0; 1024];
    stream.read(&mut buf)?;

    println!("{}", String::from_utf8(buf).unwrap());
    //}
    Ok(())
}
