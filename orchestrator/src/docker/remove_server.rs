use bollard::{container::KillContainerOptions, Docker};

pub async fn remove_server() {
    let docker = Docker::connect_with_local_defaults().unwrap(); // this will change to aws
                                                                 // sdk
    let container_id = "node-app-3";
    match docker
        .kill_container(container_id, None::<KillContainerOptions<String>>)
        .await
    {
        Ok(_) => println!("Container {} killed successfully!", container_id),
        Err(e) => println!("Error killing container: {}", e),
    }
}
