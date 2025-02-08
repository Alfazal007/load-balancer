use bollard::{
    container::{ListContainersOptions, RemoveContainerOptions},
    Docker,
};

pub async fn remove_server() {
    let docker = Docker::connect_with_local_defaults().unwrap();

    let filters = vec![("name", vec!["node-app-3"])].into_iter().collect();
    let options = ListContainersOptions {
        all: true, // Include stopped containers
        filters,
        ..Default::default()
    };

    if let Ok(containers) = docker.list_containers(Some(options)).await {
        if let Some(container) = containers.first() {
            if let Some(id) = &container.id {
                // Now remove the container using its ID
                let _ = docker
                    .remove_container(
                        id,
                        Some(RemoveContainerOptions {
                            force: true,
                            v: true,
                            ..Default::default()
                        }),
                    )
                    .await;
            }
        }
    }
}
