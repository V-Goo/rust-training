use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.01:7878").unwrap();
#[allow(unused_variables)]
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // println!("Присоединяйся!");

        handle_connection(stream);
        
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) =  if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let content = fs::read_to_string(filename).unwrap();  

//     let response = format!(
//     "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
//     content.len(),
//     content
// );

    // stream.write(response.as_bytes()).unwrap();
    // stream.flush().unwrap();

    // let status_line = "HTTP/1.1 404 NOT FOUND";
    // let content = fs::read_to_string("404.html").unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


