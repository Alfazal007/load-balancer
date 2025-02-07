use bollard::{container::KillContainerOptions, Docker};

pub async fn remove_server() {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let container_id = "node-app-3";
    match docker
        .kill_container(container_id, None::<KillContainerOptions<String>>)
        .await
    {
        Ok(_) => println!("Container {} killed successfully!", container_id),
        Err(e) => eprintln!("Error killing container: {}", e),
    }
}
