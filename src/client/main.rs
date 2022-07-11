use betta_core::error::Result;
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

    //loop {
    stream.write(b"Hello")?;
    stream.flush()?;

    let mut buf = vec![0; 1024];
    stream.read(&mut buf)?;

    println!("{}", String::from_utf8(buf).unwrap());
    //}

    std::thread::sleep(std::time::Duration::from_millis(10000));
    Ok(())
}
