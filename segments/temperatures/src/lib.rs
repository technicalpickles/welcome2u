use anyhow::Result;
use ratatui::{
    prelude::*,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};
use sysinfo::{ComponentExt, System, SystemExt};

use segment::*;

#[derive(Debug, Default)]
pub struct TemperaturesSegmentRenderer {
    info: TemperaturesInfo,
}

#[derive(Debug, Default)]
pub struct TemperaturesInfo;

impl Info for TemperaturesInfo {}

impl SegmentRenderer<TemperaturesInfo> for TemperaturesSegmentRenderer {
    fn height(&self) -> u16 {
        1
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let mut sys = System::new_all();
        sys.refresh_components();

        let temps = sys
            .components()
            .iter()
            .filter(|component| component.label().starts_with("Core"))
            .map(|component| {
                let temp = component.temperature();
                let critical = component.critical().unwrap_or(100.0);
                let high = if critical == 100.0 {
                    80.0
                } else {
                    critical - 20.0
                };

                let color = if temp >= critical {
                    Color::Red
                } else if temp >= high {
                    Color::Yellow
                } else {
                    Color::Green
                };

                Span::styled(format!("{:.1}°C", temp), Style::default().fg(color))
            })
            .collect::<Vec<_>>();

        let temps_line = Line::from(
            temps
                .into_iter()
                .enumerate()
                .flat_map(|(i, span)| {
                    if i > 0 {
                        vec![Span::raw(", "), span]
                    } else {
                        vec![span]
                    }
                })
                .collect::<Vec<Span>>(),
        );

        let temps_paragraph = Paragraph::new(vec![temps_line]);

        let [label_area, data_area] = create_label_data_layout(area);

        frame.render_widget(
            Paragraph::new("Temperatures").style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            label_area,
        );

        frame.render_widget(temps_paragraph, data_area);

        Ok(())
    }
}

impl From<Box<TemperaturesInfo>> for TemperaturesSegmentRenderer {
    fn from(info: Box<TemperaturesInfo>) -> Self {
        Self { info: *info }
    }
}
