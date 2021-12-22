use std::path::Path;
pub use crate::path_errors::custom_errors::CustomError;

pub mod path_errors;
mod tests;


pub struct Config<'a> {
    pub path: &'a Path,
    pub target: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a Vec<String>) -> Result<Config, CustomError> {
        if args.len() < 3 {
            return Err(CustomError::NotEnoughArgumentError);
        }
        if !Path::exists(args[1].as_ref()) {
            return Err(CustomError::InvalidPathError);
        }
        Ok(Config {
            path: args[1].as_ref(),
            target: args[2].as_str(),
        })
    }
}

pub fn find(config: Config) -> Vec<String> {
    let mut results: Vec<String> = vec![];
    search_depth(config.path, &mut results, &config);
    return results
}

fn search_depth(path: &Path, results: &mut Vec<String>, config: &Config) {
    match path.read_dir() {
        Ok(red_dir) => {
            red_dir.for_each(|file| {
                if let Ok(dir_entry) = file {
                    dir_entry.path().is_dir().then(|| {
                        search_depth(dir_entry.path().as_path(), results, config);
                    });
                    if let Some(name) = dir_entry.file_name().to_str() {
                        name.contains(&config.target).then(|| {
                            results.push(String::from(dir_entry.path().to_str().unwrap()));
                        });
                    }
                }
            })
        }
        Err(read_dir_err) => {
            eprintln!("{}", read_dir_err);
        }
    }
}


    


