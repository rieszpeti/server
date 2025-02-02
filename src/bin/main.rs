use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

use server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5363").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
    
        pool.execute( || {
            handle_connection(stream);
        });
    }

    println!("Shutting down.")
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body
    //
    // ex: HTTP/1.1 200 OK\r\n\r\n

    let (status_line, filename) = 
        if buffer.starts_with(get)  {
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}