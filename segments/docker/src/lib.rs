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
    status: ContainerStateStatusEnum,
    exit_code: i64,
    duration_seconds: f64,
}

#[derive(Debug, Default)]
pub struct DockerInfoBuilder;

impl DockerInfoBuilder {
    fn duration_since(seconds: &f64) -> String {
        let dt = chrono::Duration::seconds(seconds.round() as i64);
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

        let duration_seconds = if let Some(time_str) = match state.status {
            Some(ContainerStateStatusEnum::RUNNING) => state.started_at,
            Some(ContainerStateStatusEnum::EXITED) => state.finished_at,
            _ => None,
        } {
            let now = Timestamp::now_utc();
            let timestamp = Timestamp::parse(&time_str).unwrap();
            (*now - *timestamp).as_seconds_f32() as f64
        } else {
            0.0
        };

        Ok(ContainerInfo {
            name: name.trim_start_matches('/').to_string(),
            status: state.status.unwrap_or(ContainerStateStatusEnum::EMPTY),
            exit_code,
            duration_seconds,
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

impl DockerSegmentRenderer {
    fn get_hours_since_exit(&self, container: &ContainerInfo) -> Option<f64> {
        if matches!(container.status, ContainerStateStatusEnum::EXITED) {
            Some(container.duration_seconds / 3600.0)
        } else {
            None
        }
    }

    fn is_container_visible(&self, container: &ContainerInfo) -> bool {
        if let Some(hours) = self.get_hours_since_exit(container) {
            hours <= 8.0
        } else {
            true // Keep non-exited containers
        }
    }
}

impl SegmentRenderer<DockerInfo> for DockerSegmentRenderer {
    fn height(&self) -> u16 {
        self.info
            .containers
            .iter()
            .filter(|c| self.is_container_visible(c))
            .count() as u16
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let chunks = create_label_data_layout(area);

        frame.render_widget(label("Docker"), chunks[0]);

        match &self.info.status {
            DockerStatus::Running => {
                let active_containers: Vec<&ContainerInfo> = self
                    .info
                    .containers
                    .iter()
                    .filter(|c| self.is_container_visible(c))
                    .collect();

                let max_name_width = active_containers
                    .iter()
                    .map(|container| container.name.len())
                    .max()
                    .unwrap_or(0)
                    + 1; // +1 for the colon

                let rows: Vec<Row> = active_containers
                    .iter()
                    .filter_map(|container| {
                        let status_style = match container.status {
                            ContainerStateStatusEnum::RUNNING => {
                                Some(Style::default().fg(Color::Green))
                            }
                            ContainerStateStatusEnum::EXITED => {
                                if let Some(hours) = self.get_hours_since_exit(container) {
                                    if hours > 8.0 {
                                        return None;
                                    }
                                }
                                Some(Style::default().fg(Color::Red))
                            }
                            _ => Some(Style::default()),
                        };

                        let status_text = match container.status {
                            ContainerStateStatusEnum::RUNNING => {
                                format!(
                                    "Up {}",
                                    DockerInfoBuilder::duration_since(&container.duration_seconds)
                                )
                            }
                            ContainerStateStatusEnum::EXITED => {
                                format!(
                                    "Exited ({}) {}",
                                    container.exit_code,
                                    DockerInfoBuilder::duration_since(&container.duration_seconds)
                                )
                            }
                            _ => container.status.to_string(),
                        };

                        Some(Row::new(vec![
                            Cell::from(format!(
                                "{:>width$}:",
                                container.name,
                                width = max_name_width - 1
                            )),
                            Cell::from(status_text).style(status_style.unwrap_or_default()),
                        ]))
                    })
                    .collect();

                let table = Table::new(rows, &[])
                    .widths(&[
                        Constraint::Length(max_name_width as u16),
                        Constraint::Percentage(100),
                    ])
                    .column_spacing(1);

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
