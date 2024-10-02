use anyhow::Context;
use anyhow::Result;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::{backend::CrosstermBackend, Terminal, TerminalOptions, Viewport};
use std::env;
use std::io::stdout;

use display::MotdSegment;

fn render_segments(segments: &mut [Box<dyn MotdSegment>]) -> Result<()> {
    let backend = CrosstermBackend::new(stdout());
    let options = TerminalOptions {
        viewport: Viewport::Inline(segments.len() as u16 * 3), // Adjust the multiplier as needed
    };
    let mut terminal = Terminal::with_options(backend, options)?;

    terminal.draw(|frame| {
        let constraints = segments
            .iter()
            .map(|_| Constraint::Length(3))
            .collect::<Vec<Constraint>>();

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

    let mut segments: Vec<Box<dyn MotdSegment>> = vec![
        // Box::<heading::HeadingSegment>::default(),
        Box::<quote::FortuneHeaderSegment>::default(),
        Box::new(<user::UserSegment>::default()),
        Box::new(<ip::IpSegment>::default()),
        Box::new(<os::OsSegment>::default()),
        Box::new(<uptime::UptimeSegment>::default()),
        Box::new(<load::LoadSegment>::default()),
        Box::new(<memory::MemorySegment>::default()),
        // Box::<disk::DiskSegment>::default(),
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
