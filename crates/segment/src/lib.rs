use anyhow::Result;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Padding, Paragraph};
use std::fmt::Debug;

pub trait SegmentRenderer<T: Info + ?Sized>: Debug + From<Box<T>> {
    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()>;
    fn height(&self) -> u16;
}

pub trait Info: Debug {}

pub trait InfoBuilder<T: Info>: Debug {
    fn build(&self) -> impl std::future::Future<Output = Result<T>> + Send;
}

pub fn create_label_data_layout(area: Rect) -> [Rect; 3] {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Length(16),
            Constraint::Fill(1),
            Constraint::Length(4),
        ]);

    layout.areas(area)
}

pub fn label(text: &str) -> Paragraph<'_> {
    Paragraph::new(text)
        .fg(Color::Blue)
        .bold()
        .alignment(Alignment::Right)
        .block(Block::default().padding(Padding::new(0, 2, 0, 0)))
}
