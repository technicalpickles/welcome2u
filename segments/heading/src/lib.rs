use anyhow::Result;
use figlet_rs::FIGfont;
use rand::{seq::SliceRandom, thread_rng};
use ratatui::{layout::Rect, Frame};
use segment::*;
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

#[derive(Debug)]
pub struct HeadingSegmentInfo {
    pub heading: String,
    pub figure: String,
    pub font_choice: String,
}

impl Info for HeadingSegmentInfo {}

#[derive(Debug, Default)]
pub struct HeadingSegmentInfoBuilder {}

impl InfoBuilder<HeadingSegmentInfo> for HeadingSegmentInfoBuilder {
    fn build(&self) -> Result<HeadingSegmentInfo> {
        let heading = choose_fortune()?;
        let figure = figlet(random_font(), &heading)?;
        Ok(HeadingSegmentInfo {
            heading,
            figure,
            font_choice: random_font(),
        })
    }
}

#[derive(Debug)]
pub struct HeadingSegmentRenderer {
    pub info: HeadingSegmentInfo,
}

impl SegmentRenderer<HeadingSegmentInfo> for HeadingSegmentRenderer {
    fn new(info: HeadingSegmentInfo) -> Self {
        Self { info }
    }

    fn height(&self) -> u16 {
        // FIXME: need lines of the figure
        self.info.figure.lines().count() as u16
    }

    fn prepare(&mut self) -> Result<()> {
        Ok(())
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        // FIXME: doesn't seem to correctly render, ie only getting part of the figlet
        frame.render_widget(self.info.figure.clone(), area);
        Ok(())
    }
}
