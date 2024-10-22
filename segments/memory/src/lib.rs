use anyhow::Result;
use fmtsize::{Conventional, FmtSize};
use ratatui::{
    prelude::*,
    style::{Color, Style},
    widgets::*,
};
use segment::*;
use sysinfo::System;
use tracing::instrument;
#[derive(Debug)]
pub struct MemorySegmentRenderer {
    info: MemoryInfo,
}

#[derive(Debug)]
pub struct MemoryInfo {
    used_memory: f64,
    available_memory: f64,
    total_memory: f64,
    warning_threshold_percent: f64,
    critical_threshold_percent: f64,
}

impl MemoryInfo {
    fn format_gb(&self, value: f64) -> String {
        if value < 2.0 {
            format!("{:.2} GB", value)
        } else {
            format!("{} GB", value.round() as u64)
        }
    }

    fn used_memory_formatted(&self) -> String {
        self.format_gb(self.used_memory)
    }

    fn available_memory_formatted(&self) -> String {
        self.format_gb(self.available_memory)
    }

    fn total_memory_formatted(&self) -> String {
        self.format_gb(self.total_memory)
    }
}

impl Info for MemoryInfo {}

#[derive(Debug)]
pub struct MemoryInfoBuilder {
    warning_threshold_percent: f64,
    critical_threshold_percent: f64,
}

impl Default for MemoryInfoBuilder {
    fn default() -> Self {
        Self {
            warning_threshold_percent: 20.0,
            critical_threshold_percent: 10.0,
        }
    }
}

impl MemoryInfoBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn warning_threshold_percent(mut self, percent: f64) -> Self {
        self.warning_threshold_percent = percent;
        self
    }

    pub fn critical_threshold_percent(mut self, percent: f64) -> Self {
        self.critical_threshold_percent = percent;
        self
    }
}

impl InfoBuilder<MemoryInfo> for MemoryInfoBuilder {
    #[instrument(skip(self), fields(builder_type = "MemoryInfoBuilder"))]
    async fn build(&self) -> Result<MemoryInfo> {
        let mut sys = System::new_all();
        sys.refresh_all();

        let bytes_to_gb = |bytes: u64| -> f64 {
            bytes as f64 / 1_073_741_824.0 // 1024^3
        };

        Ok(MemoryInfo {
            used_memory: bytes_to_gb(sys.used_memory()),
            available_memory: bytes_to_gb(sys.available_memory()),
            total_memory: bytes_to_gb(sys.total_memory()),
            warning_threshold_percent: self.warning_threshold_percent,
            critical_threshold_percent: self.critical_threshold_percent,
        })
    }
}

impl SegmentRenderer<MemoryInfo> for MemorySegmentRenderer {
    fn height(&self) -> u16 {
        1
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let [label_area, data_area] = create_label_data_layout(area);

        frame.render_widget(label("RAM"), label_area);

        let available_percentage = (self.info.available_memory / self.info.total_memory) * 100.0;
        let free_color = if available_percentage >= self.info.warning_threshold_percent {
            Color::Green
        } else if available_percentage >= self.info.critical_threshold_percent {
            Color::Yellow
        } else {
            Color::Red
        };

        let formatted_memory = Line::from(vec![
            Span::raw(format!(
                "{} used / {} total (",
                self.info.used_memory_formatted(),
                self.info.total_memory_formatted()
            )),
            Span::styled(
                self.info.available_memory_formatted(),
                Style::default().fg(free_color),
            ),
            Span::raw(" free)"),
        ]);

        frame.render_widget(Paragraph::new(formatted_memory), data_area);

        Ok(())
    }
}

impl From<Box<MemoryInfo>> for MemorySegmentRenderer {
    fn from(info: Box<MemoryInfo>) -> Self {
        Self { info: *info }
    }
}
