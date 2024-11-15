use crate::logger;
use crate::logger::Header;
use actix_web::error::ErrorBadRequest;
use actix_web::web::{method, Data, Payload};
use actix_web::{web, HttpRequest};
use futures_util::StreamExt;
use maydaylib::appstate::AppState;
use maydaylib::is_key_valid;
use maydaylib::mayday::{MaydayError, MaydayRequest, MaydayRequestType};
use maydaylib::user::{User, UserRequest, UserRequestType};
use std::sync::Mutex;

#[utoipa::path(
    post,
    path = "/user",
    responses(
        (status = 201, description = "User created successfully", body = UserRequest,
            headers(
                ("X-API-KEY" = String, description = "api-key")
            ),
        ),
        (status = 500, description = "User could not be created")
        // (status = 409, description = "User with id already exists", body = ErrorResponse, example = json!(ErrorResponse::Conflict(String::from("id = 1"))))
    )
)]
pub async fn user(
    // name: web::Path<String>,
    mut payload: Payload,
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
) -> Result<String, MaydayError> {
    const MAX_SIZE: usize = 262_144; // max payload size is 256k
    let mut auth = false;
    let mut key = String::new();
    // verify api_key
    if let Some(k) = req.headers().get("X-API-KEY") {
        key = k.to_str().unwrap().to_string();
        if is_key_valid(
            req.headers()
                .get("X-API-KEY")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            data.lock().unwrap().api_keys.lock().unwrap().to_vec(),
        ) {
            auth = true;
        }
    }

    if auth {
        let mut body = web::BytesMut::new();

        while let Some(chunk) = payload.next().await {
            let chunk = chunk.unwrap();
            // limit max size of in-memory payload
            if (body.len() + chunk.len()) > MAX_SIZE {
                return Err(MaydayError::BadRequest)
            }
            body.extend_from_slice(&chunk);
        }

        // println!("{:?}", body);
        let mut response = "ok\n".to_string();
        if let Ok(message) = serde_json::from_slice::<UserRequest>(&body) {
            println!("PARSED: {:?}", message);
            match message.user_request_type {
                // curl -XPOST -H'X-API-KEY: somekey' localhost:8202/user -d '{
                // "name":"test@gmail.com",
                // "email":"john",
                // "secret":"pss",
                // "user_request_type":"Create"
                // }'
                UserRequestType::Create => {
                    let app_state = data.lock().unwrap();
                    let db_conn = app_state.db_pool.clone();
                    message
                        .create(db_conn, MaydayRequestType::User(message.clone()))
                        .await?
                }
                // curl -XPOST -H'X-API-KEY: somekey' localhost:8202/user -d '{
                // "name":"test@gmail.com",
                // "email":"john",
                // "secret":"pss",
                // "user_request_type":"Read"
                // }'
                UserRequestType::Read => {
                    let app_state = data.lock().unwrap();
                    let db_conn = app_state.db_pool.clone();
                    message
                        .read(db_conn, MaydayRequestType::User(message.clone()))
                        .await?
                }
                // curl -XPOST -H'X-API-KEY: somekey' localhost:8202/user -d '{
                // "name":"test@gmail.com",
                // "email":"john",
                // "secret":"pss",
                // "user_request_type":"Update"
                // }'
                UserRequestType::Update => {
                    let app_state = data.lock().unwrap();
                    let db_conn = app_state.db_pool.clone();
                    message
                        .update(db_conn, MaydayRequestType::User(message.clone()))
                        .await?
                }
                // curl -XPOST -H'X-API-KEY: somekey' localhost:8202/user -d '{
                // "name":"test@gmail.com",
                // "email":"john",
                // "secret":"pss",
                // "user_request_type":"Delete"
                // }'
                UserRequestType::Delete => {
                    let app_state = data.lock().unwrap();
                    let db_conn = app_state.db_pool.clone();
                    message
                        .delete(db_conn, MaydayRequestType::User(message.clone()))
                        .await?
                }
            };

            Ok(response)
        } else {
            let message = format!(
                "FAIL to deserialize: {} {} {}",
                req.method(),
                req.uri(),
                String::from_utf8(body.to_vec()).unwrap()
            );
            logger::log(Header::WARNING, message.as_str());
            Err(MaydayError::InvalidSchema)
        }
    } else {
        let message = format!("INVALID API {} {} {}", req.method(), req.uri(), key);
        logger::log(Header::WARNING, message.as_str());
        Err(MaydayError::Unauthorized)
    }
}
