use anyhow::Result;
use ratatui::{prelude::*, widgets::*};
use segment::{Info, InfoBuilder, SegmentRenderer};

#[derive(Default, Debug)]
pub struct OsSegmentRenderer {
    info: Option<OsInfo>,
}

#[derive(Debug)]
struct OsInfo {
    os_string: String,
}

impl Info for OsInfo {}

#[derive(Debug, Default)]
struct OsInfoBuilder {}

impl InfoBuilder<OsInfo> for OsInfoBuilder {
    fn build(&self) -> Result<OsInfo> {
        let info = os_info::get();
        Ok(OsInfo {
            os_string: info.to_string(),
        })
    }
}

impl SegmentRenderer for OsSegmentRenderer {
    fn height(&self) -> u16 {
        1
    }

    fn prepare(&mut self) -> Result<()> {
        self.info = Some(OsInfoBuilder::default().build()?);
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
