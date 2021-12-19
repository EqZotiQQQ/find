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

    // path.read_dir().unwrap().into_iter().map(|item| println!("item: {:?}", item));

    // for_each terminate operation
    // map is non-terminate operator
    // path.read_dir().unwrap().into_iter().for_each(|item| println!("item: {:?}", item));
    // path.read_dir().unwrap().into_iter().for_each(|item| println!("{}", item.unwrap().file_name().to_str().unwrap())); // prints file names
    // path.read_dir().unwrap().into_iter().for_each(|item| if let x= item.unwrap().file_type().unwrap() {
    //     x.is_dir().then(|| { println!("dir {:?}", x) });
    //     x.is_file().then(|| { println!("file {:?}", x) });
    // });

    // path.read_dir().unwrap().into_iter().for_each(|item| if let x= item.unwrap() {
    //     x.file_type().unwrap().is_file().then(|| { println!("file {:?}", x) });
    //     x.file_type().unwrap().is_dir().then(|| { search_depth(x.path().as_path(), results, config) });
    // });

    match path.read_dir() {
        Ok(red_dir) => {
            red_dir.for_each(|file| {
                match file {
                    Ok(dir_entry) => {
                        match dir_entry.file_name().to_str() {
                            None => {
                                println!("No such file?");
                            }
                            Some(s) => {
                                dir_entry.path().is_dir().then(|| {
                                    search_depth(dir_entry.path().as_path(), results, config);
                                });
                                s.contains(&config.target).then(|| {
                                    results.push(String::from(dir_entry.path().to_str().unwrap()));
                                });
                            }
                        }
                    }
                    Err(_) => {}
                }
            })
        }
        Err(red_dir_err) => { println!("Failed to read directory: \
        The provided path doesn’t exist.\
        The process lacks permissions to view the contents.\
        The path points at a non-directory file. {}", red_dir_err); }
    }
    // path.read_dir().unwrap().into_iter().for_each(|item| {
    //
    // });

    // path.map(|item| println!("item: {:?}", item));

    // for i in path.read_dir().unwrap() {
    //
    // }
    // println!("Current root: {}", path.to_str().unwrap().red());
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

    


