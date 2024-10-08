use anyhow::Result;
use ratatui::layout::Rect;
use ratatui::Frame;
use segment::Info;
use segment::InfoBuilder;
use segment::SegmentRenderer;
use segment::Text;
use std::process::{Command, ExitStatus, Stdio};
use thiserror::Error;

#[derive(Debug, Default)]
struct CommandInfo {
    output: String,
    command: String,
}

impl Info for CommandInfo {}

#[derive(Debug, Default)]
struct CommandInfoBuilder {
    command: String,
}

impl CommandInfoBuilder {
    pub fn command(mut self, command: String) -> Self {
        self.command = command;
        self
    }
}

impl InfoBuilder<CommandInfo> for CommandInfoBuilder {
    fn build(&self) -> Result<CommandInfo> {
        let output = Command::new(&self.command)
            .stdout(Stdio::piped())
            .output()?;

        if !output.status.success() {
            return Err(CommandError::CommandFailed {
                command_ran: self.command.clone(),
                status: output.status,
            }
            .into());
        }

        let output_str = String::from_utf8(output.stdout)?;

        Ok(CommandInfo {
            command: self.command.clone(),
            output: output_str,
        })
    }
}

#[derive(Debug)]
pub struct CommandSegmentRenderer {
    info: CommandInfo,
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

impl SegmentRenderer<CommandInfo> for CommandSegmentRenderer {
    fn new(info: CommandInfo) -> Self {
        Self { info }
    }

    fn height(&self) -> u16 {
        self.info.output.lines().count() as u16
    }

    fn prepare(&mut self) -> Result<()> {
        self.info = CommandInfoBuilder::default()
            .command(self.info.command.clone())
            .build()?;

        Ok(())
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        Text::new(&self.info.output).render(frame, area)
    }
}
