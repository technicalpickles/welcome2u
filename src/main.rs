use anyhow::Result;
use ratatui::layout::*;
use ratatui::{backend::CrosstermBackend, *};
use std::io::stdout;
use tokio;

use segment::*;

#[tokio::main]
async fn main() -> Result<()> {
    let backend = CrosstermBackend::new(stdout());

    // Create async tasks for building segment info
    let heading_info_future =
        tokio::spawn(async { heading::HeadingSegmentInfoBuilder::default().build() });
    let quote_info_future =
        tokio::spawn(async { Ok::<_, std::io::Error>(quote::QuoteSegmentInfo::default()) });
    let user_info_future = tokio::spawn(async { user::UserInfoBuilder::default().build() });
    let ip_info_future = tokio::spawn(async { ip::IpInfoBuilder::default().build() });
    let os_info_future = tokio::spawn(async { os::OsInfoBuilder::default().build() });
    let uptime_info_future = tokio::spawn(async { uptime::UptimeInfoBuilder::default().build() });
    let load_info_future = tokio::spawn(async { load::LoadInfoBuilder::default().build() });
    let memory_info_future = tokio::spawn(async { memory::MemoryInfoBuilder::default().build() });

    // Wait for all futures to complete
    let (
        heading_info,
        quote_info,
        user_info,
        ip_info,
        os_info,
        uptime_info,
        load_info,
        memory_info,
    ) = tokio::try_join!(
        heading_info_future,
        quote_info_future,
        user_info_future,
        ip_info_future,
        os_info_future,
        uptime_info_future,
        load_info_future,
        memory_info_future
    )?;

    // Unwrap results
    let heading_info = heading_info?;
    let quote_info = quote_info?;
    let user_info = user_info?;
    let ip_info = ip_info?;
    let os_info = os_info?;
    let uptime_info = uptime_info?;
    let load_info = load_info?;
    let memory_info = memory_info?;

    // -----

    let heading_renderer = heading::HeadingSegmentRenderer::from(Box::new(heading_info));
    let heading_constraint = Constraint::Length(heading_renderer.height());

    let quote_renderer = quote::QuoteSegmentRenderer::from(Box::new(quote_info));
    let quote_constraint = Constraint::Length(quote_renderer.height());

    let user_renderer = user::UserSegmentRenderer::from(Box::new(user_info));
    let user_constraint = Constraint::Length(user_renderer.height());

    let ip_renderer = ip::IpSegmentRenderer::from(Box::new(ip_info));
    let ip_constraint = Constraint::Length(ip_renderer.height());

    let os_renderer = os::OsSegmentRenderer::from(Box::new(os_info));
    let os_constraint = Constraint::Length(os_renderer.height());

    let uptime_renderer = uptime::UptimeSegmentRenderer::from(Box::new(uptime_info));
    let uptime_constraint = Constraint::Length(uptime_renderer.height());

    let load_renderer = load::LoadSegmentRenderer::from(Box::new(load_info));
    let load_constraint = Constraint::Length(load_renderer.height());

    let memory_renderer = memory::MemorySegmentRenderer::from(Box::new(memory_info));
    let memory_constraint = Constraint::Length(memory_renderer.height());

    let constraints = vec![
        heading_constraint,
        quote_constraint,
        user_constraint,
        ip_constraint,
        os_constraint,
        uptime_constraint,
        load_constraint,
        memory_constraint,
    ];

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
        user_renderer.render(frame, layout[2]).unwrap();
        ip_renderer.render(frame, layout[3]).unwrap();
        os_renderer.render(frame, layout[4]).unwrap();
        uptime_renderer.render(frame, layout[5]).unwrap();
        load_renderer.render(frame, layout[6]).unwrap();
        memory_renderer.render(frame, layout[7]).unwrap();
    })?;

    Ok(())
}
