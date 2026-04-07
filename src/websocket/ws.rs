use axum::extract::ws::{WebSocket, WebSocketUpgrade, Message};
use axum::response::IntoResponse;
use tokio::time::{sleep, Duration};
use std::collections::HashMap;
use crate::models::ship::Ship;
use crate::services::redis::publish;

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

type ShipState = HashMap<String, Ship>;

fn init_ships() -> ShipState {
    let mut ships = HashMap::new();

    for i in 0..50 {
        let mmsi = format!("525{}", i);

        ships.insert(
            mmsi.clone(),
            Ship {
                mmsi,
                lat: -6.2,
                lon: 106.8,
                speed: 10.0,
            },
        );
    }

    ships
}

pub async fn handle_socket(mut socket: WebSocket) {
    let mut ships = init_ships();

    loop {
        for ship in ships.values_mut() {
            ship.lat += rand::random::<f64>() / 100.0;
            ship.lon += rand::random::<f64>() / 100.0;

            let msg = serde_json::to_string(&ship).unwrap();

            if socket.send(Message::Text(msg.clone())).await.is_err() {
                return;
            }

            publish(&ship);
        }

        sleep(Duration::from_secs(1)).await;
    }
}