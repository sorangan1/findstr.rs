use std::{env, process};
use rust_findstr::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for => '{}'", config.query);
    println!("In file => [{}]", config.file_path);

    if let Err(e) = rust_findstr::run(config) {
        println!("App error: {e}");
        process::exit(1);
    }
}
