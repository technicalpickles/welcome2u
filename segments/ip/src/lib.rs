use anyhow::Result;
use local_ip_address::local_ip;
use ratatui::{prelude::*, widgets::*};
use segment::{Info, InfoBuilder, SegmentRenderer};

#[derive(Default, Debug)]
pub struct IpSegmentRenderer {
    info: Option<IpInfo>,
}

#[derive(Debug)]
struct IpInfo {
    ip_address: String,
}

impl Info for IpInfo {}

#[derive(Debug, Default)]
struct IpInfoBuilder {}

impl InfoBuilder<IpInfo> for IpInfoBuilder {
    fn build(&self) -> Result<IpInfo> {
        let ip = local_ip()?;
        let ip_address = ip.to_string();
        Ok(IpInfo { ip_address })
    }
}

impl SegmentRenderer for IpSegmentRenderer {
    fn height(&self) -> u16 {
        1
    }

    fn prepare(&mut self) -> Result<()> {
        self.info = Some(IpInfoBuilder::default().build()?);
        Ok(())
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(16), Constraint::Fill(1)]);

        let [label_area, data_area] = layout.areas(area);

        frame.render_widget(
            Paragraph::new("IP address").fg(Color::Blue).bold(),
            label_area,
        );

        if let Some(info) = &self.info {
            frame.render_widget(Paragraph::new(info.ip_address.clone()), data_area);
        }

        Ok(())
    }
}
