use crate::appstate::AppState;
use crate::{is_key_valid, user};
use actix_web::error::ErrorBadRequest;
use actix_web::web::{Data, Payload};
use actix_web::{web, HttpRequest};
use base64::Engine;
use futures_util::StreamExt;
use magic_crypt::generic_array::typenum::U256;
use magic_crypt::MagicCryptTrait;
use rand::{random, thread_rng, Rng, RngCore};
use sea_orm::{sqlx, ActiveModelBehavior, ActiveModelTrait, ActiveValue, ConnectionTrait, DatabaseConnection, DeriveEntityModel, TransactionTrait};
use sqlx::{MySql, Pool, Row};
use std::io::Cursor;
use std::ops::Deref;
use std::sync::{Mutex, MutexGuard};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use crate::mayday::MaydayRequest;
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
    async fn process(&self, dbcon: DatabaseConnection, message: UserRequest) {
        todo!()
    }

    // async fn process(&self, dbcon: DatabaseConnection, message: UserRequest) {
    //     match &self.user_request_type {
    //         UserRequestType::Create => { self.create(dbcon, message) }
    //         UserRequestType::Read => { self.read(dbcon, message) }
    //         UserRequestType::Update => { self.update(dbcon, message) }
    //         UserRequestType::Delete => { self.delete(dbcon, message) }
    //     };
    // }
    async fn create(&self, dbcon: DatabaseConnection, message: UserRequest) {
        let db = dbcon.clone();
        let rand = random::<i16>();
        let mut user = user::ActiveModel {
            id: Default::default(),
            user_id: ActiveValue::Set(rand),
            name: ActiveValue::Set(message.name),
            email: ActiveValue::Set(message.email),
            password: ActiveValue::Set(message.password),
        };
        let inserted = user.insert(&db).await;
        println!("{:?}", inserted);
    }
    async fn read(&self, dbcon: DatabaseConnection, message: UserRequest) {
        // todo!()
    }
    async fn update(&self, dbcon: DatabaseConnection, message: UserRequest) {
        // todo!()
    }
    async fn delete(&self, dbcon: DatabaseConnection, message: UserRequest) {
        // todo!()
    }
}

// CREATE TABLE `user` (
// `userid` INT NOT NULL,
// `name` VARCHAR(255) NOT NULL,
// `email` VARCHAR(255) NOT NULL,
// `password` VARCHAR(255) NOT NULL,
// PRIMARY KEY (`userid`)
// ) ENGINE=InnoDB;

// curl -XPOST -H'X-API-KEY: somekey' localhost:8202/user -d '{
// "name":"test@gmail.com",
// "email":"john",
// "password":"pss",
// "user_request_type":"Create"
// }'
pub async fn create_user_route(
    // name: web::Path<String>,
    mut payload: web::Payload,
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
) -> String {
    const MAX_SIZE: usize = 262_144; // max payload size is 256k
                                     // verify api_key
    if req.headers().get("X-API-KEY").is_some() {
        if is_key_valid(
            req.headers()
                .get("X-API-KEY")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            data.lock().unwrap().api_keys.lock().unwrap().to_vec(),
        ) {
            let mut body = web::BytesMut::new();

            while let Some(chunk) = payload.next().await {
                let chunk = chunk.unwrap();
                // limit max size of in-memory payload
                if (body.len() + chunk.len()) > MAX_SIZE {
                    return ErrorBadRequest("overflow").to_string();
                }
                body.extend_from_slice(&chunk);
            }

            let apps_state = data.lock().unwrap();
            let settings = apps_state.settings.lock().unwrap();
            let hash_key = settings.get("hash_key").unwrap().clone();
            drop(settings);
            drop(apps_state);

            // println!("{:?}", body);
            // body is loaded, now we can deserialize serde-json
            if let Ok(obj) = serde_json::from_slice::<UserRequest>(&body) {
                // println!("{:?}", obj);
                let mut rand = rand::thread_rng();
                let new_user_id: u16 = rand.gen();
                let user_req = obj.clone();
                let new_user_id_i = new_user_id as i16;
                let password_hash = create_password_hash(obj.password, hash_key.to_string());
                let new_user = User {
                    user_id: new_user_id_i,
                    name: obj.name,
                    email: obj.email,
                    password: password_hash,
                };

                // println!("{:#?}", new_user.clone());
                let user_exists = check_user_exist(user_req.email, data.clone()).await;
                if !user_exists {
                    let _ = create_user(new_user.clone(), data.clone()).await;
                    "user created\n".to_string()
                } else if user_exists {
                    "user exists\n".to_string()
                } else {
                    "error creating user\n".to_string()
                }
            } else {
                "error creating user\n".to_string()
            }
        } else {
            "invalid api key\n".to_string()
        }
    } else {
        "invalid api key\n".to_string()
    }
}

