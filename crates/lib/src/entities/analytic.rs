use crate::appstate::AppState;
use crate::is_key_valid;
use actix_web::error::ErrorBadRequest;
use actix_web::web::{Data, Payload};
use actix_web::{web, HttpRequest};
use anyhow::anyhow;
use futures_util::StreamExt;
use sqlx::{MySql, Pool};
use std::sync::Mutex;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Analytic {
    date_created: String,
    ip_address: String,
    landing: String,
    user_agent: String,
    web_session: String,
}

// CREATE TABLE `analytic` (
// `ip_address` INT NOT NULL,
// `user_agent` INT NOT NULL,
// `landing` INT NOT NULL,
// `web_session` VARCHAR(255) NOT NULL,
// `date_created` VARCHAR(255) NOT NULL,
// PRIMARY KEY (`web_session`)
// ) ENGINE=InnoDB;

pub async fn create_analytic_route(
    // name: web::Path<String>,
    mut payload: Payload,
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
) -> String {
    const MAX_SIZE: usize = 262_144; // max payload size is 256k
    let mut auth = false;
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
        if let Ok(analytic) = serde_json::from_slice::<Analytic>(&body) {
            if let Ok(_) = create_analytic(0, 0, analytic.clone(), data).await {
                println!(
                    "Analytic added :: {} :: {} :: {} :: {}",
                    analytic.ip_address,
                    analytic.landing,
                    analytic.user_agent.split_at(12).0.trim(),
                    analytic.web_session
                );
                "ok\n".to_string()
            } else {
                "query failed\n".to_string()
            }
        } else {
            "failure to deserialize\n".to_string()
        }
    } else {
        "invalid api key\n".to_string()
    }
}

#[allow(unused)]
pub async fn create_analytic(
    user_id: i64,
    tank_id: i64,
    analytic: Analytic,
    data: Data<Mutex<AppState>>,
) -> Result<(), anyhow::Error> {
    let mut app_state = data.lock();
    let db_pool = app_state.as_mut().unwrap().db_pool.lock().unwrap();
    if let Ok(_query_result) =
        sqlx::query("INSERT INTO analytic (date_created, ip_address, landing, user_agent, web_session) VALUES (?,?,?,?,?)")
            .bind(analytic.date_created)
            .bind(analytic.ip_address)
            .bind(analytic.landing)
            .bind(analytic.user_agent)
            .bind(analytic.web_session)
            .execute(db_pool.get_mysql_connection_pool())
            .await {
        Ok(())
    } else {
        Err(anyhow!("query failed"))
    }

    // println!("{:#?}", query_result);
}

#[allow(unused)]
pub async fn delete_analytic_route(
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
            data.lock().unwrap().api_key.lock().unwrap().to_vec(),
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
pub fn delete_analytic(user_id: i64, tank_id: i64, analytic: Analytic, db_pool: Pool<MySql>) {}

#[allow(unused)]
pub async fn modify_analytic_route(
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
            data.lock().unwrap().api_key.lock().unwrap().to_vec(),
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
pub fn modify_analytic(user_id: i64, tank_id: i64, analytic: Analytic, db_pool: Pool<MySql>) {}
