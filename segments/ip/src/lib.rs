use anyhow::Result;
use display::MotdSegment;
use local_ip_address::local_ip;
use ratatui::{prelude::*, widgets::*};

#[derive(Default, Debug)]
pub struct IpSegment {
    info: Option<IpInfo>,
}

#[derive(Debug)]
struct IpInfo {
    ip_address: String,
}

impl IpInfo {
    fn new(ip_address: String) -> Self {
        Self { ip_address }
    }

    fn collect() -> Result<Self> {
        let ip = local_ip()?;
        Ok(Self::new(ip.to_string()))
    }
}

impl MotdSegment for IpSegment {
    fn prepare(&mut self) -> Result<()> {
        self.info = Some(IpInfo::collect()?);
        Ok(())
    }

    fn render(&self, frame: &mut Frame) -> Result<()> {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(16), Constraint::Fill(1)]);

        let [label_area, data_area] = layout.areas(frame.area());

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
