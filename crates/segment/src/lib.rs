use anyhow::Result;
use ratatui::{prelude::*, style::Style, widgets::Paragraph};
use std::fmt::Debug;

pub trait SegmentRenderer<T: Info>: Debug {
    fn new(info: T) -> Self;

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

impl Info for Text {}

impl Text {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }
}

#[derive(Debug)]
struct TextInfoBuilder {
    content: String,
}

impl Default for TextInfoBuilder {
    fn default() -> Self {
        Self {
            content: "".to_string(),
        }
    }
}

impl TextInfoBuilder {
    pub fn content(mut self, content: String) -> Self {
        self.content = content;
        self
    }
}

impl InfoBuilder<TextInfo> for TextInfoBuilder {
    fn build(&self) -> Result<TextInfo> {
        Ok(TextInfo {
            content: self.content.clone(),
        })
    }
}

#[derive(Debug)]
pub struct TextInfo {
    content: String,
}

impl Info for TextInfo {}

impl SegmentRenderer<TextInfo> for Text {
    fn new(info: TextInfo) -> Self {
        Self {
            content: info.content,
        }
    }

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
