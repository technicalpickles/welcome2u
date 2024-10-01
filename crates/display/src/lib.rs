use anyhow::Result;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::Paragraph,
    Terminal, TerminalOptions, Viewport,
};
use std::fmt::Debug;
use std::io::stdout;

pub trait MotdSegment: Debug {
    fn prepare(&mut self) -> Result<()> {
        Ok(())
    }
    fn render(&self) -> Result<()>;
}

#[derive(Debug)]
pub struct Single {
    content: String,
}

impl Single {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }
}

impl MotdSegment for Single {
    fn render(&self) -> Result<()> {
        let backend = CrosstermBackend::new(stdout());
        let options = TerminalOptions {
            viewport: Viewport::Inline(1),
        };
        let mut terminal = Terminal::with_options(backend, options)?;

        terminal.draw(|f| {
            let paragraph = Paragraph::new(self.content.clone()).style(Style::default());
            f.render_widget(paragraph, f.area());
        })?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct LabelWithContent {
    label: String,
    content: String,
}

impl LabelWithContent {
    pub fn new(label: &str, content: &str) -> Self {
        Self {
            label: label.to_string(),
            content: content.to_string(),
        }
    }
}

impl MotdSegment for LabelWithContent {
    fn render(&self) -> Result<()> {
        let backend = CrosstermBackend::new(stdout());
        let options = TerminalOptions {
            viewport: Viewport::Inline(1),
        };
        let mut terminal = Terminal::with_options(backend, options)?;

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(16), Constraint::Min(0)].as_ref())
                .split(f.area());

            let label = Paragraph::new(format!("{}:", self.label)).style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            );
            f.render_widget(label, chunks[0]);

            let content = Paragraph::new(self.content.clone());
            f.render_widget(content, chunks[1]);
        })?;

        Ok(())
    }
}
