use crate::register::RegisterRequest;
use crate::session::SessionRequest;
use crate::user::{ActiveModel, User, UserRequest};
use sea_orm::{ActiveModelTrait, ColIdx, DatabaseConnection, DeriveRelation, EnumIter};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::sync::MutexGuard;
use actix_web::http::{Error, StatusCode};
use actix_web::ResponseError;
use crate::mayday::MaydayError::Conflict;

pub enum MaydayRequestType {
    User(UserRequest),
    Session(SessionRequest),
    Register(RegisterRequest),
}

#[derive(Debug)]
pub enum MaydayError {
    Unimplemented,
    InvalidSchema,
    Conflict,
    Unauthorized,
    NotFound,
    BadRequest,
}
impl Display for MaydayError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for MaydayError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub trait MaydayRequest {
    async fn process(&self, dbcon: DatabaseConnection, message: MaydayRequestType);
    async fn create(&self, dbcon: DatabaseConnection, message: MaydayRequestType) -> Result<String, MaydayError> {
        Ok("Unimplemented".parse().unwrap())
    }
    async fn read(&self, dbcon: DatabaseConnection, message: MaydayRequestType) -> Result<String, MaydayError> {
        Ok("Unimplemented".parse().unwrap())
    }
    async fn update(&self, dbcon: DatabaseConnection, message: MaydayRequestType) -> Result<String, MaydayError> {
        Ok("Unimplemented".parse().unwrap())
    }
    async fn delete(&self, dbcon: DatabaseConnection, message: MaydayRequestType) -> Result<String, MaydayError> {
        Ok("Unimplemented".parse().unwrap())
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
