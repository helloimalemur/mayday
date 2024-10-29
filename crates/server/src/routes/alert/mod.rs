use std::sync::Mutex;
use actix_web::error::ErrorBadRequest;
use actix_web::{web, HttpRequest};
use actix_web::web::{Data, Payload};
use futures_util::StreamExt;
use serde_json::Value;
use maydaylib::appstate::AppState;
use maydaylib::is_key_valid;
use maydaylib::user::User;
use crate::logger;
use crate::logger::Header;

pub async fn alert(
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
            data.lock().unwrap().api_key.lock().unwrap().to_vec(),
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
        if let Ok(message) = serde_json::from_slice::<Value>(&body) {
            response
        } else {
            let message = format!("FAIL to deserialize: {} {} {}", req.method(), req.uri(), String::from_utf8(body.to_vec()).unwrap());
            logger::log(Header::WARNING, message.as_str());
            "invalid schema\n".to_string()
        }
    } else {
        let message = format!("INVALID API {} {} {}", req.method(), req.uri(), key);
        logger::log(Header::WARNING, message.as_str());
        "invalid api key\n".to_string()
    }
}