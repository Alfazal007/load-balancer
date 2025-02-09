use sqlx::{Pool, Postgres};

#[derive(serde::Deserialize, Debug, sqlx::FromRow, PartialEq)]
pub struct Server {
    pub id: i32,
    pub server_url: String,
}

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
