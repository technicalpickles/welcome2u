use anyhow::Result;
use flamescope;
use ratatui::layout::*;
use ratatui::{backend::CrosstermBackend, *};
use std::fs::File;
use std::io::stdout;
use tokio;
use tracing_flame::FlameLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

use segment::*;

async fn build_segments() -> Result<(
    heading::HeadingSegmentRenderer,
    quote::QuoteSegmentRenderer,
    user::UserSegmentRenderer,
    ip::IpSegmentRenderer,
    os::OsSegmentRenderer,
    uptime::UptimeSegmentRenderer,
    load::LoadSegmentRenderer,
    disk::DiskSegmentRenderer,
    memory::MemorySegmentRenderer,
    docker::DockerSegmentRenderer,
)> {
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

    // Unwrap results and create renderers
    Ok((
        heading::HeadingSegmentRenderer::from(Box::new(heading_info?)),
        quote::QuoteSegmentRenderer::from(Box::new(quote_info?)),
        user::UserSegmentRenderer::from(Box::new(user_info?)),
        ip::IpSegmentRenderer::from(Box::new(ip_info?)),
        os::OsSegmentRenderer::from(Box::new(os_info?)),
        uptime::UptimeSegmentRenderer::from(Box::new(uptime_info?)),
        load::LoadSegmentRenderer::from(Box::new(load_info?)),
        disk::DiskSegmentRenderer::from(Box::new(disk_info?)),
        memory::MemorySegmentRenderer::from(Box::new(memory_info?)),
        docker::DockerSegmentRenderer::from(Box::new(docker_info?)),
    ))
}

async fn render_segments(
    heading_renderer: heading::HeadingSegmentRenderer,
    quote_renderer: quote::QuoteSegmentRenderer,
    user_renderer: user::UserSegmentRenderer,
    ip_renderer: ip::IpSegmentRenderer,
    os_renderer: os::OsSegmentRenderer,
    uptime_renderer: uptime::UptimeSegmentRenderer,
    load_renderer: load::LoadSegmentRenderer,
    disk_renderer: disk::DiskSegmentRenderer,
    memory_renderer: memory::MemorySegmentRenderer,
    docker_renderer: docker::DockerSegmentRenderer,
) -> Result<()> {
    let backend = CrosstermBackend::new(stdout());

    // Create constraints
    let constraints = vec![
        Constraint::Length(heading_renderer.height()),
        Constraint::Length(quote_renderer.height()),
        Constraint::Length(user_renderer.height()),
        Constraint::Length(ip_renderer.height()),
        Constraint::Length(os_renderer.height()),
        Constraint::Length(uptime_renderer.height()),
        Constraint::Length(load_renderer.height()),
        Constraint::Length(disk_renderer.height()),
        Constraint::Length(memory_renderer.height()),
        Constraint::Length(docker_renderer.height()),
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

#[tokio::main]
async fn main() -> Result<()> {
    // Set up tracing
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let subscriber = tracing_subscriber::registry().with(env_filter);

    // Use FlameLayer only if MOTD_PROFILE is set to "debug"
    let guard = if std::env::var("MOTD_PROFILE").unwrap_or_default() == "debug" {
        let (flame_layer, guard) = FlameLayer::with_file("flame.folded").unwrap();
        subscriber.with(flame_layer).init();
        Some(guard)
    } else {
        subscriber.init();
        None
    };

    // Run your main logic
    let (
        heading_renderer,
        quote_renderer,
        user_renderer,
        ip_renderer,
        os_renderer,
        uptime_renderer,
        load_renderer,
        disk_renderer,
        memory_renderer,
        docker_renderer,
    ) = build_segments().await?;

    let result = render_segments(
        heading_renderer,
        quote_renderer,
        user_renderer,
        ip_renderer,
        os_renderer,
        uptime_renderer,
        load_renderer,
        disk_renderer,
        memory_renderer,
        docker_renderer,
    )
    .await;

    drop(guard);

    result
}
