use ansi_term::Colour::Blue;
use fmtsize::{Conventional, FmtSize};
use ratatui::{prelude::*, widgets::*};
use std::path::Path;
use sysinfo::Disks;

use anyhow::Result;
use display::MotdSegement;

use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph,
};
use std::io::stdout;

pub struct DiskSegment {
    disks : Vec<Disk>
}

impl DiskSegment {
    pub fn new() -> Self {
        Self {
            disks: vec![]
        }
    }
}

struct Disk {
    name: String,
    mount_point: String,
    free_space: u64,
    total_space: u64,
    used_space: u64,
    percent_used: f64
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

impl MotdSegement for DiskSegment {
    fn prepare(&mut self) -> Result<()> {
        let disks = Disks::new_with_refreshed_list();

        let excluded_mount_points = [Path::new("/System/Volumes/Data")];

        self.disks = disks.into_iter().filter_map(|disk| {
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
                percent_used
            })
        }).collect();

        Ok(())
    }

    fn render(&self) -> Result<()> {
        let backend = CrosstermBackend::new(stdout());
        let options = TerminalOptions {
            viewport: Viewport::Inline(3),
        };
        let mut terminal = Terminal::with_options(backend, options)?;

        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Length(16), Constraint::Fill(1)]);

            let [label_area, data_area] = layout.areas(frame.size());

            frame.render_widget(
                Paragraph::new(Blue.bold().paint("Disk").to_string()),
                label_area,
            );

            for disk in self.disks.iter() {
                frame.render_widget(
                    LineGauge::default()
                        .block(Block::default().title(disk.format()))
                        .gauge_style(
                            Style::default()
                                .fg(Color::Red)
                                .bg(Color::Green)
                                .add_modifier(Modifier::BOLD),
                        )
                        .line_set(symbols::line::THICK)
                        .ratio(disk.percent_used),
                    data_area,
                );
            }
        })?;

        Ok(())
    }
}