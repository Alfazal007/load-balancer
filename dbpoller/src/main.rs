use std::{env, time::Duration};

use database::get_data::get_db_state;
use in_memory_server_urls::{add_server::add_server, remove_server::remove_server};
use redis::Commands;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use tokio::time::sleep;

pub mod database;
pub mod in_memory_server_urls;

#[derive(Serialize, Deserialize)]
struct Message {
    url: String,
    #[serde(rename = "type")]
    msg_type: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("database url not provided");
    let redis_url = env::var("REDIS_URL").expect("redis url not provided");

    let pool_result = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("Issue connecting to the database");

    let redis_client = redis::Client::open(redis_url).expect("Issue opening redis connection");

    let mut server_urls = get_db_state(&pool_result).await;

    let mut redis_con = redis_client
        .get_connection()
        .expect("Issue connecting to redis");

    loop {
        sleep(Duration::from_secs(15)).await;
        let current_server_state = get_db_state(&pool_result).await;

        match current_server_state.len().cmp(&server_urls.len()) {
            std::cmp::Ordering::Less => {
                let server_to_remove = remove_server(&server_urls, &current_server_state);
                println!("Server to remove {:?}", server_to_remove);
                for server in server_to_remove.iter() {
                    let message = json!({
                        "url": server.server_url,
                        "type": "remove"
                    });

                    let _: () = redis_con
                        .publish("server-update", message.to_string())
                        .unwrap();
                }
            }
            std::cmp::Ordering::Greater => {
                let server_to_add = add_server(&server_urls, &current_server_state);
                println!("Server to add {:?}", server_to_add);
                for server in server_to_add.iter() {
                    let message = json!({
                        "url": server.server_url,
                        "type": "add"
                    });

                    let _: () = redis_con
                        .publish("server-update", message.to_string())
                        .unwrap();
                }
            }
            _ => (),
        }
        server_urls = current_server_state;
        println!("New server list {:?}", server_urls);
    }
}
