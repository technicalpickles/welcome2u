use anyhow::Result;
use ratatui::{prelude::*, widgets::*};
use segment::*;
use std::process::Command;

#[derive(Default, Debug)]
pub struct UpdatesSegmentRenderer {
    info: UpdatesInfo,
}

#[derive(Debug, Default)]
struct UpdatesInfo {
    updates_available: String,
}

impl Info for UpdatesInfo {}

#[derive(Debug, Default)]
struct UpdatesInfoBuilder {}

impl InfoBuilder<UpdatesInfo> for UpdatesInfoBuilder {
    fn build(&self) -> Result<UpdatesInfo> {
        let output = Command::new("softwareupdate").arg("--list").output()?;

        let stdout = String::from_utf8(output.stdout)?;
        let updates_available = if stdout.contains("No new software available.") {
            "No updates available".to_string()
        } else {
            let count = stdout.lines().filter(|line| line.contains("*")).count();
            format!(
                "{} update{} available",
                count,
                if count == 1 { "" } else { "s" }
            )
        };

        Ok(UpdatesInfo { updates_available })
    }
}

impl SegmentRenderer<UpdatesInfo> for UpdatesSegmentRenderer {
    fn height(&self) -> u16 {
        1
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(16), Constraint::Fill(1)]);

        let [label_area, data_area] = layout.areas(area);

        frame.render_widget(
            Paragraph::new("Updates").style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            label_area,
        );

        frame.render_widget(
            Paragraph::new(self.info.updates_available.clone()),
            data_area,
        );

        Ok(())
    }
}

impl From<Box<UpdatesInfo>> for UpdatesSegmentRenderer {
    fn from(info: Box<UpdatesInfo>) -> Self {
        Self { info: *info }
    }
}
