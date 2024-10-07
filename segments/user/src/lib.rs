use anyhow::Result;
use ratatui::{prelude::*, widgets::*};
use segment::*;
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
    fn user_with_hostname(&self) -> String {
        format!("{}@{}", self.username, self.hostname)
    }
}

impl Info for UserInfo {}

#[derive(Debug, Default)]
struct UserInfoBuilder {}

impl InfoBuilder<UserInfo> for UserInfoBuilder {
    fn build(&self) -> Result<UserInfo> {
        let user = get_user_by_uid(get_current_uid()).unwrap();
        let username = user.name().to_str().unwrap();

        let hostname = hostname::get().unwrap();
        let hostname_str = hostname.to_str().unwrap();

        Ok(UserInfo {
            username: username.to_string(),
            hostname: hostname_str.to_string(),
        })
    }
}

impl Segment for UserSegment {
    fn height(&self) -> u16 {
        1
    }

    fn prepare(&mut self) -> Result<()> {
        self.info = Some(UserInfoBuilder::default().build()?);
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
            frame.render_widget(Paragraph::new(info.user_with_hostname()), data_area);
        }

        Ok(())
    }
}
