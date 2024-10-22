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
        1
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let [label_area, data_area] = create_label_data_layout(area);

        frame.render_widget(Paragraph::new("Temps").fg(Color::Blue).bold(), label_area);

        let mut spans = Vec::new();

        for (index, sensor) in self.info.sensors.iter().enumerate() {
            let color = match sensor.status() {
                TemperatureStatus::Critical => Color::Red,
                TemperatureStatus::High => Color::Yellow,
                TemperatureStatus::Ok => Color::Green,
            };

            spans.push(Span::styled(
                format!("{} {:.1}°C", sensor.name, sensor.temperature),
                Style::default().fg(color),
            ));

            if index < self.info.sensors.len() - 1 {
                spans.push(Span::raw(", "));
            }
        }

        let temps_line = Line::from(spans);
        let temps_paragraph = Paragraph::new(temps_line);
        frame.render_widget(temps_paragraph, data_area);

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

        let mut cpu_temp = 0.0;
        let mut cpu_count = 0;
        let mut cpu_high = 0.0;
        let mut cpu_critical = 0.0;

        let mut gpu_temp = 0.0;
        let mut gpu_count = 0;
        let mut gpu_high = 0.0;
        let mut gpu_critical = 0.0;

        let mut battery_temp = 0.0;
        let mut battery_count = 0;
        let mut battery_high = 0.0;
        let mut battery_critical = 0.0;

        for component in sys.components() {
            let name = component.label().to_lowercase();
            let temp = component.temperature();

            if name.contains("cpu") || name.contains("tdie") {
                cpu_temp += temp;
                cpu_count += 1;
                if cpu_count == 1 {
                    cpu_high = component.max();
                    cpu_critical = component.critical().unwrap_or(100.0);
                }
            } else if name.contains("gpu") {
                gpu_temp += temp;
                gpu_count += 1;
                if gpu_count == 1 {
                    gpu_high = component.max();
                    gpu_critical = component.critical().unwrap_or(100.0);
                }
            } else if name.contains("battery") {
                battery_temp += temp;
                battery_count += 1;
                if battery_count == 1 {
                    battery_high = component.max();
                    battery_critical = component.critical().unwrap_or(60.0);
                }
            }
        }

        let temperatures = vec![
            SensorTemperature {
                name: "CPU".to_string(),
                temperature: if cpu_count > 0 {
                    cpu_temp / cpu_count as f32
                } else {
                    0.0
                },
                high: cpu_high,
                critical: cpu_critical,
            },
            SensorTemperature {
                name: "GPU".to_string(),
                temperature: if gpu_count > 0 {
                    gpu_temp / gpu_count as f32
                } else {
                    0.0
                },
                high: gpu_high,
                critical: gpu_critical,
            },
            SensorTemperature {
                name: "Battery".to_string(),
                temperature: if battery_count > 0 {
                    battery_temp / battery_count as f32
                } else {
                    0.0
                },
                high: battery_high,
                critical: battery_critical,
            },
        ];

        Ok(TemperaturesInfo {
            sensors: temperatures
                .into_iter()
                .filter(|s| s.temperature > 0.0)
                .collect(),
        })
    }
}
