use anyhow::Result;
use ratatui::{prelude::*, widgets::*};
use segment::{Info, InfoBuilder, Segment};
use sysinfo::{LoadAvg, System};

#[derive(Default, Debug)]
pub struct LoadSegment {
    info: Option<LoadInfo>,
}

#[derive(Debug, Default)]
struct LoadInfo {
    loads: LoadAvg,
    cores: usize,
}

impl Info for LoadInfo {}

#[derive(Debug, Default)]
struct LoadInfoBuilder {}

impl InfoBuilder<LoadInfo> for LoadInfoBuilder {
    fn build(&self) -> Result<LoadInfo> {
        let mut sys = System::new_all();
        sys.refresh_all();

        let loads = System::load_average();
        let cores = sys.physical_core_count().unwrap_or(1);
        Ok(LoadInfo { loads, cores })
    }
}

impl LoadSegment {
    fn format_loads(&self, info: &LoadInfo) -> Vec<Span> {
        let warning_threshold = info.cores as f64 * 0.9;
        let error_threshold = info.cores as f64 * 1.5;

        let colored_loads: Vec<Span> = [info.loads.one, info.loads.five, info.loads.fifteen]
            .iter()
            .map(|&load| {
                let content = format!("{:.2}", load);
                if load < warning_threshold {
                    Span::styled(content, Style::default().fg(Color::Green))
                } else if load < error_threshold {
                    Span::styled(content, Style::default().fg(Color::Yellow))
                } else {
                    Span::styled(content, Style::default().fg(Color::Red))
                }
            })
            .collect();

        vec![
            colored_loads[0].clone(),
            Span::raw(", "),
            colored_loads[1].clone(),
            Span::raw(", "),
            colored_loads[2].clone(),
            Span::raw(format!(" (across {} cores)", info.cores)),
        ]
    }
}

impl Segment for LoadSegment {
    fn height(&self) -> u16 {
        1
    }

    fn prepare(&mut self) -> Result<()> {
        self.info = Some(LoadInfoBuilder::default().build()?);
        Ok(())
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(16), Constraint::Fill(1)]);

        let [label_area, data_area] = layout.areas(area);

        frame.render_widget(
            Paragraph::new("Load average").style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            label_area,
        );

        if let Some(info) = &self.info {
            let formatted_loads = self.format_loads(info);
            frame.render_widget(Paragraph::new(Line::from(formatted_loads)), data_area);
        }

        Ok(())
    }
}
