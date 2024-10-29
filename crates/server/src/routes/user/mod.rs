use std::sync::Mutex;
use actix_web::HttpRequest;
use actix_web::web::Data;
use maydaylib::appstate::AppState;
use maydaylib::is_key_valid;
use crate::logger;
use crate::logger::Header;

pub async fn create_user(data: Data<Mutex<AppState>>, req: HttpRequest) -> String {
    if is_key_valid(
        match req.headers().get("X-API-KEY") {
            Some(x) => x.to_str().unwrap().to_string(),
            None => "".to_string(),
        },
        data.clone()
            .lock()
            .unwrap()
            .api_key
            .lock()
            .unwrap()
            .to_vec(),
    ) {
        let message = format!("{} {} {:?}", req.method(), req.uri(), req.headers());
        logger::log(Header::SUCCESS, message.as_str());
        "Hello Astronaut!\n".to_string()
    } else {
        let message = format!(
            "{} {} {} {:?}",
            "invalid api key",
            req.method(),
            req.uri(),
            req.headers()
        );
        logger::log(Header::WARNING, message.as_str());
        "invalid api key\n".to_string()
    }
}