use axum::{Json, extract::State};
use sqlx::PgPool;
use crate::models::ship::Ship;

pub async fn get_ships(
    State(pool): State<PgPool>,
) -> Json<Vec<Ship>> {
    let ships = sqlx::query_as::<_, Ship>(
        "SELECT mmsi, lat, lon, speed FROM ships"
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(ships)
}