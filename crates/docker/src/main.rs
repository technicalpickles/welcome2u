extern crate bollard;
extern crate futures_util;
extern crate timeago;

use bollard::{
    container::{InspectContainerOptions, ListContainersOptions},
    models::{ContainerStateStatusEnum,ContainerSummary},
    API_DEFAULT_VERSION,
    Docker,
};

use std::default::Default;

use futures_util::stream;
use futures_util::stream::StreamExt;

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
    let status = match info.state.as_ref().unwrap().status {
            Some(ContainerStateStatusEnum::EMPTY) => format!(""),
            Some(ContainerStateStatusEnum::CREATED) => format!(""),
            Some(ContainerStateStatusEnum::RUNNING) => format!("Up"),
            Some(ContainerStateStatusEnum::PAUSED) => format!(""),
            Some(ContainerStateStatusEnum::RESTARTING) => format!(""),
            Some(ContainerStateStatusEnum::REMOVING) => format!(""),
            Some(ContainerStateStatusEnum::EXITED) => {
                let exit_code = info.state.unwrap().exit_code.unwrap_or(0);
                // let finished_at = info.state.unwrap().finished_at.unwrap();
                // let exited_at = 
                format!(
                    "Exited ({}) {}",
                    exit_code,
                    ""
                    // timeago::Formatter::new().convert(info.state.unwrap().finished_at.unwrap().to_string())
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
