use actix_web::Error;
use actix_web::error::ErrorConflict;
use crate::mayday::{MaydayError, MaydayRequest, MaydayRequestType};
use crate::{session, user};
use rand::random;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ActiveValue, ColIdx, DatabaseConnection, DeleteResult, DeriveEntityModel, QueryTrait};
use chrono::Local;
use sqlx::Row;
use utoipa::{OpenApi, ToSchema};
use log::info;
use crate::mayday::MaydayError::Unauthorized;

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(
    Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema, PartialEq, DeriveEntityModel,
)]
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
    #[sea_orm(column_name = "secret")]
    pub secret: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct User {
    pub user_id: i16,
    pub name: String,
    pub email: String,
    pub secret: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema)]
pub enum UserRequestType {
    Create, // register
    Read,   // login
    Update, // edit / update
    Delete, // delete account
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct UserRequest {
    pub name: String,
    pub email: String,
    pub secret: String,
    pub new_name: Option<String>,
    pub new_email: Option<String>,
    pub new_secret: Option<String>,
    pub user_request_type: UserRequestType,
}

impl MaydayRequest for UserRequest {
    async fn process(&self, dbcon: DatabaseConnection, message: MaydayRequestType) {
        let _ = match &self.user_request_type {
            UserRequestType::Create => self.create(dbcon, message).await,
            UserRequestType::Read => self.read(dbcon, message).await,
            UserRequestType::Update => self.update(dbcon, message).await,
            UserRequestType::Delete => self.delete(dbcon, message).await,
        };
    }
    // curl -XPOST -H'X-API-KEY: somekey' localhost:8202/user -d '{
    // "name":"test1@gmail.com",
    // "email":"test1",
    // "secret":"p1",
    // "user_request_type":"Create"
    // }'
    async fn create(&self, dbcon: DatabaseConnection, message: MaydayRequestType) -> Result<String, MaydayError> {
        let db = dbcon.clone();
        let mut exists = false;
        let rand = random::<u16>();
        let mut randi = rand as i16;
        if randi < 0i16 {
            randi = randi * -1i16;
        }
        if let MaydayRequestType::User(user) = message {
            let mut user = user::ActiveModel {
                id: Default::default(),
                user_id: ActiveValue::Set(randi),
                name: ActiveValue::Set(user.name),
                email: ActiveValue::Set(user.email),
                secret: ActiveValue::Set(user.secret),
                ..Default::default()
            };

            // check if user already exists
            if let Ok(u) = user::Entity::find()
                .filter(user::Column::Email.eq(user.clone().email.unwrap().clone()))
                .one(&dbcon)
                .await
            {
                if let Some(u) = u {
                    exists = true;
                }
            }
            // create if not already exists
            let cloned = user.clone();
            if !exists {
                if let Ok(_inserted) = user.insert(&db).await {
                    Ok("Created".to_string())
                }else {
                    Err(MaydayError::Conflict)
                }
            } else {
                Err(MaydayError::Conflict)
            }
        } else {
            Err(Unauthorized)
        }
    }
    // curl -XPOST -H'X-API-KEY: somekey' localhost:8202/user -d '{
    // "name":"test1@gmail.com",
    // "email":"test1",
    // "secret":"p1",
    // "user_request_type":"Read"
    // }'
    async fn read(&self, dbcon: DatabaseConnection, message: MaydayRequestType) -> Result<String, MaydayError> {
        let db = dbcon.clone();
        if let MaydayRequestType::User(user) = message {
            if let Ok(u) = user::Entity::find()
                .filter(user::Column::Secret.eq(user.secret.clone()))
                .filter(user::Column::Email.eq(user.email.clone()))
                .one(&dbcon)
                .await
            {
                if let Some(_user) = u {
                    // println!("{:#?}", u);
                    if let Ok(check) = session::Entity::find()
                        .filter(session::Column::Email.eq(user.email.clone()))
                        .one(&db)
                        .await
                    {
                        if let Some(u) = check {
                            // info!("session exists: {:?}", u);
                            Ok(u.session_id)
                        } else {
                            // println!("no session");
                            let rand = random::<u128>();
                            let new_session = session::ActiveModel {
                                user_id: Default::default(),
                                name: ActiveValue::Set(user.name.clone()),
                                email: ActiveValue::Set(user.email.clone()),
                                session_id: ActiveValue::Set(rand.to_string()),
                                timestamp: ActiveValue::Set(Local::now().timestamp()),
                            };
                            // println!(
                            //     "new session token: {}",
                            //     new_session.clone().session_id.unwrap()
                            // );
                            info!("new session token: {:?}", new_session);
                            let new_session_cloned = new_session.clone();
                            if let Ok(res) = new_session.insert(&db).await {
                                let session_id = res.session_id.as_str();
                                Ok(session_id.parse().unwrap())
                            } else {
                                Ok(new_session_cloned.session_id.unwrap())
                            }
                        }
                    } else {
                        Err(Unauthorized)
                    }
                } else {
                    Err(Unauthorized)
                }
            } else {
                Err(Unauthorized)
            }
        } else {
            Err(Unauthorized)
        }
    }
    // curl -XPOST -H'X-API-KEY: somekey' localhost:8202/user -d '{
    // "name":"test1@gmail.com",
    // "email":"test1",
    // "secret":"p1",
    // "new_name":"test2@gmail.com",
    // "new_email":"test2",
    // "new_secret":"p2",
    // "user_request_type":"Update"
    // }'
    async fn update(&self, dbcon: DatabaseConnection, message: MaydayRequestType) -> Result<String, MaydayError> {
        let db = dbcon.clone();
        let rand = random::<u16>();
        // Locate user
        if let MaydayRequestType::User(user) = message {
            if let Ok(fouds) = user::Entity::find()
                .filter(user::Column::Secret.eq(user.secret))
                .filter(user::Column::Email.eq(user.email))
                .one(&dbcon)
                .await
            {
                if let Some(mut u) = fouds {
                    // Update user
                    let mut active_user: user::ActiveModel = u.into();
                    active_user.name = ActiveValue::Set(user.new_name.unwrap());
                    active_user.email = ActiveValue::Set(user.new_email.unwrap());
                    active_user.secret = ActiveValue::Set(user.new_secret.unwrap());
                    if let Ok(res) = active_user.update(&db).await {
                        Ok("User Updated".to_string())
                    } else {
                        Err(MaydayError::NotFound)
                    }
                    // println!("{:?}", res);
                } else {
                    Err(Unauthorized)
                }
            } else {
                Err(Unauthorized)
            }
        } else {
            Err(Unauthorized)
        }
    }
    // curl -XPOST -H'X-API-KEY: somekey' localhost:8202/user -d '{
    // "name":"test2@gmail.com",
    // "email":"test2",
    // "secret":"p2",
    // "user_request_type":"Delete"
    // }'
    async fn delete(&self, dbcon: DatabaseConnection, message: MaydayRequestType) -> Result<String, MaydayError> {
        let db = dbcon.clone();
        let rand = random::<u16>();
        // Locate user
        if let MaydayRequestType::User(user) = message {
            if let Ok(fouds) = user::Entity::find()
                .filter(user::Column::Secret.eq(user.secret.clone()))
                .filter(user::Column::Email.eq(user.email.clone()))
                .one(&dbcon)
                .await
            {
                if let Some(mut u) = fouds {
                    // Delete user
                    let mut active_user: user::ActiveModel = u.into();
                    let res: DeleteResult = active_user.delete(&dbcon).await.unwrap();
                    println!("Deleted user: {:?}", res);
                    // Delete session
                    if let Ok(session) = session::Entity::find()
                        .filter(session::Column::Email.eq(user.email))
                        .one(&dbcon)
                        .await
                    {
                        if let Some(s) = session {
                            let mut active_session: session::ActiveModel = s.into();
                            let res = active_session.delete(&dbcon).await.unwrap();
                            info!("Deleted session: {:?}", res);
                            Ok("Session Deleted".to_string())
                        } else {
                            Err(MaydayError::NotFound)
                        }
                    } else {
                        Err(MaydayError::NotFound)
                    }
                } else {
                    Err(Unauthorized)
                }
            } else {
                Err(Unauthorized)
            }
        } else {
            Err(Unauthorized)
        }
    }
}
