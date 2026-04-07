use crate::models::ship::Ship;

pub fn update_ship(ship: &mut Ship) {
    ship.lat += rand::random::<f64>() / 100.0;
    ship.lon += rand::random::<f64>() / 100.0;
}