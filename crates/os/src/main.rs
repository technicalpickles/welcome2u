use display::format_label;

fn main() {
    let info = os_info::get();
    println!("{}{}", format_label("OS"), info)
}
