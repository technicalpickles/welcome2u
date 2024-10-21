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
        tokio::spawn(async { heading::HeadingSegmentInfoBuilder::default().build().await });
    let quote_info_future =
        tokio::spawn(async { quote::QuoteInfoBuilder::default().build().await });
    let user_info_future = tokio::spawn(async { user::UserInfoBuilder::default().build().await });
    let ip_info_future = tokio::spawn(async { ip::IpInfoBuilder::default().build().await });
    let os_info_future = tokio::spawn(async { os::OsInfoBuilder::default().build().await });
    let uptime_info_future =
        tokio::spawn(async { uptime::UptimeInfoBuilder::default().build().await });
    let load_info_future = tokio::spawn(async { load::LoadInfoBuilder::default().build().await });
    let disk_info_future = tokio::spawn(async {
        disk::DiskInfoBuilder::default()
            .exclude_mount_point("/System/Volumes/Data".to_string())
            .build()
            .await
    });
    let memory_info_future =
        tokio::spawn(async { memory::MemoryInfoBuilder::default().build().await });
    let docker_info_future =
        tokio::spawn(async { docker::DockerInfoBuilder::default().build().await });

    // Wait for all futures to complete
    let (
        heading_info,
        quote_info,
        user_info,
        ip_info,
        os_info,
        uptime_info,
        load_info,
        disk_info,
        memory_info,
        docker_info,
    ) = tokio::try_join!(
        heading_info_future,
        quote_info_future,
        user_info_future,
        ip_info_future,
        os_info_future,
        uptime_info_future,
        load_info_future,
        disk_info_future,
        memory_info_future,
        docker_info_future
    )?;

    // Unwrap results
    let heading_info = heading_info?;
    let quote_info = quote_info?;
    let user_info = user_info?;
    let ip_info = ip_info?;
    let os_info = os_info?;
    let uptime_info = uptime_info?;
    let load_info = load_info?;
    let disk_info = disk_info?;
    let memory_info = memory_info?;
    let docker_info = docker_info?;

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

    let disk_renderer = disk::DiskSegmentRenderer::from(Box::new(disk_info));
    let disk_constraint = Constraint::Length(disk_renderer.height());

    let memory_renderer = memory::MemorySegmentRenderer::from(Box::new(memory_info));
    let memory_constraint = Constraint::Length(memory_renderer.height());

    let docker_renderer = docker::DockerSegmentRenderer::from(Box::new(docker_info));
    let docker_constraint = Constraint::Length(docker_renderer.height());

    let constraints = vec![
        heading_constraint,
        quote_constraint,
        user_constraint,
        ip_constraint,
        os_constraint,
        uptime_constraint,
        load_constraint,
        disk_constraint,
        memory_constraint,
        docker_constraint,
    ];

    let options = TerminalOptions {
        viewport: Viewport::Inline(
            constraints
                .iter()
                .map(|c| match c {
                    Constraint::Length(l) => *l,
                    _ => panic!("All constraints must be Constraint::Length"),
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
        disk_renderer.render(frame, layout[7]).unwrap();
        memory_renderer.render(frame, layout[8]).unwrap();
        docker_renderer.render(frame, layout[9]).unwrap();
    })?;

    Ok(())
}
