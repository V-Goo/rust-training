use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contetnt = fs::read_to_string(filename)
        .expect("Ничего не понимаю!");
        
    println!("With text:\n{}", contetnt);

}
