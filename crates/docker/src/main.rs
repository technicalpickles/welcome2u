extern crate bollard;
extern crate futures_util;
extern crate timeago;

use bollard::{
    container::{InspectContainerOptions, ListContainersOptions},
    models::{ContainerStateStatusEnum,ContainerSummary},
    API_DEFAULT_VERSION,
    Docker,
};

use iso8601_timestamp::{Timestamp};

use std::time::{Duration};

use std::default::Default;

use futures_util::stream;
use futures_util::stream::StreamExt;

fn timeago(str : &str) -> String {
    let now = Timestamp::now_utc();

    let timestamp = Timestamp::parse(str).unwrap();
    let iso8601_duration = (*now - *timestamp).as_seconds_f32(); 
    let duration = Duration::from_secs_f32(iso8601_duration);

    timeago::Formatter::new().convert(duration)
}

async fn conc(arg: (Docker, &ContainerSummary)) {
    let (docker, container) = arg;
    let info = docker
            .inspect_container(
                container.id.as_ref().unwrap(),
                None::<InspectContainerOptions>
            )
            .await
            .unwrap();

    let name = info.name.as_ref().unwrap();
    let state = info.state;
    let status = match state.as_ref().unwrap().status {
            Some(ContainerStateStatusEnum::EMPTY) => format!(""),
            Some(ContainerStateStatusEnum::CREATED) => format!(""),
            Some(ContainerStateStatusEnum::RUNNING) => format!("Up"),
            Some(ContainerStateStatusEnum::PAUSED) => format!(""),
            Some(ContainerStateStatusEnum::RESTARTING) => format!(""),
            Some(ContainerStateStatusEnum::REMOVING) => format!(""),
            Some(ContainerStateStatusEnum::EXITED) => {
                let exit_code = state.as_ref().unwrap().exit_code.unwrap_or(0);
                let finished_at = state.as_ref().unwrap().finished_at.as_ref().unwrap().as_str();

                format!(
                    "Exited ({}) {}",
                    exit_code,
                    timeago(finished_at),
                )
            },
            Some(ContainerStateStatusEnum::DEAD) => format!(""),
            None => format!(""),
    };

    println!( "{}\t{}", name, status )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let docker = Docker::connect_with_socket(
        "unix:///Users/josh.nichols/.colima/gusto/docker.sock",
        5,
        API_DEFAULT_VERSION,
    ).unwrap();

    let options = ListContainersOptions::<String>{all:true, ..Default::default()};

    let containers = &docker.list_containers(Some(options)).await?;

    let docker_stream = stream::repeat(docker);
    docker_stream
        .zip(stream::iter(containers))
        .for_each_concurrent(2, conc)
        .await;

    Ok(())
}
