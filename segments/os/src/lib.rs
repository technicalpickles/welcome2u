use anyhow::Result;
use ratatui::{prelude::*, widgets::*};
use segment::{Info, InfoBuilder, SegmentRenderer};

#[derive(Debug)]
struct OsInfo {
    os_string: String,
}

impl Info for OsInfo {}

#[derive(Debug, Default)]
pub struct OsInfoBuilder {}

impl InfoBuilder<OsInfo> for OsInfoBuilder {
    fn build(&self) -> Result<OsInfo> {
        let info = os_info::get();
        Ok(OsInfo {
            os_string: info.to_string(),
        })
    }
}

#[derive(Debug)]
pub struct OsSegmentRenderer {
    info: OsInfo,
}

impl SegmentRenderer<OsInfo> for OsSegmentRenderer {
    fn new(info: OsInfo) -> Self {
        Self { info }
    }

    fn height(&self) -> u16 {
        1
    }

    fn prepare(&mut self) -> Result<()> {
        Ok(())
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(16), Constraint::Fill(1)]);

        let [label_area, data_area] = layout.areas(area);

        frame.render_widget(Paragraph::new("OS").fg(Color::Blue).bold(), label_area);

        frame.render_widget(Paragraph::new(self.info.os_string.clone()), data_area);

        Ok(())
    }
}
