// // CREATE TABLE `session` (
// // `userid` INT NOT NULL,
// // `name` VARCHAR(255) NOT NULL,
// // `email` VARCHAR(255) NOT NULL,
// // `sessionid` VARCHAR(255) NOT NULL,
// // PRIMARY KEY (`sessionid`)
// // ) ENGINE=InnoDB;

use crate::mayday::{MaydayError, MaydayRequest, MaydayRequestType};
use crate::session;
use rand::random;
use sea_orm::entity::prelude::*;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ActiveValue, DatabaseConnection, DeriveEntityModel,
};
use chrono;
use log::info;
use sqlx::Row;
use utoipa::{OpenApi, ToSchema};

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(
    Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema, PartialEq, DeriveEntityModel,
)]
#[sea_orm(table_name = "session")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[sea_orm(column_name = "id")]
    pub user_id: i16,
    #[sea_orm(column_name = "name")]
    pub name: String,
    #[sea_orm(column_name = "email")]
    pub email: String,
    #[sea_orm(column_name = "timestamp")]
    pub timestamp: i64,
    #[sea_orm(column_name = "session_id")]
    pub session_id: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema)]
pub enum SessionRequestType {
    Create,
    Read,
    Update,
    Delete,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct SessionRequest {
    pub user_id: i16,
    pub name: String,
    pub email: String,
    pub session_id: String,
    pub timestamp: i64,
    pub session_request_type: SessionRequestType,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct Session {
    pub user_id: u16,
    pub name: String,
    pub email: String,
    pub timestamp: i64,
    pub session_id: String,
}
//
// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema)]
// pub struct SessionId {
//     pub session_id: String,
// }
//
// impl SessionId {
//     pub fn new(string: String) -> SessionId {
//         SessionId { session_id: string }
//     }
// }

// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema)]
// pub struct LoginRequest {
//     pub email: String,
//     pub password: String,
// }
//
// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema)]
// pub struct LogoutRequest {
//     pub session_id: String,
// }

// CREATE TABLE `session` (
// `userid` INT NOT NULL,
// `name` VARCHAR(255) NOT NULL,
// `email` VARCHAR(255) NOT NULL,
// `sessionid` VARCHAR(255) NOT NULL,
// PRIMARY KEY (`sessionid`)
// ) ENGINE=InnoDB;

impl MaydayRequest for SessionRequest {
    async fn process(&self, dbcon: DatabaseConnection, message: MaydayRequestType) {
        match &self.session_request_type {
            SessionRequestType::Create => self.create(dbcon, message).await,
            SessionRequestType::Read => self.read(dbcon, message).await,
            SessionRequestType::Update => self.update(dbcon, message).await,
            SessionRequestType::Delete => self.delete(dbcon, message).await,
        };
    }
    // curl -XPOST -H'X-API-KEY: somekey' localhost:8202/session -d '{
    // "name":"test@gmail.com",
    // "email":"john",
    // "password":"pass",
    // "session_request_type":"Create"
    // }'
    async fn create(&self, dbcon: DatabaseConnection, message: MaydayRequestType) -> Result<String, MaydayError> {
        let db = dbcon.clone();
        let rand = random::<u16>();
        if let MaydayRequestType::Session(session) = message {
            let mut session = session::ActiveModel {
                user_id: ActiveValue::Set(session.user_id),
                name: ActiveValue::Set(session.name),
                email: ActiveValue::Set(session.email),
                timestamp: ActiveValue::Set(chrono::Local::now().timestamp()),
                session_id: ActiveValue::Set(session.session_id),
            };
            let inserted = session.insert(&db).await;
            info!("{:?}", inserted);
            Ok("Session created".to_string())
        } else { 
            Err(MaydayError::Conflict)
        }
    }
    async fn read(&self, dbcon: DatabaseConnection, message: MaydayRequestType) -> Result<String, MaydayError> {
        // let db = dbcon.clone();
        // let rand = random::<u16>();
        // let mut session = session::ActiveModel {
        //     id: Default::default(),
        //     session_id: ActiveValue::Set(rand),
        //     name: ActiveValue::Set(message.name),
        //     email: ActiveValue::Set(message.email),
        //     password: ActiveValue::Set(message.password),
        // };
        // let inserted = session(&db).await;
        // println!("{:?}", inserted);
        Ok("dev".to_string())
    }
    async fn update(&self, dbcon: DatabaseConnection, message: MaydayRequestType) -> Result<String, MaydayError> {
        // let db = dbcon.clone();
        // let rand = random::<u16>();
        // let mut session = session::ActiveModel {
        //     id: Default::default(),
        //     session_id: ActiveValue::Set(rand),
        //     name: ActiveValue::Set(message.name),
        //     email: ActiveValue::Set(message.email),
        //     password: ActiveValue::Set(message.password),
        // };
        // let inserted = session.insert(&db).await;
        // println!("{:?}", inserted);
        Ok("dev".to_string())
    }
    async fn delete(&self, dbcon: DatabaseConnection, message: MaydayRequestType) -> Result<String, MaydayError> {
        // let db = dbcon.clone();
        // let rand = random::<u16>();
        // let mut session = session::ActiveModel {
        //     id: Default::default(),
        //     session_id: ActiveValue::Set(rand),
        //     name: ActiveValue::Set(message.name),
        //     email: ActiveValue::Set(message.email),
        //     password: ActiveValue::Set(message.password),
        // };
        // let inserted = session.insert(&db).await;
        // println!("{:?}", inserted);
        Ok("dev".to_string())
    }
}
