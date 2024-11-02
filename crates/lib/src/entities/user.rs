use crate::{user};
use rand::{random};
use sea_orm::{sqlx, ActiveModelBehavior, ActiveModelTrait, ActiveValue, DatabaseConnection, DeriveEntityModel, QueryTrait};
use sea_orm::ActiveValue::Set;
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
    // #[sea_orm(column_name = "new_name")]
    // pub new_name: Option<String>,
    // #[sea_orm(column_name = "new_email")]
    // pub new_email: Option<String>,
    // #[sea_orm(column_name = "new_password")]
    // pub new_password: Option<String>,

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

// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema)]
// pub struct UserRequest {
//     pub name: String,
//     pub email: String,
//     pub password: String,
//     pub user_request_type: UserRequestType
// }

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct UserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub new_name: Option<String>,
    pub new_email: Option<String>,
    pub new_password: Option<String>,
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
                ..Default::default()
            };
            let inserted = user.insert(&db).await;
            println!("{:?}", inserted);
        }
    }
    // curl -XPOST -H'X-API-KEY: somekey' localhost:8202/user -d '{
    // "name":"test@gmail.com",
    // "email":"john",
    // "password":"pass",
    // "user_request_type":"Read"
    // }'
    async fn read(&self, dbcon: DatabaseConnection, message: MaydayRequestType) {
        let db = dbcon.clone();
        let rand = random::<i16>();
        if let MaydayRequestType::User(user) = message {
            // Find a cake model first
            if let Ok(u) = user::Entity::find()
                .filter(user::Column::Password.eq(user.password))
                .filter(user::Column::Email.eq(user.email))
                .one(&dbcon)
                .await {
                if let Some(u) = u {
                    println!("{:#?}", u);
                }
            }
        }
    }
    // curl -XPOST -H'X-API-KEY: somekey' localhost:8202/user -d '{
    // "name":"test1@gmail.com",
    // "email":"tset1",
    // "password":"p1",
    // "new_name":"test2@gmail.com",
    // "new_email":"test2",
    // "new_password":"p2",
    // "user_request_type":"Update"
    // }'
    async fn update(&self, dbcon: DatabaseConnection, message: MaydayRequestType) {
        let db = dbcon.clone();
        let rand = random::<i16>();
        if let MaydayRequestType::User(user) = message {
            if let Ok(fouds) = user::Entity::find()
                .filter(user::Column::Email.eq(user.email.clone()))
                .filter(user::Column::Password.eq(user.password.clone()))
                .one(&dbcon)
                .await {
                if let Some(mut u) = fouds {

                    // Into ActiveModel
                    let mut active_user: user::ActiveModel = u.into();

                    active_user.name = ActiveValue::Set(user.new_name.unwrap());
                    active_user.email = ActiveValue::Set(user.new_email.unwrap());
                    active_user.password = ActiveValue::Set(user.new_password.unwrap());

                    let pear: user::Model = active_user.update(&dbcon).await.unwrap();

                    println!("{:?}", "updated");
                }
            }
        }
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

