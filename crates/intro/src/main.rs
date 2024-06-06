use figlet_rs::FIGfont;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::env;

fn main() {
    // possible font list
    let fonts = [
        "bell",
        "big",
        "slant",
        "contessa",
        "computer",
        "cricket",
        "cybermedium",
        "jazmine",
        "rectangles",
    ];

    let font_directory = "/opt/homebrew/opt/figlet";

    // choose a random font
    let mut rng = thread_rng();
    let font = fonts.choose(&mut rng);
    let font_path = format!("{}/share/figlet/fonts/{}.flf", font_directory, font.unwrap());

    let standard_font = FIGfont::from_file(font_path.as_str()).unwrap();
    let message = env::args().nth(1).unwrap();
    let figure = standard_font.convert(message.as_str());
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
}
