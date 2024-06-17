use figlet_rs::FIGfont;
use rand::{seq::SliceRandom, thread_rng};

use fortune::{Fortunes, NoFortunesError};

fn choose_fortune() -> Result<String, NoFortunesError> {
    let fortune_path = String::from("/opt/homebrew/opt/fortune/share/games/fortunes/intro");
    let fortune_file = Fortunes::from_file(&fortune_path).unwrap();
    let fortune = fortune_file.choose_one()?;

    Ok(fortune.to_string())
}

enum FigletErrors {}

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
        // "big", # seems broken from figlet-rs code?
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
    let message = match choose_fortune() {
        Ok(message) => message,
        Err(_) => {
            println!("No fortunes found");
            return;
        }
    };

    let font_choice = random_font();
    let figure = match figlet(font_choice, message) {
        Ok(figure) => figure,
        Err(_) => {
            println!("Could not generate figure");
            return;
        }
    };

    let seed = rand::random::<f64>() * 1_000_000.0;
    let freq = 0.1;
    // default is 1.0 ... increase the number to have it spread out a lil less, ie not changing as
    // much
    let spread = 5.0;
    let inverse = false;

    // printli

    figure.lines().for_each(|line| {
        lolcat::print_rainbow(line, freq, seed, spread, inverse);
        println!();
    });
}
