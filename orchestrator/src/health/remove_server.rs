use sqlx::{Pool, Postgres};

pub async fn remove_server(server_url: &str, pool: &Pool<Postgres>) -> bool {
    let server_remove_result = sqlx::query("delete from servers where server_url=$1")
        .bind(server_url)
        .fetch_optional(pool)
        .await;
    if server_remove_result.is_err() {
        return false;
    }
    true
}
