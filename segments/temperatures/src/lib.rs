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
pub enum TemperatureStatus {
    Ok,
    High,
    Critical,
}

#[derive(Debug)]
pub struct SensorTemperature {
    name: String,
    temperature: f32,
    high: f32,
    critical: f32,
}

impl SensorTemperature {
    fn status(&self) -> TemperatureStatus {
        if self.temperature >= self.critical {
            TemperatureStatus::Critical
        } else if self.temperature >= self.high {
            TemperatureStatus::High
        } else {
            TemperatureStatus::Ok
        }
    }
}

#[derive(Debug)]
pub struct TemperaturesInfo {
    sensors: Vec<SensorTemperature>,
}

impl Info for TemperaturesInfo {}

#[derive(Debug)]
pub struct TemperaturesSegmentRenderer {
    info: Box<TemperaturesInfo>,
}

impl SegmentRenderer<TemperaturesInfo> for TemperaturesSegmentRenderer {
    fn height(&self) -> u16 {
        self.info.sensors.len() as u16 + 1 // +1 for the header
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let mut lines = vec![Line::from(vec![Span::styled(
            "Temperatures",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )])];

        for sensor in &self.info.sensors {
            let color = match sensor.status() {
                TemperatureStatus::Critical => Color::Red,
                TemperatureStatus::High => Color::Yellow,
                TemperatureStatus::Ok => Color::Green,
            };

            let temp_span = Span::styled(
                format!("{:.1}°C", sensor.temperature),
                Style::default().fg(color),
            );

            lines.push(Line::from(vec![
                Span::raw(format!("{}: ", sensor.name)),
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
        Self { info: info }
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
                SensorTemperature {
                    name,
                    temperature: temp,
                    high,
                    critical,
                }
            })
            .collect();

        Ok(TemperaturesInfo {
            sensors: temperatures,
        })
    }
}
