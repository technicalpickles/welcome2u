use std::process::{Command, Stdio};

use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("BASE_DIR", ".");
    env::set_var("CONFIG_PATH", "./config.sh");

    let mut processes = Vec::new();

    let commands = vec![
        Command::new("target/debug/intro"),
        Command::new("target/debug/fortune-header"),
        Command::new("target/debug/user"),
        Command::new("target/debug/os"),
        Command::new("modules/20-uptime"),
        Command::new("modules/30-load"),
        Command::new("target/debug/memory"),
        Command::new("modules/33-disk"),
        Command::new("target/debug/docker"),
    ];

    // spawn all child processes in the background
    for mut command in commands {
        let child = command.stdout(Stdio::piped()).spawn()?;
        processes.push(child);
    }

    // once the child processes have started, we can wait for them to finish
    // and get their output
    for child in processes {
        let output = child.wait_with_output()?;
        if !output.status.success() {
            println!("Command failed: {:?}", output.status)
        } else {
            print!("{}", String::from_utf8(output.stdout)?)
        }
    }

    Ok(())
}
