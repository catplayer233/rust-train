use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::thread_pool::ThreadPool;

mod thread_pool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8099").unwrap();

    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    //the byte buffer
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get_request = b"GET / HTTP/1.1\r\n";

    let (status_line, content_file) = if buffer.starts_with(get_request) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let response = format!("{}{}", status_line, fs::read_to_string(content_file).unwrap());

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
