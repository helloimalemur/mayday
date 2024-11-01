use crate::mayday::{Mayday, MaydayConfig};
use sea_orm::DatabaseConnection;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct AppState {
    pub mayday: Mayday,
    pub api_keys: Mutex<Vec<String>>,
    pub db_pool: DatabaseConnection,
    pub settings: Mutex<HashMap<String, String>>,
}

impl AppState {
    pub fn new(
        keys: Vec<String>,
        db_pool: DatabaseConnection,
        settings_map: HashMap<String, String>,
    ) -> AppState {
        AppState {
            mayday: Mayday::new(MaydayConfig::new()),
            api_keys: Mutex::new(keys),
            db_pool,
            settings: Mutex::new(settings_map),
        }
    }
}
