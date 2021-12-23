#[cfg(test)]
mod tests {
    use crate::{Config, find};

    #[test]
    fn folder_exists() {
        let where_to_find = String::from("/home/mikhail/git");
        let what_to_find = String::from("dotfiles");

        let expected: Vec<&str> = vec!["/home/mikhail/git/dotfiles"];

        // find(Config{path: where_to_find.as_ref(), target: &*what_to_find });
    }

    #[test]
    fn path_doesnt_exist() {
        let where_to_find = String::from("/home/mikhail/any_folder");
        let what_to_find = String::from("dotfiles");

        let expected = "No such path to search";

        // find(Config{path: where_to_find.as_ref(), target: &*what_to_find });
    }

    #[test]
    fn file_exist() {
        let where_to_find = String::from("/home/mikhail/git");
        let what_to_find = String::from("init.sh");

        let expected: Vec<&str> = vec!["/home/mikhail/git/dotfiles/init.sh"];

        // find(Config{path: where_to_find.as_ref(), target: &*what_to_find });
    }
}