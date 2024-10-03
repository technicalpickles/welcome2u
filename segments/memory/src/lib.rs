use ansi_term::Colour::Blue;
use anyhow::Result;
use display::Segment;
use fmtsize::{Conventional, FmtSize};
use ratatui::{prelude::*, widgets::*};
use sysinfo::System;

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
            self.used_memory, self.available_memory, self.total_memory
        )
    }
}

impl Segment for MemorySegment {
    fn height(&self) -> u16 {
        1
    }

    fn prepare(&mut self) -> Result<()> {
        self.info = Some(MemoryInfo::collect());
        Ok(())
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(16), Constraint::Fill(1)]);

        let [label_area, data_area] = layout.areas(area);

        frame.render_widget(
            Paragraph::new(Blue.bold().paint("RAM").to_string()),
            label_area,
        );

        if let Some(info) = &self.info {
            frame.render_widget(Paragraph::new(info.format()), data_area);
        }

        Ok(())
    }
}
