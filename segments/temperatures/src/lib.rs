use anyhow::Result;
use ratatui::{
    prelude::*,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};
use sysinfo::{ComponentExt, System, SystemExt};

use segment::*;

#[derive(Debug)]
pub struct TemperaturesSegmentRenderer {
    info: TemperaturesInfo,
}

// Update the TemperaturesInfo struct to include the component name
#[derive(Debug)]
pub struct TemperaturesInfo {
    temperatures: Vec<(String, f32, f32, f32)>, // (name, temperature, high, critical)
}

impl Info for TemperaturesInfo {}

impl SegmentRenderer<TemperaturesInfo> for TemperaturesSegmentRenderer {
    fn height(&self) -> u16 {
        self.info.temperatures.len() as u16 + 1 // +1 for the header
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let mut lines = vec![Line::from(vec![Span::styled(
            "Temperatures",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )])];

        for (name, temp, high, critical) in &self.info.temperatures {
            let color = if *temp >= *critical {
                Color::Red
            } else if *temp >= *high {
                Color::Yellow
            } else {
                Color::Green
            };

            let temp_span = Span::styled(format!("{:.1}°C", temp), Style::default().fg(color));

            lines.push(Line::from(vec![
                Span::raw(format!("{}: ", name)),
                temp_span,
            ]));
        }

        let temps_paragraph = Paragraph::new(lines);
        frame.render_widget(temps_paragraph, area);

        Ok(())
    }
}

impl From<Box<TemperaturesInfo>> for TemperaturesSegmentRenderer {
    fn from(info: Box<TemperaturesInfo>) -> Self {
        Self { info: *info }
    }
}

#[derive(Debug, Default)]
pub struct TemperaturesInfoBuilder;

impl InfoBuilder<TemperaturesInfo> for TemperaturesInfoBuilder {
    async fn build(&self) -> Result<TemperaturesInfo> {
        let mut sys = System::new_all();
        sys.refresh_components();

        let temperatures = sys
            .components()
            .iter()
            .map(|component| {
                let name = component.label().to_string();
                let temp = component.temperature();
                let critical = component.critical().unwrap_or(100.0);
                let high = component.max();
                (name, temp, high, critical)
            })
            .collect();

        Ok(TemperaturesInfo { temperatures })
    }
}
