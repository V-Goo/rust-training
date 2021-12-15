use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contetnt = fs::read_to_string(config.filename).expect("Ничего не понимаю!");

    println!("With text:\n{}", contetnt);
}

struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Малавато будет")
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
