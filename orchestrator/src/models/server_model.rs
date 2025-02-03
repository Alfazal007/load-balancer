#[derive(serde::Deserialize, Debug, sqlx::FromRow)]
pub struct Server {
    pub id: i32,
    pub server_url: String,
}
