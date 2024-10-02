use anyhow::Result;
use display::MotdSegment;
use ratatui::{prelude::*, widgets::*};
use sysinfo::System;

#[derive(Default, Debug)]
pub struct UptimeSegment {
    info: Option<UptimeInfo>,
}

#[derive(Debug)]
struct UptimeInfo {
    uptime: String,
}

impl UptimeInfo {
    fn new(uptime: String) -> Self {
        Self { uptime }
    }

    fn collect() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let uptime_secs = System::uptime();
        let days = uptime_secs / 86400;
        let hours = (uptime_secs % 86400) / 3600;
        let minutes = (uptime_secs % 3600) / 60;
        let seconds = uptime_secs % 60;

        let mut uptime_parts = Vec::new();
        if days > 0 {
            uptime_parts.push(format!("{} day{}", days, if days == 1 { "" } else { "s" }));
        }
        if hours > 0 {
            uptime_parts.push(format!(
                "{} hour{}",
                hours,
                if hours == 1 { "" } else { "s" }
            ));
        }
        if minutes > 0 {
            uptime_parts.push(format!(
                "{} minute{}",
                minutes,
                if minutes == 1 { "" } else { "s" }
            ));
        }
        if seconds > 0 || uptime_parts.is_empty() {
            uptime_parts.push(format!(
                "{} second{}",
                seconds,
                if seconds == 1 { "" } else { "s" }
            ));
        }

        let uptime = uptime_parts.join(", ");

        Self::new(uptime)
    }
}

impl MotdSegment for UptimeSegment {
    fn prepare(&mut self) -> Result<()> {
        self.info = Some(UptimeInfo::collect());
        Ok(())
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(16), Constraint::Fill(1)]);

        let [label_area, data_area] = layout.areas(area);

        frame.render_widget(Paragraph::new("Uptime").fg(Color::Blue).bold(), label_area);

        if let Some(info) = &self.info {
            frame.render_widget(Paragraph::new(info.uptime.clone()), data_area);
        }

        Ok(())
    }
}
