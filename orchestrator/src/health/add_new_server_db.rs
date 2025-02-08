use sqlx::{query_as, Pool, Postgres};

use crate::models::server_model::Server;

pub async fn add_new_server_to_db(server_url: &str, pool: &Pool<Postgres>) -> bool {
    let add_server_to_db_res =
        query_as::<_, Server>("insert into servers(server_url) values ($1) returning *")
            .bind(server_url)
            .fetch_optional(pool)
            .await;

    if add_server_to_db_res.is_ok() {
        return true;
    }
    false
}
