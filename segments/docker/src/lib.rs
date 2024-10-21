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
use tracing::instrument;
#[derive(Debug)]
pub struct DockerInfo {
    status: DockerStatus,
    containers: Vec<ContainerInfo>,
}

#[derive(Debug)]
pub enum DockerStatus {
    Running,
    Unavailable(String),
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
    exit_code: Option<i64>,
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
            exit_code: if state.status == Some(ContainerStateStatusEnum::EXITED) {
                Some(exit_code)
            } else {
                None
            },
        })
    }
}

impl InfoBuilder<DockerInfo> for DockerInfoBuilder {
    #[instrument(skip(self), fields(builder_type = "DockerInfoBuilder"))]
    async fn build(&self) -> Result<DockerInfo> {
        match Docker::connect_with_socket(
            "unix:///Users/josh.nichols/.colima/gusto/docker.sock",
            5,
            API_DEFAULT_VERSION,
        ) {
            Ok(docker) => {
                let options = ListContainersOptions::<String> {
                    all: true,
                    ..Default::default()
                };

                match docker.list_containers(Some(options)).await {
                    Ok(containers) => {
                        let futures = containers
                            .iter()
                            .map(|container| Self::fetch_container_info(&docker, container));

                        let containers = futures_util::future::join_all(futures)
                            .await
                            .into_iter()
                            .filter_map(Result::ok)
                            .collect();

                        Ok(DockerInfo {
                            status: DockerStatus::Running,
                            containers,
                        })
                    }
                    Err(e) => Ok(DockerInfo {
                        status: DockerStatus::Unavailable(format!(
                            "Unable to list containers: {}",
                            e
                        )),
                        containers: vec![],
                    }),
                }
            }
            Err(e) => Ok(DockerInfo {
                status: DockerStatus::Unavailable(format!(
                    "Docker is not running or not accessible: {}",
                    e
                )),
                containers: vec![],
            }),
        }
    }
}

impl SegmentRenderer<DockerInfo> for DockerSegmentRenderer {
    fn height(&self) -> u16 {
        match self.info.status {
            DockerStatus::Running => self.info.containers.len() as u16,
            DockerStatus::Unavailable(_) => 1,
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let chunks = create_label_data_layout(area);

        frame.render_widget(
            Paragraph::new("Docker").style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            chunks[0],
        );

        match &self.info.status {
            DockerStatus::Running => {
                // Calculate the width of the longest container name plus colon
                let max_name_width = self
                    .info
                    .containers
                    .iter()
                    .map(|container| container.name.len())
                    .max()
                    .unwrap_or(0)
                    + 1; // +1 for the colon

                let rows: Vec<Row> = self
                    .info
                    .containers
                    .iter()
                    .map(|container| {
                        let status_style = if container.status.starts_with("Up") {
                            Style::default()
                                .fg(Color::Green)
                                .add_modifier(Modifier::DIM)
                        } else if container.exit_code.unwrap_or(0) != 0 {
                            Style::default().fg(Color::Red).add_modifier(Modifier::DIM)
                        } else {
                            Style::default().add_modifier(Modifier::DIM)
                        };

                        Row::new(vec![
                            Cell::from(format!(
                                "{:>width$}:",
                                container.name,
                                width = max_name_width - 1
                            )),
                            Cell::from(format!("    {}", container.status)).style(status_style),
                        ])
                    })
                    .collect();

                let table = Table::new(rows, &[])
                    .widths(&[
                        Constraint::Length(max_name_width as u16),
                        Constraint::Percentage(100),
                    ])
                    .column_spacing(0);

                frame.render_widget(table, chunks[1]);
            }
            DockerStatus::Unavailable(message) => {
                frame.render_widget(
                    Paragraph::new(message.as_str()).style(Style::default().fg(Color::Red)),
                    chunks[1],
                );
            }
        }

        Ok(())
    }
}

impl From<Box<DockerInfo>> for DockerSegmentRenderer {
    fn from(info: Box<DockerInfo>) -> Self {
        Self { info: *info }
    }
}
