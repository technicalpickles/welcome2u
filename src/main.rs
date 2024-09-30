use std::process::{Command, Stdio, ExitStatus};

use std::env;
use anyhow::Result;
use thiserror::Error;
use anyhow::Context;

use display::MotdSegement;

#[derive(Debug)]
struct CommandSegment {
    command : String,
    output: String,
}

impl CommandSegment {
    fn new(command: &str) -> Self{
        Self {
            command: command.to_string(),
            output: "".to_string()
        }
    }
}

#[derive(Error, Debug)]
enum CommandError {
    #[error("command `{command_ran:?}` failed with status `{status:?}`")]
    Failed {
        command_ran: String,
        status: ExitStatus,
    },
    #[error("failed to spawn command")]
    SpawnFailed(#[from] std::io::Error),

    #[error("couldn't parse output as utf8")]
    OutputParseError(#[from] std::string::FromUtf8Error),
}

impl MotdSegement for CommandSegment {
    fn prepare(&mut self) -> Result<()> {
        // TODO can we avoid cloning this?
        let mut command = Command::new(self.command.clone());

        let child = match command.stdout(Stdio::piped()).spawn() {
            Ok(child) => child,
            Err(error) => return Err(CommandError::SpawnFailed(error).into())
        };

        let output = child.wait_with_output()?;

        if !output.status.success() {
            let error = CommandError::Failed {
                // TODO can we avoid a clone?
                command_ran: self.command.clone(),
                status: output.status,

            };
            return Err(error.into())
        }
        match String::from_utf8(output.stdout) {
            Ok(string) => self.output = string,
            Err(error) => return Err(CommandError::OutputParseError(error).into())
        }

        Ok(())
    }

    fn render(&self) -> Result<()> {
        print!("{}", self.output);
        Ok(())
    }
}

fn main() -> Result<()> {
    env::set_var("BASE_DIR", ".");
    env::set_var("CONFIG_PATH", "./config.sh");

    let mut segments : Vec<Box<dyn MotdSegement>> = vec![
        Box::<heading::HeadingSegment>::default(),
        Box::<quote::FortuneHeaderSegment>::default(),
        // Box::new(CommandSegment::new("target/debug/user")),
        // Box::new(CommandSegment::new("target/debug/os")),
        // Box::new(CommandSegment::new("modules/20-uptime")),
        // Box::new(CommandSegment::new("modules/30-load")),
        // Box::new(CommandSegment::new("target/debug/memory")),
        Box::<disk::DiskSegment>::default(),
        // Box::new(CommandSegment::new("target/debug/docker"))
    ];

    for segment in segments.iter_mut() {
        segment.prepare().with_context(|| format!("Failed to prepare segment: {:?}", segment))?;
    }

    for segment in segments.iter_mut() {
        segment.render().with_context(|| format!("Failed to render segment: {:?}", segment))?;
    }

    Ok(())
}
