use ansi_term::Style;
use anyhow::Result;
use display::Segment;
use display::Single;
use fortune::{Fortunes, NoFortunesError};
use ratatui::prelude::*;
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
pub struct FortuneHeaderSegment {
    fortune: String,
}

impl Default for FortuneHeaderSegment {
    fn default() -> Self {
        Self {
            fortune: choose_fortune().unwrap(),
        }
    }
}

impl Segment for FortuneHeaderSegment {
    fn height(&self) -> u16 {
        1
    }

    fn prepare(&mut self) -> Result<()> {
        self.fortune =
            choose_fortune().map_err(|e| anyhow::anyhow!("Failed to choose fortune: {}", e))?;
        Ok(())
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let content = textwrap::fill(&self.fortune, 80);
        let content = indent(&content, "       ");
        let content = Style::default().dimmed().paint(content);

        Single::new(&content).render(frame, area)?;
        Ok(())
    }
}