pub async fn check_user_exist(user_email: String, data: Data<Mutex<AppState>>) -> bool {
    let mut user_exists: bool = false;
    let mut app_state = data.lock();
    let db_pool = app_state.as_mut().unwrap().db_pool.clone();

    let query_result = sqlx::query("SELECT email FROM user WHERE email LIKE (?)")
        .bind(user_email.clone())
        .fetch_one(db_pool.get_mysql_connection_pool())
        .await;
    if query_result.is_ok() {
        let row1 = query_result.unwrap();
        let email: String = row1.get("email");
        if email.eq_ignore_ascii_case(user_email.clone().as_str()) {
            user_exists = true;
        }
    } else {
        user_exists = false;
    }
    // println!("user exists: {}", user_exists);
    user_exists
}

pub async fn check_user_exist_with_password_hash(
    user_email: String,
    user_password: String,
    data: Data<Mutex<AppState>>,
) -> bool {
    let mut app_state = data.lock();
    let db_pool = app_state.as_mut().unwrap().db_pool.clone();

    // println!("{}", user_email);
    // println!("{}", user_password);

    let query_result =
        sqlx::query("SELECT email,password FROM user WHERE email = ? AND password = ?")
            .bind(user_email.clone())
            .bind(user_password.clone())
            .fetch_one(db_pool.get_mysql_connection_pool())
            .await;

    // println!("{:#?}", query_result);

    if query_result.is_ok() {
        let result = query_result.unwrap();

        let _email: String = result.get(0);
        let _password: String = result.get(1);

        // println!("{}", email);
        // println!("{}", password);

        if user_password.eq_ignore_ascii_case(result.get(1)) {
            println!("success: {}", true);
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
    return false;
}

pub fn create_password_hash(password: String, hash_key: String) -> String {
    let mc = magic_crypt::new_magic_crypt!(hash_key, 256);

    let mut reader = Cursor::new(password);
    let mut writer = Vec::new();
    mc.encrypt_reader_to_writer2::<U256>(&mut reader, &mut writer)
        .unwrap();
    let encrypted = base64::engine::general_purpose::STANDARD.encode(&writer);

    encrypted
}

pub async fn create_user(user: User, data: Data<Mutex<AppState>>) {
    let mut app_state = data.lock();
    let db_pool = app_state.as_mut().unwrap().db_pool.clone();

    println!("Create user: {}", user.email);

    let _query_result =
        sqlx::query("INSERT INTO user (userid, name, email, password) VALUES (?,?,?,?)")
            .bind(user.user_id as u16)
            .bind(user.name)
            .bind(user.email)
            .bind(user.password)
            .execute(db_pool.get_mysql_connection_pool())
            .await
            .unwrap();

    // println!("{:#?}", query_result);
}

// curl -XPOST -H'X-API-KEY: 12790066417744034365' localhost:8202/api/delete/user/ -d '{"name":"johnny","email":"johhny@mail.com"}'
pub async fn delete_user_route(
    // name: web::Path<String>,
    mut payload: Payload,
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
) -> String {
    const MAX_SIZE: usize = 262_144; // max payload size is 256k
                                     // verify api_key
    if req.headers().get("X-API-KEY").is_some() {
        if is_key_valid(
            req.headers()
                .get("X-API-KEY")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            data.lock().unwrap().api_keys.lock().unwrap().to_vec(),
        ) {
            let mut body = web::BytesMut::new();
            while let Some(chunk) = payload.next().await {
                let chunk = chunk.unwrap();
                if (body.len() + chunk.len()) > MAX_SIZE {
                    return "request too large".to_string();
                }
                body.extend_from_slice(&chunk);
            }

            if let Ok(user) = serde_json::from_slice::<UserRequest>(&body) {
                if check_user_exist(user.clone().email, data.clone()).await {
                    if delete_user(user, data).await {
                        "user deleted".to_string()
                    } else {
                        "error deleting user".to_string()
                    }
                } else {
                    "user does not exist".to_string()
                }
            } else {
                "error deleting user".to_string()
            }
        } else {
            "invalid api key\n".to_string()
        }
    } else {
        "invalid api key\n".to_string()
    }
}

pub async fn delete_user(user: UserRequest, data: Data<Mutex<AppState>>) -> bool {
    let mut app_state = data.lock();
    let db_pool = app_state.as_mut().unwrap().db_pool.clone();

    if let Ok(_query_result) = sqlx::query("DELETE FROM user WHERE email LIKE (?)")
        .bind(user.email)
        .execute(db_pool.get_mysql_connection_pool())
        .await
    {
        true
    } else {
        false
    }
}

#[allow(unused)]
pub async fn modify_user_route(
    // name: web::Path<String>,
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
) -> String {
    // verify api_key
    if req.headers().get("X-API-KEY").is_some() {
        if is_key_valid(
            req.headers()
                .get("X-API-KEY")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            data.lock().unwrap().api_keys.lock().unwrap().to_vec(),
        ) {
            "ok\n".to_string()
        } else {
            "invalid api key\n".to_string()
        }
    } else {
        "invalid api key\n".to_string()
    }
}

#[allow(unused)]
pub fn _modify_user(user: User, db_pool: Pool<MySql>) {}

#[cfg(test)]
mod tests {
    use crate::entities::user::create_password_hash;
    use crate::frontend::start_front_end;

    #[test]
    fn test_create_password_hash() {
        // println!(
        //     "{}",
        //     create_password_hash("password".to_string(), "spiffy".to_string())
        // );
    }
}
