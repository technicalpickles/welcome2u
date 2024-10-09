use anyhow::Result;
use futures::future::join_all;
use ratatui::layout::*;
use ratatui::{backend::CrosstermBackend, *};
use std::io::stdout;
use tokio;

use segment::*;

#[tokio::main]
async fn main() -> Result<()> {
    let backend = CrosstermBackend::new(stdout());

    // Create async tasks for building segment info
    let ip_info_future = tokio::spawn(async { ip::IpInfoBuilder::default().build() });
    let heading_info_future =
        tokio::spawn(async { heading::HeadingSegmentInfoBuilder::default().build() });
    let quote_info_future =
        tokio::spawn(async { Ok::<_, std::io::Error>(quote::QuoteSegmentInfo::default()) });

    // Wait for all futures to complete
    let (ip_info, heading_info, quote_info) =
        tokio::try_join!(ip_info_future, heading_info_future, quote_info_future)?;

    // Unwrap results
    let ip_info = ip_info?;
    let heading_info = heading_info?;
    let quote_info = quote_info?;

    // -----

    let heading_renderer = heading::HeadingSegmentRenderer::from(Box::new(heading_info));
    let heading_constraint = Constraint::Length(heading_renderer.height());

    let ip_renderer = ip::IpSegmentRenderer::from(Box::new(ip_info));
    let ip_constraint = Constraint::Length(ip_renderer.height());

    let quote_renderer = quote::QuoteSegmentRenderer::from(Box::new(quote_info));
    let quote_constraint = Constraint::Length(quote_renderer.height());

    let constraints = vec![heading_constraint, quote_constraint, ip_constraint];

    let options = TerminalOptions {
        viewport: Viewport::Inline(
            constraints
                .iter()
                .map(|c| match c {
                    Constraint::Length(l) => *l,
                    _ => 0,
                })
                .sum(),
        ),
    };

    let mut terminal = Terminal::with_options(backend, options)?;
    terminal.draw(|frame| {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(frame.area());

        heading_renderer.render(frame, layout[0]).unwrap();
        quote_renderer.render(frame, layout[1]).unwrap();
        ip_renderer.render(frame, layout[2]).unwrap();
    })?;

    Ok(())
}
