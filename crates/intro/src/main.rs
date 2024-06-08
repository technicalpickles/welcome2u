use figlet_rs::FIGfont;
use rand::{
    thread_rng,
    seq::SliceRandom,
    Rng,
};
use std::{
    fmt,
    error::Error,
    fs::File,
    io::{self, Read},
    path::Path,
};

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
struct NoFortunesError;
// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for NoFortunesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no fortunes found in file")
    }
}

// from https://github.com/zuisong/rs-fortune
struct Fortunes(Vec<String>);
impl Fortunes {
    pub fn new(content: String) -> Result<Fortunes, Box<dyn Error>> {
        let fortunes = content
            .split("\n%\n")
            .map(|it| it.to_string())
            .collect();
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

fn choose_fortune() -> Result<String, NoFortunesError> {
    let fortune_path = String::from("/opt/homebrew/opt/fortune/share/games/fortunes/intro");
    let fortune_file = Fortunes::from_file(&fortune_path).unwrap();
    let fortune = fortune_file.choose_one()?;

    Ok(fortune.to_string())
}

enum FigletErrors {
}

fn figlet(font: String, message: String) -> Result<String, FigletErrors> {
    let font_directory = "/opt/homebrew/opt/figlet";
    let font_path = format!("{}/share/figlet/fonts/{}.flf", font_directory, font);

    let font = match FIGfont::from_file(font_path.as_str()) {
        Ok(font) => font,
        Err(error) => panic!("Could not load font from {}: {}", font_path, error),
    };

    let figure = font.convert(&message).unwrap();
    Ok(figure.to_string())
}

fn random_font() -> String {
    let fonts = [
        "bell",
        "big",
        "slant",
        "contessa",
        "computer",
        "cricket",
        "cybermedium",
        "jazmine",
        "rectangles",
    ];
    let mut rng = thread_rng();
    let font_choice = fonts.choose(&mut rng);
    font_choice.unwrap().to_string()
}

fn main() {
    let fortune = match choose_fortune() {
        Ok(fortune) => fortune,
        Err(_) => {
            println!("No fortunes found");
            return;
        }
    };

    let font_choice = random_font();
    let figure_result = figlet(font_choice, fortune);

    let figure = match figure_result {
        Ok(figure) => figure,
        Err(_) => {
            println!("Could not generate figure");
            return;
        }
    };

    println!("{}", figure);
}
