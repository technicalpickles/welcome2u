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
