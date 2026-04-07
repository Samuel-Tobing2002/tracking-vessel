use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Ship {
    pub mmsi: String,
    pub lat: f64,
    pub lon: f64,
    pub speed: f64,
}