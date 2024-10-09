use anyhow::Result;
use ratatui::prelude::*;
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
