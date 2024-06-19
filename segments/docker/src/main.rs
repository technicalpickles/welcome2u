extern crate bollard;
extern crate futures_util;

use display::format_label;

use chrono_humanize::{Accuracy, HumanTime, Tense};

use bollard::{
    container::{InspectContainerOptions, ListContainersOptions},
    models::{ContainerStateStatusEnum, ContainerSummary},
    Docker, API_DEFAULT_VERSION,
};

use iso8601_timestamp::Timestamp;

use std::default::Default;

use futures_util::stream;
use futures_util::stream::StreamExt;

fn duration_since(str: &str) -> String {
    let now = Timestamp::now_utc();

    let timestamp = Timestamp::parse(str).unwrap();
    let iso8601_duration = (*now - *timestamp).as_seconds_f32();
    let dt = chrono::Duration::seconds(iso8601_duration.round() as i64);
    let ht = HumanTime::from(dt);

    ht.to_text_en(Accuracy::Rough, Tense::Present)
}

async fn conc(arg: (Docker, &ContainerSummary)) {
    let (docker, container) = arg;
    let info = docker
        .inspect_container(
            container.id.as_ref().unwrap(),
            None::<InspectContainerOptions>,
        )
        .await
        .unwrap();

    let name = info.name.unwrap();
    let state = info.state.unwrap();

    let exit_code = state.exit_code.unwrap_or(0);
    let started_at = state.started_at.as_ref().unwrap().as_str();
    let finished_at = state.finished_at.as_ref().unwrap().as_str();

    let human_status = match state.status {
        Some(ContainerStateStatusEnum::EMPTY) => "Empty".to_string(),
        Some(ContainerStateStatusEnum::CREATED) => "Created".to_string(),
        Some(ContainerStateStatusEnum::RUNNING) => {
            format!("Up {}", duration_since(started_at))
        }
        Some(ContainerStateStatusEnum::PAUSED) => "Paused".to_string(),
        Some(ContainerStateStatusEnum::RESTARTING) => "Restarting".to_string(),
        Some(ContainerStateStatusEnum::REMOVING) => "Removing".to_string(),
        Some(ContainerStateStatusEnum::EXITED) => {
            format!("Exited ({}) {} ago", exit_code, duration_since(finished_at))
        }
        Some(ContainerStateStatusEnum::DEAD) => "Dead".to_string(),
        None => String::new(),
    };

    // TODO be smarter about width
    println!("{:16}{:<40} {}", "", name, human_status)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let docker = Docker::connect_with_socket(
        "unix:///Users/josh.nichols/.colima/gusto/docker.sock",
        5,
        API_DEFAULT_VERSION,
    )
    .unwrap();

    let options = ListContainersOptions::<String> {
        all: true,
        ..Default::default()
    };

    println!("{}", format_label("Docker"));

    let containers = &docker.list_containers(Some(options)).await?;

    let docker_stream = stream::repeat(docker);
    docker_stream
        .zip(stream::iter(containers))
        .for_each_concurrent(2, conc)
        .await;

    Ok(())
}