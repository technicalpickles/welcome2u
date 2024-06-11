use sysinfo::System;

use display::print_segment;

use fmtsize::{FmtSize, Conventional};

struct MemoryInfo {
    used_memory: String,
    available_memory: String,
    total_memory: String,
}

impl MemoryInfo {
    fn new(used_memory: String, available_memory: String, total_memory: String) -> Self {
        Self {
            used_memory,
            available_memory,
            total_memory,
        }
    }

    fn collect() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        // TODO: use consistent units, instead of letting Conventional decide
        let used_memory = sys.used_memory().fmt_size(Conventional).to_string();
        let available_memory = sys.available_memory().fmt_size(Conventional).to_string();
        let total_memory = sys.total_memory().fmt_size(Conventional).to_string();

        Self::new(used_memory, available_memory, total_memory)
    }


    fn used_memory(&self) -> &str { 
        &self.used_memory
    } 

    fn available_memory(&self) -> &str {
        &self.available_memory
    }

    fn total_memory(&self) -> &str {
        &self.total_memory
    }
}

fn main() {
    let memory_info = MemoryInfo::collect();
    print_segment(
        "RAM",
        format!(
            "RAM - {} used, {} available / {}",
            memory_info.used_memory(),
            memory_info.available_memory(),
            memory_info.total_memory(),
        ).as_str(),
    );
}
