use anyhow::Result;
use ratatui::{prelude::*, style::Style, widgets::Paragraph};
use std::fmt::Debug;

pub trait SegmentRenderer: Debug {
    fn prepare(&mut self) -> Result<()>;
    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()>;
    fn height(&self) -> u16;
}

pub trait Info: Debug {}

pub trait InfoBuilder<T: Info>: Debug {
    fn build(&self) -> Result<T>;
}

#[derive(Debug)]
pub struct Text {
    content: String,
}

impl Text {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }
}

impl SegmentRenderer for Text {
    fn prepare(&mut self) -> Result<()> {
        Ok(())
    }
    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let paragraph = Paragraph::new(self.content.clone()).style(Style::default());
        frame.render_widget(paragraph, area);
        Ok(())
    }
    fn height(&self) -> u16 {
        self.content.lines().count() as u16
    }
}
