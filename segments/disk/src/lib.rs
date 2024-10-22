use fmtsize::{Conventional, FmtSize};
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
struct Disk {
    name: String,
    mount_point: String,
    free_space: u64,
    total_space: u64,
    used_space: u64,
    percent_used: f64,
}

impl Disk {
    fn format(&self) -> String {
        let free_space = self.free_space.fmt_size(Conventional).to_string();
        let total_space = self.total_space.fmt_size(Conventional).to_string();
        let used_space = self.used_space.fmt_size(Conventional).to_string();

        format!(
            "{} ({}) - {} used, {} free / {}",
            self.name, self.mount_point, used_space, free_space, total_space
        )
    }
}

#[derive(Default, Debug)]
pub struct DiskInfo {
    disks: Vec<Disk>,
}

impl Info for DiskInfo {}

#[derive(Debug, Default)]
pub struct DiskSegmentRenderer {
    info: DiskInfo,
}

#[derive(Debug, Default)]
pub struct DiskInfoBuilder {
    excluded_mount_points: Vec<String>,
}

impl DiskInfoBuilder {
    pub fn exclude_mount_point(mut self, mount_point: String) -> Self {
        self.excluded_mount_points.push(mount_point);
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
                let percent_used = used_space as f64 / total_space as f64;

                Some(Disk {
                    name,
                    mount_point,
                    free_space,
                    total_space,
                    used_space,
                    percent_used,
                })
            })
            .collect();

        Ok(DiskInfo { disks })
    }
}

impl SegmentRenderer<DiskInfo> for DiskSegmentRenderer {
    fn height(&self) -> u16 {
        (self.info.disks.len() * 2) as u16
    }

    fn render(&self, frame: &mut Frame<'_>, area: Rect) -> Result<()> {
        let [label_area, data_area] = create_label_data_layout(area);

        frame.render_widget(label("Disk"), label_area);

        for disk in self.info.disks.iter() {
            frame.render_widget(
                LineGauge::default()
                    .block(Block::default().title(disk.format()))
                    .filled_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                    .unfilled_style(Style::default().fg(Color::Green))
                    .line_set(symbols::line::THICK)
                    .ratio(disk.percent_used),
                data_area,
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
