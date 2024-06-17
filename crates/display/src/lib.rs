use ansi_term::Colour::Blue;

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
    fn render(&self);
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
    fn render(&self) {
        println!("{}", self.content);
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
    fn render(&self) {
        print_segment(&self.label, &self.content);
    }
}
