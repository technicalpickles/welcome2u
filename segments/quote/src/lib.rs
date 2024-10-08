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
    fn new(info: QuoteSegmentInfo) -> Self {
        Self { info }
    }

    fn height(&self) -> u16 {
        self.info.quote.lines().count() as u16
    }

    fn prepare(&mut self) -> Result<()> {
        self.info = QuoteSegmentInfo::default();
        Ok(())
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let styled_lines: Vec<Line> = self
            .info
            .quote
            .lines()
            .map(|line| Line::from(Span::styled(line, Style::default().dim())))
            .collect();

        let block = Block::default()
            .borders(Borders::NONE)
            .padding(Padding::horizontal(4));

        let paragraph = Paragraph::new(styled_lines)
            .wrap(Wrap { trim: true })
            .block(block);

        frame.render_widget(paragraph, area);

        Ok(())
    }
}
