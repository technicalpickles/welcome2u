use ratatui::{
    prelude::*,
    style::{Color, Style},
    widgets::*,
};
use sysinfo::Disks;

use anyhow::Result;
use segment::*;
use tracing::instrument;

#[derive(Debug)]
pub struct DiskSegmentRenderer {
    info: DiskInfo,
}

#[derive(Debug)]
pub struct DiskInfo {
    disks: Vec<Disk>,
    warning_threshold_percent: f64,
    critical_threshold_percent: f64,
}

impl Default for DiskInfo {
    fn default() -> Self {
        Self {
            disks: Vec::new(),
            warning_threshold_percent: 80.0,
            critical_threshold_percent: 90.0,
        }
    }
}

#[derive(Debug)]
struct Disk {
    name: String,
    mount_point: String,
    free_space: u64,
    total_space: u64,
    used_space: u64,
}

impl Disk {
    fn format_gb(&self, value: u64) -> String {
        let gb = value as f64 / 1_073_741_824.0;
        if gb < 2.0 {
            format!("{:.2} GB", gb)
        } else {
            format!("{} GB", gb.round() as u64)
        }
    }

    fn used_space_formatted(&self) -> String {
        self.format_gb(self.used_space)
    }

    fn free_space_formatted(&self) -> String {
        self.format_gb(self.free_space)
    }

    fn total_space_formatted(&self) -> String {
        self.format_gb(self.total_space)
    }

    fn percent_used(&self) -> f64 {
        self.used_space as f64 / self.total_space as f64 * 100.0
    }

    fn percent_free(&self) -> f64 {
        self.free_space as f64 / self.total_space as f64 * 100.0
    }
}

impl Info for DiskInfo {}

#[derive(Debug)]
pub struct DiskInfoBuilder {
    excluded_mount_points: Vec<String>,
    warning_threshold_percent: f64,
    critical_threshold_percent: f64,
}

impl Default for DiskInfoBuilder {
    fn default() -> Self {
        Self {
            excluded_mount_points: Vec::new(),
            warning_threshold_percent: 85.0,
            critical_threshold_percent: 95.0,
        }
    }
}

impl Default for DiskSegmentRenderer {
    fn default() -> Self {
        Self {
            info: DiskInfo::default(),
        }
    }
}

impl DiskInfoBuilder {
    pub fn exclude_mount_point(mut self, mount_point: String) -> Self {
        self.excluded_mount_points.push(mount_point);
        self
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

impl InfoBuilder<DiskInfo> for DiskInfoBuilder {
    #[instrument(skip(self), fields(builder_type = "DiskInfoBuilder"))]
    async fn build(&self) -> Result<DiskInfo> {
        let disks = Disks::new_with_refreshed_list();

        let disks = disks
            .into_iter()
            .filter_map(|disk| {
                if self
                    .excluded_mount_points
                    .contains(&disk.mount_point().to_str().unwrap().to_string())
                {
                    return None;
                }

                let name = disk.name().to_str().unwrap().to_string();
                let mount_point = disk.mount_point().to_str().unwrap().to_string();

                let free_space = disk.available_space();
                let total_space = disk.total_space();
                let used_space = total_space - free_space;

                Some(Disk {
                    name,
                    mount_point,
                    free_space,
                    total_space,
                    used_space,
                })
            })
            .collect();

        Ok(DiskInfo {
            disks,
            warning_threshold_percent: self.warning_threshold_percent,
            critical_threshold_percent: self.critical_threshold_percent,
        })
    }
}

impl SegmentRenderer<DiskInfo> for DiskSegmentRenderer {
    fn height(&self) -> u16 {
        (self.info.disks.len() * 2) as u16
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let [label_area, data_area, _padding] = create_label_data_layout(area);

        frame.render_widget(label("Disk"), label_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                self.info
                    .disks
                    .iter()
                    .map(|_| Constraint::Length(2))
                    .collect::<Vec<_>>(),
            )
            .split(data_area);

        for (disk, chunk) in self.info.disks.iter().zip(chunks.iter()) {
            let used_percentage = disk.percent_used();
            let free_percentage = 100.0 - used_percentage;

            let usage_color = if used_percentage >= self.info.critical_threshold_percent {
                Color::Red
            } else if used_percentage >= self.info.warning_threshold_percent {
                Color::Yellow
            } else {
                Color::Green
            };

            let summary = Line::from(vec![
                Span::raw(format!(
                    "{} ({}) - {} used / {} total (",
                    disk.name,
                    disk.mount_point,
                    disk.used_space_formatted(),
                    disk.total_space_formatted()
                )),
                Span::styled(
                    format!("{} free", disk.free_space_formatted()),
                    Style::default().fg(usage_color),
                ),
                Span::raw(")"),
            ]);

            frame.render_widget(
                LineGauge::default()
                    .block(Block::default().title(summary))
                    .filled_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                    .unfilled_style(Style::default().fg(Color::Green))
                    .line_set(symbols::line::THICK)
                    .ratio(free_percentage / 100.0),
                *chunk,
            );
        }

        Ok(())
    }
}

impl From<Box<DiskInfo>> for DiskSegmentRenderer {
    fn from(info: Box<DiskInfo>) -> Self {
        Self { info: *info }
    }
}
