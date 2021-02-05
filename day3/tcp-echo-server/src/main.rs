use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::Read;
use std::io::Write;

fn handle_client(mut stream: TcpStream) -> Result<(), io::Error> {
    let mut string: String = String::new();
    stream.read_to_string(&mut string)?;
    println!("{:?}", string);
    stream.write_all(&string.as_bytes())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        match handle_client(stream?) {
            Ok(_) => {},
            Err(e) => { eprintln!("{:?}", e) },
        };
    }
    Ok(())
}
