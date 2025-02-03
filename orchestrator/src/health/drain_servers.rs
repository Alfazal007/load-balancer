use sqlx::{Pool, Postgres};

use crate::models::server_model::Server;

pub async fn check_and_remove_servers(server_urls: &mut Vec<Server>, pool_result: &Pool<Postgres>) {
    let mut to_remove = Vec::new();

    for server in &*server_urls {
        if !crate::health::check_health::check_health(&server.server_url).await {
            let res =
                crate::health::remove_server::remove_server(&server.server_url, pool_result).await;
            if res {
                to_remove.push(server.server_url.clone());
            }
        }
    }

    server_urls.retain(|server| !to_remove.contains(&server.server_url));
}
