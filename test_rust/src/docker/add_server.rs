use bollard::container::Config;
use bollard::container::CreateContainerOptions;
use bollard::models::{HostConfig, PortBinding};
use bollard::Docker;
use std::collections::HashMap;

pub async fn add_server() -> Result<String, String> {
    let docker = Docker::connect_with_local_defaults().expect("Issue connecting docker");

    let network_name = "load_balaner_monitoring";

    let mut port_bindings = HashMap::new();
    port_bindings.insert(
        "8000/tcp".to_string(),
        Some(vec![PortBinding {
            host_ip: Some("0.0.0.0".to_string()),
            host_port: Some("8003".to_string()),
        }]),
    );

    let mut labels = HashMap::new();
    labels.insert("PROMETHEUS_SCRAPE", "true");
    labels.insert("PROMETHEUS_PORT", "8000");

    let container_config = Config {
        image: Some("server_to_balance-node-app"),
        env: Some(vec!["PORT=8000"]),
        labels: Some(labels),
        host_config: Some(HostConfig {
            network_mode: Some(network_name.to_string()),
            port_bindings: Some(port_bindings),
            ..Default::default()
        }),
        ..Default::default()
    };

    let container = docker
        .create_container(
            Some(CreateContainerOptions {
                name: "node-app-3",
                ..Default::default()
            }),
            container_config,
        )
        .await
        .expect("Issue creating the container");

    docker
        .start_container::<String>(&container.id, None)
        .await
        .expect("Issue starting the container");
    Ok("Yo".to_string())
}
