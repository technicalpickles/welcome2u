use ansi_term::Colour::Blue;
use anyhow::Result;

pub fn format_label(text: &str) -> String {
    let text = format!("{}:", text);
    let label = format!("{:<width$}", text, width = 16);
    // color last, or we width won't work
    Blue.bold().paint(label).to_string()
}

pub fn print_segment(label: &str, contents: &str) {
    println!("{}{}", format_label(label), contents);
}

pub trait MotdSegement {
    fn prepare(&mut self) -> Result<()> {
        Ok(())
    }
    fn render(&self) -> Result<()>;
}

pub struct Single {
    content: String
}

impl Single {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }
}

impl MotdSegement for Single {
    fn render(&self) -> Result<()>  {
        println!("{}", self.content);
        Ok(())
    }
}

pub struct LabelWithContent {
    label: String,
    content: String,
}

impl LabelWithContent {
    pub fn new(label: &str, content: &str) -> Self {
        Self {
            label: label.to_string(),
            content: content.to_string(),
        }
    }
}

impl MotdSegement for LabelWithContent {
    fn render(&self) -> Result<()> {
        print_segment(&self.label, &self.content);
        Ok(())
    }
}
