use anyhow::Result;
use display::Segment;
use ratatui::{prelude::*, widgets::*};

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

impl Segment for OsSegment {
    fn height(&self) -> u16 {
        1
    }

    fn prepare(&mut self) -> Result<()> {
        self.info = Some(OsInfo::collect());
        Ok(())
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(16), Constraint::Fill(1)]);

        let [label_area, data_area] = layout.areas(area);

        frame.render_widget(Paragraph::new("OS").fg(Color::Blue).bold(), label_area);

        if let Some(info) = &self.info {
            frame.render_widget(Paragraph::new(info.os_string.clone()), data_area);
        }

        Ok(())
    }
}
