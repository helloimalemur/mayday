use crate::logger;
use crate::logger::Header;
use actix_web::error::ErrorBadRequest;
use actix_web::web::{method, Data, Payload};
use actix_web::{web, HttpRequest};
use futures_util::StreamExt;
use maydaylib::appstate::AppState;
use maydaylib::{is_key_valid};
use maydaylib::session::{Session, SessionRequest, SessionRequestType};
use std::sync::Mutex;
use maydaylib::mayday::{MaydayRequest, MaydayRequestType};

#[utoipa::path(
    post,
    path = "/session",
    responses(
        (status = 201, description = "Session created successfully", body = SessionRequest,
            headers(
                ("X-API-KEY" = String, description = "api-key")
            ),
        ),
        (status = 500, description = "Session could not be created")
        // (status = 409, description = "Session with id already exists", body = ErrorResponse, example = json!(ErrorResponse::Conflict(String::from("id = 1"))))
    )
)]
pub async fn session(
    // name: web::Path<String>,
    mut payload: Payload,
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
) -> String {
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
                return ErrorBadRequest("overflow").to_string();
            }
            body.extend_from_slice(&chunk);
        }

        // println!("{:?}", body);
        let mut response = "ok\n".to_string();
        if let Ok(message) = serde_json::from_slice::<SessionRequest>(&body) {
            println!("PARSED: {:?}", message);
            match message.session_request_type {
                // curl -XPOST -H'X-API-KEY: somekey' localhost:8202/session -d '{
                // "user_id":1,
                // "name":"john",
                // "email":"pss",
                // "session_id":"pss",
                // "session_request_type":"Create"
                // }'
                SessionRequestType::Create => {
                    let app_state = data.lock().unwrap();
                    let db_conn = app_state.db_pool.clone();
                    message.create(db_conn, MaydayRequestType::Session(message.clone())).await
                }
                // curl -XPOST -H'X-API-KEY: somekey' localhost:8202/session -d '{
                // "name":"test@gmail.com",
                // "email":"john",
                // "password":"pss",
                // "session_request_type":"Read"
                // }'
                SessionRequestType::Read => {
                    let app_state = data.lock().unwrap();
                    let db_conn = app_state.db_pool.clone();
                    message.read(db_conn, MaydayRequestType::Session(message.clone())).await
                }
                // curl -XPOST -H'X-API-KEY: somekey' localhost:8202/session -d '{
                // "name":"test@gmail.com",
                // "email":"john",
                // "password":"pss",
                // "session_request_type":"Update"
                // }'
                SessionRequestType::Update => {
                    let app_state = data.lock().unwrap();
                    let db_conn = app_state.db_pool.clone();
                    message.update(db_conn, MaydayRequestType::Session(message.clone())).await
                }
                // curl -XPOST -H'X-API-KEY: somekey' localhost:8202/session -d '{
                // "name":"test@gmail.com",
                // "email":"john",
                // "password":"pss",
                // "session_request_type":"Delete"
                // }'
                SessionRequestType::Delete => {
                    let app_state = data.lock().unwrap();
                    let db_conn = app_state.db_pool.clone();
                    message.delete(db_conn, MaydayRequestType::Session(message.clone())).await
                }
            };

            response
        } else {
            let message = format!(
                "FAIL to deserialize: {} {} {}",
                req.method(),
                req.uri(),
                String::from_utf8(body.to_vec()).unwrap()
            );
            logger::log(Header::WARNING, message.as_str());
            "invalid schema\n".to_string()
        }
    } else {
        let message = format!("INVALID API {} {} {}", req.method(), req.uri(), key);
        logger::log(Header::WARNING, message.as_str());
        "invalid api key\n".to_string()
    }
}
