use anyhow::Result;
use segment::Segment;
use figlet_rs::FIGfont;
use rand::{seq::SliceRandom, thread_rng};
use ratatui::{layout::Rect, Frame};
use std::fmt;
use thiserror::Error;

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
    ConversionError { message: String },
}

fn figlet(font: String, message: &str) -> Result<String> {
    let font_directory = "/opt/homebrew/opt/figlet";
    let font_path = format!("{}/share/figlet/fonts/{}.flf", font_directory, font);

    let font =
        FIGfont::from_file(font_path.as_str()).map_err(|error| FigletError::FontLoadError {
            path: font_path.clone(),
            message: error.to_string(),
        })?;

    let figure = font
        .convert(message)
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
    pub heading: String,
    pub figure: String,
    pub font_choice: String,
}

impl Default for HeadingSegment {
    fn default() -> Self {
        Self {
            heading: choose_fortune().unwrap(),
            font_choice: String::new(),
            figure: String::new(),
        }
    }
}

impl fmt::Debug for HeadingSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HeadingSegment")
            .field("heading", &self.heading)
            .field("font_choice", &self.font_choice)
            .finish()
    }
}

impl Segment for HeadingSegment {
    fn height(&self) -> u16 {
        // FIXME: need lines of the figure
        self.figure.lines().count() as u16
    }

    fn prepare(&mut self) -> Result<()> {
        self.font_choice = random_font();
        self.figure = figlet(self.font_choice.clone(), &self.heading)?;

        Ok(())
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        // FIXME: doesn't seem to correctly render, ie only getting part of the figlet
        frame.render_widget(self.figure.clone(), area);
        Ok(())
    }
}
