use sea_orm::DatabaseConnection;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct AppState {
    pub api_key: Mutex<Vec<String>>,
    pub(crate) db_pool: Mutex<DatabaseConnection>,
    pub(crate) settings: Mutex<HashMap<String, String>>,
}

impl AppState {
    pub fn new(
        keys: Vec<String>,
        db_pool: DatabaseConnection,
        settings_map: HashMap<String, String>,
    ) -> AppState {
        AppState {
            api_key: Mutex::new(keys),
            db_pool: Mutex::new(db_pool),
            settings: Mutex::new(settings_map),
        }
    }
}
