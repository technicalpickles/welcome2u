use figlet_rs::FIGfont;
use rand::{seq::SliceRandom, thread_rng};
use display::MotdSegement;
use anyhow::Result;
use thiserror::Error;

use fortune::{Fortunes, NoFortunesError};

fn choose_fortune() -> Result<String, NoFortunesError> {
    let fortune_path = String::from("/opt/homebrew/opt/fortune/share/games/fortunes/intro");
    let fortune_file = Fortunes::from_file(&fortune_path).unwrap();
    let fortune = fortune_file.choose_one()?;

    Ok(fortune.to_string())
}

#[derive(Error, Debug)]
enum FigletError {}

fn figlet(font: String, message: &str) -> Result<String> {
    let font_directory = "/opt/homebrew/opt/figlet";
    let font_path = format!("{}/share/figlet/fonts/{}.flf", font_directory, font);

    let font = match FIGfont::from_file(font_path.as_str()) {
        Ok(font) => font,
        Err(error) => panic!("Could not load font from {}: {}", font_path, error),
    };

    let figure = font.convert(message).unwrap();
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

pub struct HeadingSegment {
    pub heading: String
}

impl HeadingSegment {
    pub fn new() -> Self {
        Self {
            heading: choose_fortune().unwrap()
        }
    }
}
impl Default for HeadingSegment {
    fn default() -> Self {
        Self::new()
    }
}

impl MotdSegement for HeadingSegment {
    fn render(&mut self) -> Result<()> { 
        let font_choice = random_font();
        let figure = figlet(font_choice, &self.heading)?;

        let seed = rand::random::<f64>() * 1_000_000.0;
        let freq = 0.1;
        // default is 1.0 ... increase the number to have it spread out a lil less, ie not changing as much
        let spread = 5.0;
        let inverse = false;

        figure.lines().for_each(|line| {
            lolcat::print_rainbow(line, freq, seed, spread, inverse);
            println!();
        });

        Ok(())
    }
}
