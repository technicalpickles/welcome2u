use fmtsize::{Conventional, FmtSize};
use ratatui::{
    prelude::*,
    style::{Color, Style},
    widgets::*,
};
use std::path::Path;
use sysinfo::Disks;

use anyhow::Result;
use display::MotdSegment;

#[derive(Default, Debug)]
pub struct DiskSegment {
    disks: Vec<Disk>,
}

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

impl MotdSegment for DiskSegment {
    fn height(&self) -> u16 {
        (self.disks.len() * 2) as u16
    }

    fn prepare(&mut self) -> Result<()> {
        let disks = Disks::new_with_refreshed_list();

        let excluded_mount_points = [Path::new("/System/Volumes/Data")];

        self.disks = disks
            .into_iter()
            .filter_map(|disk| {
                if excluded_mount_points.contains(&disk.mount_point()) {
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

        Ok(())
    }

    fn render(&self, frame: &mut Frame<'_>, area: Rect) -> Result<()> {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(16), Constraint::Fill(1)]);

        let [label_area, data_area] = layout.areas(area);

        frame.render_widget(
            Paragraph::new("Disk").style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            label_area,
        );

        for disk in self.disks.iter() {
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
