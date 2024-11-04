// // CREATE TABLE `register` (
// // `userid` INT NOT NULL,
// // `name` VARCHAR(255) NOT NULL,
// // `email` VARCHAR(255) NOT NULL,
// // `registerid` VARCHAR(255) NOT NULL,
// // PRIMARY KEY (`registerid`)
// // ) ENGINE=InnoDB;

use crate::appstate::AppState;
use crate::entities::user::{User};
use crate::{is_key_valid, register, user};
use actix_web::error::ErrorBadRequest;
use actix_web::web::Data;
use actix_web::{web, HttpRequest};
use futures_util::StreamExt;
use rand::Rng;
use sqlx::mysql::MySqlRow;
use sqlx::{MySql, Pool, Row};
use std::sync::Mutex;
use rand::{random};
use sea_orm::{sqlx, ActiveModelBehavior, ActiveModelTrait, ActiveValue, DatabaseConnection, DeriveEntityModel};
use utoipa::{OpenApi, ToSchema};
use crate::mayday::{MaydayRequest, MaydayRequestType};
use sea_orm::entity::prelude::*;
use crate::user::{UserRequest, UserRequestType};

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "register")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[sea_orm(column_name = "id")]
    user_id: u16,
    #[sea_orm(column_name = "name")]
    name: String,
    #[sea_orm(column_name = "email")]
    email: String,
    #[sea_orm(column_name = "register_id")]
    register_id: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema)]
pub enum RegisterRequestType {
    Create,
    Read,
    Update,
    Delete
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub user_id: u16,
    pub name: String,
    pub email: String,
    pub register_id: String,
    pub register_request_type: RegisterRequestType,
}


#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct Register {
    user_id: u16,
    name: String,
    email: String,
    register_id: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct RegisterId {
    register_id: String,
}

impl RegisterId {
    pub fn new(string: String) -> RegisterId {
        RegisterId { register_id: string }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct LogoutRequest {
    register_id: String,
}

// CREATE TABLE `register` (
// `userid` INT NOT NULL,
// `name` VARCHAR(255) NOT NULL,
// `email` VARCHAR(255) NOT NULL,
// `registerid` VARCHAR(255) NOT NULL,
// PRIMARY KEY (`registerid`)
// ) ENGINE=InnoDB;

impl MaydayRequest for RegisterRequest {
    async fn process(&self, dbcon: DatabaseConnection, message: MaydayRequestType) {
        match &self.register_request_type {
            RegisterRequestType::Create => { self.create(dbcon, message).await }
            RegisterRequestType::Read => { self.read(dbcon, message).await }
            RegisterRequestType::Update => { self.update(dbcon, message).await }
            RegisterRequestType::Delete => { self.delete(dbcon, message).await }
        };
    }
    // curl -XPOST -H'X-API-KEY: somekey' localhost:8202/register -d '{
    // "name":"test@gmail.com",
    // "email":"john",
    // "password":"pass",
    // "register_request_type":"Create"
    // }'
    async fn create(&self, dbcon: DatabaseConnection, message: MaydayRequestType) {
        let db = dbcon.clone();
        let rand = random::<u16>();
        if let MaydayRequestType::Register(register) = message {
            let mut register = register::ActiveModel {
                user_id: ActiveValue::Set(register.user_id),
                name: ActiveValue::Set(register.name),
                email: ActiveValue::Set(register.email),
                register_id: ActiveValue::Set(register.register_id),
            };
            let inserted = register.insert(&db).await;
            println!("{:?}", inserted);
        }
    }
    async fn read(&self, dbcon: DatabaseConnection, message: MaydayRequestType) {
        // let db = dbcon.clone();
        // let rand = random::<u16>();
        // let mut register = register::ActiveModel {
        //     id: Default::default(),
        //     register_id: ActiveValue::Set(rand),
        //     name: ActiveValue::Set(message.name),
        //     email: ActiveValue::Set(message.email),
        //     password: ActiveValue::Set(message.password),
        // };
        // let inserted = register(&db).await;
        // println!("{:?}", inserted);
    }
    async fn update(&self, dbcon: DatabaseConnection, message: MaydayRequestType) {
        // let db = dbcon.clone();
        // let rand = random::<u16>();
        // let mut register = register::ActiveModel {
        //     id: Default::default(),
        //     register_id: ActiveValue::Set(rand),
        //     name: ActiveValue::Set(message.name),
        //     email: ActiveValue::Set(message.email),
        //     password: ActiveValue::Set(message.password),
        // };
        // let inserted = register.insert(&db).await;
        // println!("{:?}", inserted);
    }
    async fn delete(&self, dbcon: DatabaseConnection, message: MaydayRequestType) {
        // let db = dbcon.clone();
        // let rand = random::<u16>();
        // let mut register = register::ActiveModel {
        //     id: Default::default(),
        //     register_id: ActiveValue::Set(rand),
        //     name: ActiveValue::Set(message.name),
        //     email: ActiveValue::Set(message.email),
        //     password: ActiveValue::Set(message.password),
        // };
        // let inserted = register.insert(&db).await;
        // println!("{:?}", inserted);
    }
}

