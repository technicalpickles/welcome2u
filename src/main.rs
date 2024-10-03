use anyhow::Context;
use anyhow::Result;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::{backend::CrosstermBackend, Terminal, TerminalOptions, Viewport};
use std::env;
use std::io::stdout;

use segment::Segment;

fn render_segments(segments: &mut [Box<dyn Segment>]) -> Result<()> {
    let backend = CrosstermBackend::new(stdout());
    let options = TerminalOptions {
        viewport: Viewport::Inline(segments.iter().map(|segment| segment.height()).sum()),
    };
    let mut terminal = Terminal::with_options(backend, options)?;

    let constraints = segments
        .iter()
        .map(|segment| Constraint::Length(segment.height()))
        .collect::<Vec<Constraint>>();

    terminal.draw(|frame| {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(frame.area());

        for (segment, area) in segments.iter().zip(layout.iter()) {
            segment.render(frame, *area).unwrap(); // Handle errors appropriately
        }
    })?;

    Ok(())
}

fn main() -> Result<()> {
    env::set_var("BASE_DIR", ".");
    env::set_var("CONFIG_PATH", "./config.sh");

    let mut segments: Vec<Box<dyn Segment>> = vec![
        // TODO: re-enable once rendering correctly
        // Box::<heading::HeadingSegment>::default(),
        Box::<quote::QuoteSegment>::default(),
        Box::new(<user::UserSegment>::default()),
        Box::new(<ip::IpSegment>::default()),
        Box::new(<os::OsSegment>::default()),
        Box::new(<uptime::UptimeSegment>::default()),
        Box::new(<load::LoadSegment>::default()),
        Box::new(<memory::MemorySegment>::default()),
        Box::new(<updates::UpdatesSegment>::default()),
        Box::<disk::DiskSegment>::default(),
        // TODO: re-enable these after testing
        // Box::<temperatures::TemperaturesSegment>::default(),
        // Box::new(<docker::DockerSegment>::default())
    ];

    for segment in segments.iter_mut() {
        segment
            .prepare()
            .with_context(|| format!("Failed to prepare segment: {:?}", segment))?;
    }

    render_segments(&mut segments)?;

    Ok(())
}
