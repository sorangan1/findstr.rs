use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let line = "_________________________";

    let content = fs::read_to_string(config.file_path)?;

    println!("Text found:\n{line}\n{content}\n{line}");

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        return Ok(Config{query, file_path});
    }
}
