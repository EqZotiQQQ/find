#[cfg(test)]
mod tests {
    // use std::process::exit;
    use find::Config;
    use find::find;

    #[test]
    fn folder_exists() {
        let where_to_find = String::from("/home/mikhail/git");
        let what_to_find = String::from("dotfiles");

        let expected: Vec<&str> = vec!["/home/mikhail/git/dotfiles"];

        match find(Config{path: where_to_find, target: what_to_find}) {
            Ok(k) => {
                assert_eq!(expected, k);
            }
            Err(e) => {
                println!("Failed to find string! Cause: {}", e);
            }
        };
    }

    #[test]
    fn path_doesnt_exist() {
        let where_to_find = String::from("/home/mikhail/any_folder");
        let what_to_find = String::from("dotfiles");

        let expected = "No such path to search";

        match find(Config{path: where_to_find, target: what_to_find}) {
            Ok(_) => {
            }
            Err(e) => {
                assert_eq!(expected, e);
            }
        };
    }

    #[test]
    fn file_exist() {
        let where_to_find = String::from("/home/mikhail/git");
        let what_to_find = String::from("init.sh");

        let expected: Vec<&str> = vec!["/home/mikhail/git/dotfiles/init.sh"];

        match find(Config{path: where_to_find, target: what_to_find}) {
            Ok(k) => {
                assert_eq!(expected, k);
            }
            Err(e) => {
                println!("Failed to find string! Cause: {}", e);
            }
        };
    }
}