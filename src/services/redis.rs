use redis::{Commands, PubSubCommands};
use crate::models::ship::Ship;
use sqlx::PgPool;

pub fn publish(ship: &Ship) {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    let msg = serde_json::to_string(ship).unwrap();
    let _: () = con.publish("ships", msg).unwrap();
}

pub async fn consume(pool: PgPool) {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    let mut pubsub = con.as_pubsub();
    pubsub.subscribe("ships").unwrap();

    println!("Listening Redis...");

    loop {
        let msg = pubsub.get_message().unwrap();
        let payload: String = msg.get_payload().unwrap();

        let ship: Ship = serde_json::from_str(&payload).unwrap();

        println!("INSERT DB: {}", ship.mmsi);

        sqlx::query(
            "INSERT INTO ships (mmsi, lat, lon, speed)
             VALUES ($1,$2,$3,$4)
             ON CONFLICT (mmsi)
             DO UPDATE SET lat=$2, lon=$3, speed=$4, updated_at=NOW()"
        )
        .bind(&ship.mmsi)
        .bind(ship.lat)
        .bind(ship.lon)
        .bind(ship.speed)
        .execute(&pool)
        .await
        .unwrap();
    }
}