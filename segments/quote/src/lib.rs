use anyhow::Result;
use fortune::{Fortunes, NoFortunesError};
use ratatui::prelude::*;
use ratatui::widgets::*;
use segment::*;

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

impl Info for QuoteSegmentInfo {}

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

impl SegmentRenderer<QuoteSegmentInfo> for QuoteSegmentRenderer {
    fn height(&self) -> u16 {
        // Add 2 to account for the new padding lines
        self.info.quote.lines().count() as u16 + 2
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let mut styled_lines = Vec::with_capacity(self.info.quote.lines().count() + 2);
        styled_lines.push(Line::default()); // Add an empty line for top padding
        styled_lines.extend(self.info.quote.lines().map(Line::from));
        styled_lines.push(Line::default()); // Add an empty line for bottom padding

        let block = Block::default()
            .borders(Borders::NONE)
            .padding(Padding::horizontal(4));

        let paragraph = Paragraph::new(styled_lines)
            .wrap(Wrap { trim: true })
            .style(Style::default().dim())
            .block(block);

        frame.render_widget(paragraph, area);

        Ok(())
    }
}

impl From<Box<QuoteSegmentInfo>> for QuoteSegmentRenderer {
    fn from(info: Box<QuoteSegmentInfo>) -> Self {
        Self { info: *info }
    }
}
