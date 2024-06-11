use display::print_segment;

fn main() {
    let info = os_info::get();

    print_segment("OS", &info.to_string());
}
