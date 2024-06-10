use ansi_term::Colour::Blue;

fn main() {
    let info = os_info::get();
    println!("{}: {}", Blue.paint("OS"), info)
}
