use ratatui::Frame;
use display::Single;
use std::process::{Command, Stdio, ExitStatus};
use anyhow::Result;
use thiserror::Error;
use display::MotdSegment;

#[derive(Debug)]
pub struct CommandSegment {
    command : String,
    output: String,
}

impl CommandSegment {
    pub fn new(command: &str) -> Self{
        Self {
            command: command.to_string(),
            output: "".to_string()
        }
    }
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("command `{command_ran:?}` failed with status `{status:?}`")]
    CommandFailed {
        command_ran: String,
        status: ExitStatus,
    },
    #[error("failed to spawn command")]
    SpawnFailed(#[from] std::io::Error),
    #[error("couldn't parse output as utf8")]
    OutputParseError(#[from] std::string::FromUtf8Error),
}

impl MotdSegment for CommandSegment {
    fn prepare(&mut self) -> Result<()> {
        // TODO can we avoid cloning this?
        let mut command = Command::new(self.command.clone());

        let child = match command.stdout(Stdio::piped()).spawn() {
            Ok(child) => child,
            Err(error) => return Err(CommandError::SpawnFailed(error).into())
        };

        let output = child.wait_with_output()?;

        if !output.status.success() {
            let error = CommandError::CommandFailed {
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

    fn render(&self, frame: &mut Frame) -> Result<()> {
        Single::new(&self.output).render(frame)
    }
}