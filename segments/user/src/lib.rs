use anyhow::Result;
use display::Segment;
use ratatui::{prelude::*, widgets::*};
use users::{get_current_uid, get_user_by_uid};

#[derive(Default, Debug)]
pub struct UserSegment {
    info: Option<UserInfo>,
}

#[derive(Debug)]
struct UserInfo {
    username: String,
    hostname: String,
}

impl UserInfo {
    fn new(username: String, hostname: String) -> Self {
        Self { username, hostname }
    }

    fn collect() -> Self {
        let user = get_user_by_uid(get_current_uid()).unwrap();
        let username = user.name().to_str().unwrap();

        let hostname = hostname::get().unwrap();
        let hostname_str = hostname.to_str().unwrap();

        Self::new(username.to_string(), hostname_str.to_string())
    }

    fn format(&self) -> String {
        format!("{}@{}", self.username, self.hostname)
    }
}

impl Segment for UserSegment {
    fn height(&self) -> u16 {
        1
    }

    fn prepare(&mut self) -> Result<()> {
        self.info = Some(UserInfo::collect());
        Ok(())
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(16), Constraint::Fill(1)]);

        let [label_area, data_area] = layout.areas(area);

        frame.render_widget(
            Paragraph::new("User").style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            label_area,
        );

        if let Some(info) = &self.info {
            frame.render_widget(Paragraph::new(info.format()), data_area);
        }

        Ok(())
    }
}
