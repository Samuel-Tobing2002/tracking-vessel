use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn connect_db() -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:123456@localhost:5432/ship_tracking")
        .await
        .unwrap()
}