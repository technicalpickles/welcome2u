use ansi_term::Style;
use fortune::{Fortunes, NoFortunesError};
use textwrap::indent;

use display::MotdSegement;

fn choose_fortune() -> Result<String, NoFortunesError> {
    // TODO: support multiple fortune files: pickleisms, collected-quotes
    let fortune_path =
        String::from("/opt/homebrew/opt/fortune/share/games/fortunes/collected-quotes");
    let fortune_file = Fortunes::from_file(&fortune_path).unwrap();
    let fortune = fortune_file.choose_one()?;

    Ok(fortune.to_string())
}

pub struct FortuneHeaderSegment {
    fortune: String
}

impl FortuneHeaderSegment {
    pub fn new() -> Self {
        Self {
            fortune: choose_fortune().unwrap()
        }
    }
}

impl MotdSegement for FortuneHeaderSegment {
    fn render(&self) {
        let content = textwrap::fill(&self.fortune, 80);
        let content = indent(&content, "       ");
        let content = Style::default().dimmed().paint(content);

        println!("{}", content);
        println!();
    }
}
