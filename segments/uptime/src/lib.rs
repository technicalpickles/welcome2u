use anyhow::Result;
use ratatui::{prelude::*, widgets::*};
use segment::*;
use sysinfo::System;
use tracing::instrument;
#[derive(Debug)]
pub struct UptimeSegmentRenderer {
    info: UptimeInfo,
}

#[derive(Debug)]
pub struct UptimeInfo {
    uptime: String,
}

impl Info for UptimeInfo {}

#[derive(Debug, Default)]
pub struct UptimeInfoBuilder {}

impl InfoBuilder<UptimeInfo> for UptimeInfoBuilder {
    #[instrument(skip(self), fields(builder_type = "UptimeInfoBuilder"))]
    async fn build(&self) -> Result<UptimeInfo> {
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
        Ok(UptimeInfo { uptime })
    }
}

impl SegmentRenderer<UptimeInfo> for UptimeSegmentRenderer {
    fn height(&self) -> u16 {
        1
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let [label_area, data_area] = create_label_data_layout(area);

        frame.render_widget(label("Uptime"), label_area);

        let uptime_color = if self.info.uptime.contains("day") || self.info.uptime.contains("days")
        {
            Color::Yellow
        } else {
            Color::Reset
        };
        frame.render_widget(
            Paragraph::new(self.info.uptime.clone()).fg(uptime_color),
            data_area,
        );

        Ok(())
    }
}

impl From<Box<UptimeInfo>> for UptimeSegmentRenderer {
    fn from(info: Box<UptimeInfo>) -> Self {
        Self { info: *info }
    }
}
