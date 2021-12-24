use std::path::{Path, PathBuf};
use clap::Parser;

pub use super::errors::custom_errors::CustomError;

#[derive(Parser, Debug)]
pub struct AppArgs {
    pub path: String,
    pub target: String,
}

pub struct Config {
    pub path: PathBuf,
    pub target: String,
}

impl Config {
    pub fn new(args: AppArgs) -> Result<Config, CustomError> {
        if !Path::exists(args.path.as_ref()) {
            return Err(CustomError::InvalidPathError);
        }
        Ok(Config {
            path: PathBuf::from(args.path),
            target: args.target,
        })
    }
}

pub fn find(config: Config) -> Vec<String> {
    let mut results: Vec<String> = vec![];
    search_depth(&config.path, &mut results, &config);
    return results
}

fn search_depth(path: &PathBuf, results: &mut Vec<String>, config: &Config) {
    match path.read_dir() {
        Ok(red_dir) => {
            red_dir.for_each(|file| {
                if let Ok(dir_entry) = file {
                    dir_entry.path().is_dir().then(|| {
                        search_depth(&dir_entry.path(), results, config);
                    });
                    if let Some(name) = dir_entry.path().to_str() {
                        name.contains(&config.target).then(|| {
                            results.push(String::from(dir_entry.path().to_str().unwrap()));
                        });
                    }
                    // dir_entry.file_name().to_str().unwrap().equivalent(&config.target).then(||
                    //     results.push(String::from(dir_entry.path().to_str().unwrap()))
                    // );
                    // if let Some(name) = dir_entry.file_name().to_str() {
                    //     name.contains(&config.target).then(|| {
                    //         results.push(String::from(dir_entry.path().to_str().unwrap()));
                    //     });
                    // }
                }
            })
        }
        Err(read_dir_err) => {
            eprintln!("{}", read_dir_err);
        }
    }
}


    


