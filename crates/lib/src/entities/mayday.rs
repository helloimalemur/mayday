use sea_orm::{ActiveModelTrait, DatabaseConnection, DeriveRelation, EnumIter};
use std::collections::HashMap;
use std::sync::MutexGuard;
use crate::register::RegisterRequest;
use crate::session::SessionRequest;
use crate::user::{User, UserRequest};

pub enum MaydayRequestType {
    User(UserRequest),
    Session(SessionRequest),
    Register(RegisterRequest)
}

pub trait MaydayRequest {
    async fn process(&self, dbcon: DatabaseConnection, message: MaydayRequestType);
    async fn create(&self, dbcon: DatabaseConnection, message: MaydayRequestType) {
        println!("Mayday request");
    }
    async fn read(&self, dbcon: DatabaseConnection, message: MaydayRequestType) {
        println!("Mayday request");
    }
    async fn update(&self, dbcon: DatabaseConnection, message: MaydayRequestType) {
        println!("Mayday request");
    }
    async fn delete(&self, dbcon: DatabaseConnection, message: MaydayRequestType) {
        println!("Mayday request");
    }
}

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
