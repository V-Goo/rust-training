use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.01:7878").unwrap();
#[allow(unused_variables)]
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Присоединяйся!");
    }
}

