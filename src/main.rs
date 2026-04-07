mod db;
mod models;
mod handlers;
mod services;
mod websocket;

use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use db::postgres::connect_db;
use handlers::ship_handler::get_ships;
use websocket::ws::ws_handler;
use services::redis::consume;
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    let pool: PgPool = connect_db().await;

    tokio::spawn(consume(pool.clone()));

    let app = Router::new()
        .route("/ships", get(get_ships))
        .route("/ws", get(ws_handler))
        .with_state(pool.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("🚀 Server running on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}