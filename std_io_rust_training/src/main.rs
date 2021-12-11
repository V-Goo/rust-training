    use std::io;
use std::io::prelude::*;
use std::fs::File;

fn test() -> io::Result<()> {
    let mut file = File::create("foo.txt")?;// Файл создаётся в режиме "только запись"
    file.write_all(b"Hello, world!")?; // "?" -- https://ru.stackoverflow.com/a/765311
    let mut f = File::open("foo.txt")?;// expression? - это сокращенная запись для try!(expression)
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer)?;

    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}

fn main() {
    println!("{:?}", test());
}