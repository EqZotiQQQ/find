mod tests;

use std::env;
use std::process;

use find::{Config, find};

/**
 * find . -name "path"
 * find WHERE_TO_FIND [OPTIONS] WHAT_TO_FIND
 */
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(
        |err| {
            println!("Failed to parse arguments: {}", err);
            process::exit(1);
        }
    );

    let results = find(config).unwrap_or_else(
        |err| {
            println!("No such directory: {}", err);
            process::exit(2);
        }
    );

    for result in results {
        println!("{}", result);
    }
}

