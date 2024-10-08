use rand::Rng;
use thiserror::Error;

use std::{
    error::Error,
    fmt,
    fs::File,
    io::{self, Read},
    path::Path,
};

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Error)]
pub struct NoFortunesError;

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for NoFortunesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No fortunes available")
    }
}

// from https://github.com/zuisong/rs-fortune
pub struct Fortunes(Vec<String>);
impl Fortunes {
    pub fn new(content: String) -> Result<Fortunes, Box<dyn Error>> {
        let fortunes = content.split("\n%\n").map(|it| it.to_string()).collect();
        Ok(Self(fortunes))
    }

    pub fn from_file(path: &String) -> Result<Fortunes, Box<dyn Error>> {
        let file_path = Path::new(&path);
        if !file_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("The fortune file '{path}' does not exist"),
            )
            .into());
        }
        if file_path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("'{path}' is a directory, not a file"),
            )
            .into());
        }
        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Self::new(content)
    }

    pub fn choose_one(&self) -> Result<&String, NoFortunesError> {
        let fortunes = &self.0;
        if fortunes.is_empty() {
            return Err(NoFortunesError);
        }
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..fortunes.len());

        Ok(&fortunes[index])
    }
}
