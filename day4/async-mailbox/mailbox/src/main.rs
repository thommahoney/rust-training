use std::io;
use tokio::task;
use tokio::io::{AsyncBufReadExt,AsyncWriteExt,BufReader,BufWriter};
use tokio::net::{TcpListener,TcpStream};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;
use redisish;

#[derive(Debug)]
enum ServerError {
    ParseError(redisish::Error),
    IoError(std::io::Error),
}

impl From<redisish::Error> for ServerError {
    fn from(e: redisish::Error) -> ServerError {
        ServerError::ParseError(e)
    }
}

impl From<std::io::Error> for ServerError {
    fn from(e: std::io::Error) -> ServerError {
        ServerError::IoError(e)
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;

    let storage = VecDeque::new();
    let rced_storage: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(storage));

    loop {
        let (mut connection, _) = listener.accept().await?;

        let storage_handle = rced_storage.clone();

        task::spawn(async move {
            let res = handle(&mut connection, &storage_handle).await;

            if let Err(e) = res {
                println!("Error occured: {:?}", e);
            }
        });
    }
}

async fn handle(stream: &mut TcpStream, mutex: &Mutex<VecDeque<String>>) -> Result<(), ServerError> {
    let command = read_command(stream).await?;

    match command {
        redisish::Command::Publish(message) => {
            let mut storage = mutex.lock().await;
            storage.push_back(message);
        }
        redisish::Command::Retrieve => {
            let mut storage = mutex.lock().await;
            let data = storage.pop_front();
            match data {
                Some(message) => { 
                    let mut writer = BufWriter::new(stream);

                    // Write a byte to the buffer.
                    writer.write(message.as_bytes()).await?;

                    // Flush the buffer before it goes out of scope.
                    writer.flush().await?;
                },
                None => {
                    let mut writer = BufWriter::new(stream);

                    // Write a byte to the buffer.
                    writer.write("No message in inbox!".as_bytes()).await?;

                    // Flush the buffer before it goes out of scope.
                    writer.flush().await?;
                }
            };
        }
    }
    Ok(())
}

async fn read_command(stream: &mut TcpStream) -> Result<redisish::Command, ServerError> {
    let mut read_buffer = String::new();
    let mut buffered_stream = BufReader::new(stream);
    buffered_stream.read_line(&mut read_buffer).await?;
    let command = redisish::parse(&read_buffer)?;
    Ok(command)
}
