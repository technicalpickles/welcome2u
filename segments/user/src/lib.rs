use anyhow::Result;
use display::MotdSegment;
use ratatui::{
    TerminalOptions,
    Viewport,
    prelude::*,
    widgets::*,
};
use std::io::stdout;
use users::{get_current_uid, get_user_by_uid};
use ansi_term::Colour::Blue;

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

impl MotdSegment for UserSegment {
    fn prepare(&mut self) -> Result<()> {
        self.info = Some(UserInfo::collect());
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
                Paragraph::new(Blue.bold().paint("User").to_string()),
                label_area,
            );

            if let Some(info) = &self.info {
                frame.render_widget(
                    Paragraph::new(info.format()),
                    data_area,
                );
            }
        })?;

        // FIXME each sgement should'nt have to print its own newline
        println!();

        Ok(())
    }
}
