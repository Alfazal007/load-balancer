use sqlx::{Pool, Postgres};

use super::server_model::Server;

pub async fn get_db_state(pool_result: &Pool<Postgres>) -> Vec<Server> {
    let servers_result: Result<Vec<Server>, _> =
        sqlx::query_as::<_, Server>("select * from servers")
            .fetch_all(pool_result)
            .await;
    if servers_result.is_err() {
        panic!("Issue fetching the database data");
    }
    servers_result.unwrap()
}
