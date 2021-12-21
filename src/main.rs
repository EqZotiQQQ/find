mod tests;

use std::env;
use std::process;

use find::{Config, CustomError, find};

/**
 * find . -name "path"
 * find WHERE_TO_FIND [OPTIONS] WHAT_TO_FIND
 */
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::new(&args) {
        Ok(config) => {
            config
        }
        Err(not_enough_arg_error) => {
            match not_enough_arg_error {
                CustomError::NotEnoughArgumentError => {
                    eprintln!("{}", not_enough_arg_error);
                    process::exit(1);
                }
                CustomError::InvalidPathError => {
                    eprintln!("{}", not_enough_arg_error);
                    process::exit(2);
                }
            }

        }
    };

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

