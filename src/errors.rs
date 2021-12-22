pub mod custom_errors {

    use std::fmt;

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
}