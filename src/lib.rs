use std::fs::read_dir;
// use std::{fs, io};
use std::path::{Path};

// use colored::*;

pub struct Config {
    pub path: String,
    pub target: String,
}
        
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let path = args[1].clone();
        let target = args[2].clone();
        Ok(Config {
            path,
            target,
        })
    }
}

// fn path_exists(path: &str) -> Result<T, E> {
//      return Ok(Path::new(&config.path))
// }

pub fn search_depth(path: &Path, results: &mut Vec<String>, config: &Config) {
    // for i in path.read_dir().unwrap() {
    //     if i.as_ref().unwrap().file_name().to_str().unwrap().ends_with(&config.target) {
    //          results.push(i.as_ref().unwrap().path().to_str().unwrap().parse().unwrap());
    //     }
    //     if i.as_ref().unwrap().path().is_dir() {
    //         search_depth(i.unwrap().path().as_path(), results, config);
    //     } else {
    //         // its a file
    //         // println!("{}", i.unwrap().path().to_str().unwrap().green());
    //     }
    // }

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


    // if let Ok(red_dir) = path.read_dir() {
    //     red_dir.for_each(|file| {
    //         if let Ok(dir_entry) = file {
    //             dir_entry.path().is_dir().then(|| {
    //                 search_depth(dir_entry.path().as_path(), results, config);
    //             });
    //             if let Some(name) = dir_entry.file_name().to_str() {
    //                 name.contains(&config.target).then(|| {
    //                     results.push(String::from(dir_entry.path().to_str().unwrap()));
    //                 });
    //             }
    //         }
    //     })
    // } else if Err(e) == path.read_dir() {
    //     println!("Failed to read dir: {}", e);
    // }
}


pub fn find(config: Config) -> Result<Vec<String>, &'static str> {
    let mut results: Vec<String> = vec![];
    // let path = path_exists().unwrap_or_else(||{}); TODO
    if !Path::exists((&config.path).as_ref()) {
        return Err("No such path to search");
    }

    let path = Path::new(&config.path);

    search_depth(path, &mut results, &config);

    return Ok(results)
}

    


