use anyhow::Result;
use ratatui::{prelude::*, widgets::*};
use segment::*;

#[derive(Debug)]
pub struct OsInfo {
    os_string: String,
}

impl Info for OsInfo {}

#[derive(Debug, Default)]
pub struct OsInfoBuilder {}

impl InfoBuilder<OsInfo> for OsInfoBuilder {
    async fn build(&self) -> Result<OsInfo> {
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
    fn height(&self) -> u16 {
        1
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let [label_area, data_area] = create_label_data_layout(area);

        frame.render_widget(Paragraph::new("OS").fg(Color::Blue).bold(), label_area);

        frame.render_widget(Paragraph::new(self.info.os_string.clone()), data_area);

        Ok(())
    }
}

impl From<Box<OsInfo>> for OsSegmentRenderer {
    fn from(info: Box<OsInfo>) -> Self {
        Self { info: *info }
    }
}
