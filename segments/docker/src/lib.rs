use anyhow::Result;
use bollard::{
    container::{InspectContainerOptions, ListContainersOptions},
    models::{ContainerStateStatusEnum, ContainerSummary},
    Docker, API_DEFAULT_VERSION,
};
use chrono_humanize::{Accuracy, HumanTime, Tense};
use iso8601_timestamp::Timestamp;
use ratatui::{prelude::*, widgets::*};
use segment::*;
use std::default::Default;

#[derive(Debug, Default)]
pub struct DockerInfo {
    containers: Vec<ContainerInfo>,
}

impl Info for DockerInfo {}

#[derive(Debug)]
pub struct DockerSegmentRenderer {
    info: DockerInfo,
}

#[derive(Debug)]
struct ContainerInfo {
    name: String,
    status: String,
}

#[derive(Debug, Default)]
pub struct DockerInfoBuilder;

impl DockerInfoBuilder {
    fn duration_since(str: &str) -> String {
        let now = Timestamp::now_utc();
        let timestamp = Timestamp::parse(str).unwrap();
        let iso8601_duration = (*now - *timestamp).as_seconds_f32();
        let dt = chrono::Duration::seconds(iso8601_duration.round() as i64);
        let ht = HumanTime::from(dt);
        ht.to_text_en(Accuracy::Rough, Tense::Present)
    }

    async fn fetch_container_info(
        docker: &Docker,
        container: &ContainerSummary,
    ) -> Result<ContainerInfo> {
        let info = docker
            .inspect_container(
                container.id.as_ref().unwrap(),
                None::<InspectContainerOptions>,
            )
            .await?;

        let name = info.name.unwrap();
        let state = info.state.unwrap();

        let exit_code = state.exit_code.unwrap_or(0);
        let started_at = state.started_at.as_ref().unwrap().as_str();
        let finished_at = state.finished_at.as_ref().unwrap().as_str();

        let status = match state.status {
            Some(ContainerStateStatusEnum::EMPTY) => "Empty".to_string(),
            Some(ContainerStateStatusEnum::CREATED) => "Created".to_string(),
            Some(ContainerStateStatusEnum::RUNNING) => {
                format!("Up {}", Self::duration_since(started_at))
            }
            Some(ContainerStateStatusEnum::PAUSED) => "Paused".to_string(),
            Some(ContainerStateStatusEnum::RESTARTING) => "Restarting".to_string(),
            Some(ContainerStateStatusEnum::REMOVING) => "Removing".to_string(),
            Some(ContainerStateStatusEnum::EXITED) => {
                format!(
                    "Exited ({}) {} ago",
                    exit_code,
                    Self::duration_since(finished_at)
                )
            }
            Some(ContainerStateStatusEnum::DEAD) => "Dead".to_string(),
            None => String::new(),
        };

        Ok(ContainerInfo {
            name: name.trim_start_matches('/').to_string(),
            status,
        })
    }
}

impl InfoBuilder<DockerInfo> for DockerInfoBuilder {
    async fn build(&self) -> Result<DockerInfo> {
        let docker = Docker::connect_with_socket(
            "unix:///Users/josh.nichols/.colima/gusto/docker.sock",
            5,
            API_DEFAULT_VERSION,
        )?;

        let options = ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        };

        let containers = docker.list_containers(Some(options)).await?;

        let futures = containers
            .iter()
            .map(|container| Self::fetch_container_info(&docker, container));

        let containers = futures_util::future::join_all(futures)
            .await
            .into_iter()
            .filter_map(Result::ok)
            .collect();

        Ok(DockerInfo { containers })
    }
}

impl SegmentRenderer<DockerInfo> for DockerSegmentRenderer {
    fn height(&self) -> u16 {
        self.info.containers.len() as u16
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        // TODO: figure out how to use create_label_data_layout here
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(area);

        frame.render_widget(
            Paragraph::new("Docker").style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            chunks[0],
        );

        let items: Vec<ListItem> = self
            .info
            .containers
            .iter()
            .map(|container| ListItem::new(format!("{:<40} {}", container.name, container.status)))
            .collect();

        let list = List::new(items);
        frame.render_widget(list, chunks[1]);

        Ok(())
    }
}

impl From<Box<DockerInfo>> for DockerSegmentRenderer {
    fn from(info: Box<DockerInfo>) -> Self {
        Self { info: *info }
    }
}
