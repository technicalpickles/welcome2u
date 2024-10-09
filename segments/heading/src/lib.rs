use anyhow::Result;
use figlet_rs::FIGfont;
use rand::{seq::SliceRandom, thread_rng};
use ratatui::{prelude::*, widgets::*, Frame};
use segment::*;
use thiserror::Error;

use ansi_to_tui::IntoText;
use fortune::{Fortunes, NoFortunesError};
use lolcrab::Lolcrab;

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

    // Remove empty lines from the beginning and end of the figure
    let trimmed_figure = figure.to_string().trim_matches('\n').to_string();
    Ok(trimmed_figure)
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
    fn height(&self) -> u16 {
        // FIXME: need lines of the figure
        self.info.figure.lines().count() as u16
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let mut colorized_figure = Vec::new();
        Lolcrab::new(None, None).colorize_str(&self.info.figure, &mut colorized_figure)?;

        let paragraph = Paragraph::new(colorized_figure.into_text()?);
        frame.render_widget(paragraph, area);

        Ok(())
    }
}

impl From<Box<HeadingSegmentInfo>> for HeadingSegmentRenderer {
    fn from(info: Box<HeadingSegmentInfo>) -> Self {
        Self { info: *info }
    }
}
