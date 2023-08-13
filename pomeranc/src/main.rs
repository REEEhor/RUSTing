use pomeranc::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::parse_arguments(&args).unwrap_or_else(|err| {
        println!("Invalid arguments: {}", err);
        process::exit(1);
    });

    if let Err(error) = pomeranc::run(&config) {
        println!("Application error: \x1b[31m{}\x1b[0m", error);
        process::exit(2);
    }
}
