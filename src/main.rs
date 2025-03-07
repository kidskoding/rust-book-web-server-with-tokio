use std::error::Error;
use tokio::fs;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (socket, _) = listener.accept().await?;
        println!("Connection established!");

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let buf_reader = BufReader::new(&mut stream);

    let mut http_request = Vec::new();
    let mut lines = buf_reader.lines();

    while let Some(line) = lines.next_line().await? {
        if line.is_empty() {
            break;
        }
        http_request.push(line);
    }

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("index.html").await?;
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).await?;

    Ok(())
}
