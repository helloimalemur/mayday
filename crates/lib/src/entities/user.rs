use crate::{user};
use rand::{random};
use sea_orm::{sqlx, ActiveModelBehavior, ActiveModelTrait, ActiveValue, DatabaseConnection, DeriveEntityModel};
use sqlx::{Row};
use utoipa::{OpenApi, ToSchema};
use crate::mayday::{MaydayRequest, MaydayRequestType};
use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[sea_orm(column_name = "id")]
    pub id: i16,
    #[sea_orm(column_name = "user_id")]
    pub user_id: i16,
    #[sea_orm(column_name = "name")]
    pub name: String,
    #[sea_orm(column_name = "email")]
    pub email: String,
    #[sea_orm(column_name = "password")]
    pub password: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct User {
    pub user_id: i16,
    pub name: String,
    pub email: String,
    pub password: String,
}


#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema)]
pub enum UserRequestType {
    Create,
    Read,
    Update,
    Delete
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct UserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub user_request_type: UserRequestType
}

impl MaydayRequest for UserRequest {
    async fn process(&self, dbcon: DatabaseConnection, message: MaydayRequestType) {
        match &self.user_request_type {
            UserRequestType::Create => { self.create(dbcon, message).await }
            UserRequestType::Read => { self.read(dbcon, message).await }
            UserRequestType::Update => { self.update(dbcon, message).await }
            UserRequestType::Delete => { self.delete(dbcon, message).await }
        };
    }
    // curl -XPOST -H'X-API-KEY: somekey' localhost:8202/user -d '{
    // "name":"test@gmail.com",
    // "email":"john",
    // "password":"pass",
    // "user_request_type":"Create"
    // }'
    async fn create(&self, dbcon: DatabaseConnection, message: MaydayRequestType) {
        let db = dbcon.clone();
        let rand = random::<i16>();
        if let MaydayRequestType::User(user) = message {
            let mut user = user::ActiveModel {
                id: Default::default(),
                user_id: ActiveValue::Set(rand),
                name: ActiveValue::Set(user.name),
                email: ActiveValue::Set(user.email),
                password: ActiveValue::Set(user.password),
            };
            let inserted = user.insert(&db).await;
            println!("{:?}", inserted);
        }
    }
    async fn read(&self, dbcon: DatabaseConnection, message: MaydayRequestType) {
        // let db = dbcon.clone();
        // let rand = random::<i16>();
        // if let MaydayRequestType::User(user) = message {
        //     let mut user = user::ActiveModel {
        //         id: Default::default(),
        //         user_id: ActiveValue::Set(rand),
        //         name: ActiveValue::Set(user.name),
        //         email: ActiveValue::Set(user.email),
        //         password: ActiveValue::Set(user.password),
        //     };
        //     let inserted = user.insert(&db).await;
        //     println!("{:?}", inserted);
        // }
    }
    async fn update(&self, dbcon: DatabaseConnection, message: MaydayRequestType) {
        // let db = dbcon.clone();
        // let rand = random::<i16>();
        // if let MaydayRequestType::User(user) = message {
        //     let mut user = user::ActiveModel {
        //         id: Default::default(),
        //         user_id: ActiveValue::Set(rand),
        //         name: ActiveValue::Set(user.name),
        //         email: ActiveValue::Set(user.email),
        //         password: ActiveValue::Set(user.password),
        //     };
        //     let inserted = user.insert(&db).await;
        //     println!("{:?}", "inserted");
        // }
    }
    async fn delete(&self, dbcon: DatabaseConnection, message: MaydayRequestType) {
        // let db = dbcon.clone();
        // let rand = random::<i16>();
        // if let MaydayRequestType::User(user) = message {
        //     let mut user = user::ActiveModel {
        //         id: Default::default(),
        //         user_id: ActiveValue::Set(rand),
        //         name: ActiveValue::Set(user.name),
        //         email: ActiveValue::Set(user.email),
        //         password: ActiveValue::Set(user.password),
        //     };
        //     let inserted = user.insert(&db).await;
        //     println!("{:?}", "inserted");
        // }
    }
}

