use anyhow::Result;
use ratatui::{prelude::*, widgets::*};
use segment::*;
use sysinfo::{LoadAvg, System};
use tracing::instrument;

#[derive(Default, Debug)]
pub struct LoadSegmentRenderer {
    info: LoadInfo,
}

#[derive(Debug, Default)]
pub struct LoadInfo {
    loads: LoadAvg,
    cores: usize,
}

impl Info for LoadInfo {}

#[derive(Debug, Default)]
pub struct LoadInfoBuilder {}

impl InfoBuilder<LoadInfo> for LoadInfoBuilder {
    #[instrument(skip(self), fields(builder_type = "LoadInfoBuilder"))]
    async fn build(&self) -> Result<LoadInfo> {
        let mut sys = System::new_all();
        sys.refresh_all();

        let loads = System::load_average();
        let cores = sys.physical_core_count().unwrap_or(1);
        Ok(LoadInfo { loads, cores })
    }
}

impl LoadSegmentRenderer {
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

impl SegmentRenderer<LoadInfo> for LoadSegmentRenderer {
    fn height(&self) -> u16 {
        1
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let [label_area, data_area] = create_label_data_layout(area);

        frame.render_widget(label("Load"), label_area);

        let formatted_loads = self.format_loads(&self.info);
        frame.render_widget(Paragraph::new(Line::from(formatted_loads)), data_area);

        Ok(())
    }
}

impl From<Box<LoadInfo>> for LoadSegmentRenderer {
    fn from(info: Box<LoadInfo>) -> Self {
        Self { info: *info }
    }
}
