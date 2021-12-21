use std::fmt;
use std::path::Path;

pub enum CustomError {
    NotEnoughArgumentError,
    InvalidPathError,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::NotEnoughArgumentError => {
                write!(f, "Not enough arguments!")
            }
            CustomError::InvalidPathError => {
                write!(f, "Invalid path!")
            }
        }
    }
}

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

pub fn find(config: Config) -> Result<Vec<String>, &str> {
    let mut results: Vec<String> = vec![];
    search_depth(config.path, &mut results, &config);
    return Ok(results)
}

pub fn search_depth(path: &Path, results: &mut Vec<String>, config: &Config) {
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
        Err(red_dir_err) => {
            println!("Failed to read directory: \
                The provided path doesn’t exist.\
                The process lacks permissions to view the contents.\
                The path points at a non-directory file. {}",
                red_dir_err); }
    }
}


    


