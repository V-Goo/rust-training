use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contetnt = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contetnt);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Малавато будет");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duc";
        let contetnt = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contetnt));
    }
}
// ! мы говорим Rust, что данные, возвращаемые функцией search, будут жить до тех пор, пока живут данные, переданные в функцию search через аргумент contents
pub fn search<'a>(query: &str, contetnt: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();
	for line in contetnt.lines() {
		if line.contains(query) {
			results.push(line);
		}
	}
	results
}
