use sqlx::{query, Pool, Postgres};

pub async fn remove_new_server_to_db(server_url: &str, pool: &Pool<Postgres>) -> bool {
    let remove_server_from_db_res = query("delete from servers where server_url=$1")
        .bind(server_url)
        .fetch_optional(pool)
        .await;
    println!("Remove server result {:?}", remove_server_from_db_res);

    if remove_server_from_db_res.is_ok() {
        return true;
    }
    false
}
