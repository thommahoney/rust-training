use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;
use redisish;

fn handle_client(mut stream: TcpStream, storage: &mut VecDeque<String>) -> Result<(), io::Error> {
    let mut string = String::new();
    stream.read_to_string(&mut string)?;

    // use redisish parse function and store
    match redisish::parse(string.as_str()) {
        Ok(command) => {
            match command {
                redisish::Command::Publish(msg) => {
                    storage.push_back(msg);
                    Ok(())
                },
                redisish::Command::Retrieve => {
                    match storage.pop_front() {
                        Some(s) => {
                            stream.write_all(s.as_bytes())
                        },
                        None => {
                            write!(stream, "Nothing to see here!")
                        }
                    }
                },
            }

        },
        Err(e) => {
            write!(stream, "error: {:?}", e)
        }
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    let mut storage: VecDeque<String> = VecDeque::new();

    // accept connections and process them serially
    for stream in listener.incoming() {
        match handle_client(stream?, &mut storage) {
            Ok(_) => {},
            Err(e) => { eprintln!("{:?}", e) },
        };
    }
    Ok(())
}
