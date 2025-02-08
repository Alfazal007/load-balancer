use std::{env, sync::Arc, time::Duration};

use health::query_prom::query_prom_need_more;
use models::server_model::Server;
use sqlx::postgres::PgPoolOptions;
use tokio::{sync::Mutex, time::sleep};

pub mod docker;
pub mod health;
pub mod models;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //    dotenvy::dotenv().expect("Error loading env file");
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("database url not provided");
    let pool_result = PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
        .expect("Issue connecting to the database");

    let server_urls = crate::models::active_urls::get_db_state(&pool_result).await;
    if server_urls.is_empty() {
        panic!("There are no api servers to load balance on");
    }

    let mut server_urls = Arc::new(Mutex::new(server_urls));

    loop {
        sleep(Duration::from_secs(30)).await;
        crate::health::drain_servers::check_and_remove_servers(&mut server_urls, &pool_result)
            .await;
        if server_urls.lock().await.is_empty() {
            panic!("There are no api servers to load balance on");
        }
        println!("{:?}", server_urls.lock().await);

        if server_urls.lock().await.len() <= 3 {
            let need_more_servers = query_prom_need_more().await;
            if need_more_servers && server_urls.lock().await.len() < 3 {
                let added_new_server = docker::add_server::add_server().await;
                if added_new_server.is_ok() {
                    health::add_new_server_db::add_new_server_to_db(
                        "node-app-3:8000",
                        &pool_result,
                    )
                    .await;
                    server_urls.lock().await.push(Server {
                        id: -1,
                        server_url: "node-app-3:8000".to_string(),
                    });
                }
            } else if server_urls.lock().await.len() == 3 && !need_more_servers {
                let remove_res = health::remove_server_from_db::remove_new_server_to_db(
                    "node-app-3:8000",
                    &pool_result,
                )
                .await;
                server_urls.lock().await.pop();
                if remove_res {
                    docker::remove_server::remove_server().await;
                }
            }
        }
    }
}
