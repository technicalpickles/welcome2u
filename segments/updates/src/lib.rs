use anyhow::Result;
use display::MotdSegment;
use ratatui::{prelude::*, widgets::*};
use std::process::Command;

#[derive(Default, Debug)]
pub struct UpdatesSegment {
    info: Option<UpdatesInfo>,
}

#[derive(Debug)]
struct UpdatesInfo {
    updates_available: String,
}

impl UpdatesInfo {
    fn new(updates_available: String) -> Self {
        Self { updates_available }
    }

    fn collect() -> Result<Self> {
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

        Ok(Self::new(updates_available))
    }
}

impl MotdSegment for UpdatesSegment {
    fn height(&self) -> u16 {
        1
    }

    fn prepare(&mut self) -> Result<()> {
        self.info = Some(UpdatesInfo::collect()?);
        Ok(())
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

        if let Some(info) = &self.info {
            frame.render_widget(Paragraph::new(info.updates_available.clone()), data_area);
        }

        Ok(())
    }
}
