use sysinfo::{
    System,
    // Components, Disks, Networks, System,
};

use fmtsize::{FmtSize, Conventional};

fn main() {
    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();


    let used_memory = sys.used_memory().fmt_size(Conventional);
    let available_memory = sys.available_memory().fmt_size(Conventional);
    let total_memory = sys.total_memory().fmt_size(Conventional);

    println!("RAM - {} used, {} available / {}", used_memory, available_memory, total_memory);
}
