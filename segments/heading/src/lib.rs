use figlet_rs::FIGfont;
use rand::{seq::SliceRandom, thread_rng};
use display::MotdSegment;
use anyhow::Result;
use thiserror::Error;
use ratatui::Frame;
use std::fmt;

use fortune::{Fortunes, NoFortunesError};

fn choose_fortune() -> Result<String, NoFortunesError> {
    let fortune_path = String::from("/opt/homebrew/opt/fortune/share/games/fortunes/intro");
    let fortune_file = Fortunes::from_file(&fortune_path).unwrap();
    let fortune = fortune_file.choose_one()?;

    Ok(fortune.to_string())
}

#[derive(Error, Debug)]
pub enum FigletError {
    #[error("Could not load font from {path}: {message}")]
    FontLoadError { path: String, message: String },
    #[error("Could not convert text to figlet: {message}")]
    ConversionError { message: String }
}

fn figlet(font: String, message: &str) -> Result<String> {
    let font_directory = "/opt/homebrew/opt/figlet";
    let font_path = format!("{}/share/figlet/fonts/{}.flf", font_directory, font);

    let font = FIGfont::from_file(font_path.as_str())
        .map_err(|error| FigletError::FontLoadError {
            path: font_path.clone(),
            message: error.to_string(),
        })?;

    let figure = font.convert(message)
        .ok_or_else(|| FigletError::ConversionError {
            message: "Failed to convert text to figlet".to_string(),
        })?;
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

impl Default for HeadingSegment {
    fn default() -> Self {
        Self {
            heading: choose_fortune().unwrap()
        }
    }
}

impl fmt::Debug for HeadingSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HeadingSegment")
            .field("heading", &self.heading)
            .finish()
    }
}

impl MotdSegment for HeadingSegment {
    fn prepare(&mut self) -> Result<()> {
        Ok(())
    }

    fn render(&self, frame: &mut Frame) -> Result<()> { 
        let font_choice = random_font();
        let figure = figlet(font_choice, &self.heading)?;

        let seed = rand::random::<f64>() * 1_000_000.0;
        let freq = 0.1;
        // default is 1.0 ... increase the number to have it spread out a lil less, ie not changing as much
        let spread = 5.0;
        let inverse = false;

        // FIXME: figure out how to print-rainbow to a string instead
        figure.lines().for_each(|line| {
            lolcat::print_rainbow(line, freq, seed, spread, inverse);
            println!();
        });

        Ok(())
    }
}
