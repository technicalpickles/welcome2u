use anyhow::Result;
use display::MotdSegment;
use ratatui::{
    TerminalOptions,
    Viewport,
    prelude::*,
    widgets::*,
};
use std::io::stdout;
use os_info;

#[derive(Default, Debug)]
pub struct OsSegment {
    info: Option<OsInfo>,
}

#[derive(Debug)]
struct OsInfo {
    os_string: String,
}

impl OsInfo {
    fn new(os_string: String) -> Self {
        Self { os_string }
    }

    fn collect() -> Self {
        let info = os_info::get();
        Self::new(info.to_string())
    }
}

impl MotdSegment for OsSegment {
    fn prepare(&mut self) -> Result<()> {
        self.info = Some(OsInfo::collect());
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
                Paragraph::new("OS").fg(Color::Blue).bold(),
                label_area,
            );

            if let Some(info) = &self.info {
                frame.render_widget(
                    Paragraph::new(info.os_string.clone()),
                    data_area,
                );
            }
        })?;

        println!();

        Ok(())
    }
}
