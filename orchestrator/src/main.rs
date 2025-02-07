use std::{env, sync::Arc, time::Duration};

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
        // check the usage of items
        // based on gauge number increase or decrease the load balancer count
        println!("{:?}", server_urls.lock().await);
        //query_prom().await;
    }
}
