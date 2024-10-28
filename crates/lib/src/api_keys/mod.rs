use actix_web::web::{BytesMut, Data};
use actix_web::{web, HttpRequest};
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, Write};
use std::sync::Mutex;
use futures_util::StreamExt;
use rand::Rng;
use crate::appstate::AppState;
use crate::{load_keys_from_file};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiKeyRequest {
    api_key: String,
}

pub fn is_key_valid(key_to_test: String, keys: Vec<String>) -> bool {
    keys.contains(&key_to_test)
}

// curl -XPOST -H'X-API-KEY: 12790066417744034365' localhost:8202/api/create/
pub async fn create_api_key(
    // name: web::Path<String>,
    mut payload: web::Payload,
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
) -> String {
    // verify api_key
    const MAX_SIZE: usize = 262_144; // max payload size is 256k
    if req.headers().get("X-API-KEY").is_some() {
        if is_key_valid(
            req.headers()
                .get("X-API-KEY")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            data.lock().unwrap().api_key.lock().unwrap().to_vec(),
        ) {
            let mut body = BytesMut::new();
            while let Some(chunk) = payload.next().await {
                let chunk = chunk.unwrap();
                if (body.len() + chunk.len()) > MAX_SIZE {
                    return "request too large".to_string();
                }
                body.extend_from_slice(&chunk);
            }

            let mut rng = rand::thread_rng();
            let new_key: u64 = rng.gen(); // generates a new api-key
            data.lock()
                .as_mut()
                .unwrap()
                .api_key
                .lock()
                .as_mut()
                .unwrap()
                .push(new_key.to_string());
            add_api_key_to_file(new_key.to_string());
            let api_request = ApiKeyRequest {
                api_key: new_key.to_string(),
            };
            serde_json::to_string(&api_request).unwrap()
        } else {
            "invalid api key\n".to_string()
        }
    } else {
        "invalid api key\n".to_string()
    }
}

// curl -XPOST -H'X-API-KEY: 12790066417744034365' localhost:8202/api/delete/ -d'{"api_key":"9860738100897034443"}'
pub async fn delete_api_key(
    // key: web::Path<String>,
    mut payload: web::Payload,
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
            data.lock().unwrap().api_key.lock().unwrap().to_vec(),
        ) {
            const MAX_SIZE: usize = 262_144; // max payload size is 256k
            let mut body = BytesMut::new();
            while let Some(chunck) = payload.next().await {
                let chunk = chunck.unwrap();
                if (chunk.len() + body.len()) > MAX_SIZE {
                    return "request too large".to_string();
                }
                body.extend_from_slice(&chunk);
            }
            let key_request = serde_json::from_slice::<ApiKeyRequest>(&body).unwrap();

            remove_api_key_from_file(key_request.api_key);
            reload_state(&data.lock().unwrap().api_key, load_keys_from_file());
            "api key deleted".to_string()
        } else {
            "invalid api key\n".to_string()
        }
    } else {
        "invalid api key\n".to_string()
    }
}

fn reload_state(data: &Mutex<Vec<String>>, keys: Vec<String>) {
    data.lock().unwrap().clear();
    let mut new_vec: Vec<String> = vec![];
    for i in keys {
        new_vec.push(i)
    }

    for i in new_vec {
        data.lock().as_mut().unwrap().push(i)
    }

    // println!("{}", &data.lock().unwrap().last().unwrap());
}

fn add_api_key_to_file(new_key: String) {
    let new_key_formatted = format!("{}\n", new_key);
    let mut opt = OpenOptions::new()
        .write(true)
        .append(true)
        .open("config/api_keys")
        .unwrap();
    opt.write(new_key_formatted.as_bytes()).unwrap();
}

fn remove_api_key_from_file(del_key: String) {
    // load current keys
    let mut keys: Vec<String> = vec![];
    let file = fs::read("config/api_keys").unwrap();
    for line in file.lines() {
        keys.push(line.unwrap())
    }
    // remove key
    let mut rewrite_keys: Vec<String> = vec![];
    for (_i, u) in keys.iter().enumerate() {
        if !(*u == del_key) {
            rewrite_keys.push(u.to_string())
        }
    }

    // println!("{:#?}", rewrite_keys);

    let _ = fs::remove_file("config/api_keys");

    // write keys back to file
    let mut new_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .append(false)
        .open("config/api_keys")
        .unwrap();
    new_file.write("".as_bytes()).unwrap();

    for u in rewrite_keys {
        let formatted = format!("{}\n", u);
        new_file.write(formatted.as_bytes()).unwrap();
    }
}
