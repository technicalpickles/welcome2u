use anyhow::Result;
use display::MotdSegment;
use ratatui::{
    TerminalOptions,
    Viewport,
    prelude::*,
    widgets::*,
};
use std::io::stdout;
use sysinfo::System;
use fmtsize::{Conventional, FmtSize};
use ansi_term::Colour::Blue;

#[derive(Default, Debug)]
pub struct MemorySegment {
    info: Option<MemoryInfo>,
}

#[derive(Debug)]
struct MemoryInfo {
    used_memory: String,
    available_memory: String,
    total_memory: String,
}

impl MemoryInfo {
    fn new(used_memory: String, available_memory: String, total_memory: String) -> Self {
        Self {
            used_memory,
            available_memory,
            total_memory,
        }
    }

    fn collect() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        // TODO: use consistent units, instead of letting Conventional decide
        let used_memory = sys.used_memory().fmt_size(Conventional).to_string();
        let available_memory = sys.available_memory().fmt_size(Conventional).to_string();
        let total_memory = sys.total_memory().fmt_size(Conventional).to_string();

        Self::new(used_memory, available_memory, total_memory)
    }

    fn format(&self) -> String {
        format!(
            "RAM - {} used, {} available / {}",
            self.used_memory,
            self.available_memory,
            self.total_memory
        )
    }
}

impl MotdSegment for MemorySegment {
    fn prepare(&mut self) -> Result<()> {
        self.info = Some(MemoryInfo::collect());
        Ok(())
    }

    fn render(&self) -> Result<()> {
        let backend = CrosstermBackend::new(stdout());
        let options = TerminalOptions {
            viewport: Viewport::Inline(1),
        };
        let mut terminal = Terminal::with_options(backend, options)?;

        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Length(16), Constraint::Fill(1)]);

            let [label_area, data_area] = layout.areas(frame.area());

            frame.render_widget(
                Paragraph::new(Blue.bold().paint("RAM").to_string()),
                label_area,
            );

            if let Some(info) = &self.info {
                frame.render_widget(
                    Paragraph::new(info.format()),
                    data_area,
                );
            }
        })?;

        // FIXME each segment shouldn't have to print its own newline
        println!();

        Ok(())
    }
}