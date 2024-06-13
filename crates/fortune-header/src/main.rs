use ansi_term::Style;
use fortune::{Fortunes, NoFortunesError};
use textwrap::indent;

fn choose_fortune() -> Result<String, NoFortunesError> {
    // TODO: support multiple fortune files: pickleisms, collected-quotes
    let fortune_path =
        String::from("/opt/homebrew/opt/fortune/share/games/fortunes/collected-quotes");
    let fortune_file = Fortunes::from_file(&fortune_path).unwrap();
    let fortune = fortune_file.choose_one()?;

    Ok(fortune.to_string())
}

fn main() {
    let message = match choose_fortune() {
        Ok(message) => message,
        Err(_) => {
            println!("No fortunes found");
            return;
        }
    };

    let message = textwrap::fill(&message, 80);
    let message = indent(&message, "       ");
    let message = Style::default().dimmed().paint(message);

    println!("{}", message);
}
