use ansi_term::Style;
use anyhow::Result;
use fortune::{Fortunes, NoFortunesError};
use ratatui::prelude::*;
use segment::SegmentRenderer;
use segment::Text;
use textwrap::indent;

fn choose_fortune() -> Result<String, NoFortunesError> {
    // TODO: support multiple fortune files: pickleisms, collected-quotes
    let fortune_path =
        String::from("/opt/homebrew/opt/fortune/share/games/fortunes/collected-quotes");
    let fortune_file = Fortunes::from_file(&fortune_path).unwrap();
    let fortune = fortune_file.choose_one()?;

    Ok(fortune.to_string())
}

#[derive(Debug)]
pub struct QuoteSegmentInfo {
    quote: String,
}

impl Default for QuoteSegmentInfo {
    fn default() -> Self {
        Self {
            quote: choose_fortune().unwrap(),
        }
    }
}

#[derive(Debug, Default)]
pub struct QuoteSegmentRenderer {
    info: QuoteSegmentInfo,
}

impl SegmentRenderer for QuoteSegmentRenderer {
    fn height(&self) -> u16 {
        1
    }

    fn prepare(&mut self) -> Result<()> {
        self.info = QuoteSegmentInfo::default();
        Ok(())
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let content = textwrap::fill(&self.info.quote, 80);
        let content = indent(&content, "       ");
        let content = Style::default().dimmed().paint(content);

        Text::new(&content).render(frame, area)?;
        Ok(())
    }
}
