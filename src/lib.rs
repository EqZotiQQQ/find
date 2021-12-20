use std::path::Path;

pub struct Config<'a> {
    pub path: &'a str,
    pub target: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Config {
            path: args[1].as_str(),
            target: args[2].as_str(),
        })
    }
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
        Err(red_dir_err) => { println!("Failed to read directory: \
        The provided path doesn’t exist.\
        The process lacks permissions to view the contents.\
        The path points at a non-directory file. {}", red_dir_err); }
    }
}


pub fn find(config: Config) -> Result<Vec<String>, &str> {
    let mut results: Vec<String> = vec![];
    if !Path::exists((&config.path).as_ref()) {
        return Err("No such path to search");
    }

    let path = Path::new(&config.path);

    search_depth(path, &mut results, &config);

    return Ok(results)
}

    


