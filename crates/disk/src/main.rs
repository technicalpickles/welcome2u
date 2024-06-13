use ansi_term::Colour::Blue;
use fmtsize::{Conventional, FmtSize};
use ratatui::{prelude::*, widgets::*};
use std::path::Path;
use sysinfo::Disks;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    let backend = CrosstermBackend::new(stdout());
    let options = TerminalOptions {
        viewport: Viewport::Inline(3),
    };
    let mut terminal = Terminal::with_options(backend, options)?;

    let disks = Disks::new_with_refreshed_list();

    let excluded_mount_points = [Path::new("/System/Volumes/Data")];

    terminal.draw(|frame| {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(16), Constraint::Fill(1)]);

        let [label_area, data_area] = layout.areas(frame.size());

        frame.render_widget(
            Paragraph::new(Blue.bold().paint("Disk").to_string()),
            label_area,
        );
        for disk in &disks {
            if excluded_mount_points.contains(&disk.mount_point()) {
                continue;
            }

            let name = disk.name().to_str().unwrap();
            let mount_point = disk.mount_point().display();

            let free_space = disk.available_space();
            let total_space = disk.total_space();
            let used_space = total_space - free_space;
            let ratio = used_space as f64 / total_space as f64;

            let free_space = free_space.fmt_size(Conventional).to_string();
            let total_space = total_space.fmt_size(Conventional).to_string();
            let used_space = used_space.fmt_size(Conventional).to_string();

            let text = format!(
                "{} ({}) - {} used, {} free / {}",
                name, mount_point, used_space, free_space, total_space
            );

            frame.render_widget(
                LineGauge::default()
                    .block(Block::default().title(text))
                    .gauge_style(
                        Style::default()
                            .fg(Color::Red)
                            .bg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    )
                    .line_set(symbols::line::THICK)
                    .ratio(ratio),
                data_area,
            );
        }
    })?;
    Ok(())
}
