use sea_orm::DatabaseConnection;
use std::collections::HashMap;

pub struct Mayday {
    mayday: String,
    config: MaydayConfig,
}

impl Mayday {
    pub fn new(config: MaydayConfig) -> Mayday {
        Mayday {
            mayday: "mayday".to_string(),
            config,
        }
    }
    pub fn get_mayday(&self) -> &str {
        &self.mayday
    }
}

pub struct MaydayConfig {
    db_conn: DatabaseConnection,
    settings: HashMap<String, String>,
}
impl MaydayConfig {
    pub fn new() -> MaydayConfig {
        MaydayConfig {
            db_conn: Default::default(),
            settings: Default::default(),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_get_mayday() {
//         let mayday = Mayday::new(String::from("test"));
//         assert_eq!("test", mayday.get_mayday());
//     }
// }
