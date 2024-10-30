use crate::logger;
use crate::logger::Header;
use actix_web::web::Data;
use actix_web::HttpRequest;
use maydaylib::appstate::AppState;
use maydaylib::is_key_valid;
use std::sync::Mutex;

pub async fn root(data: Data<Mutex<AppState>>, req: HttpRequest) -> String {
    if is_key_valid(
        match req.headers().get("X-API-KEY") {
            Some(x) => x.to_str().unwrap().to_string(),
            None => "".to_string(),
        },
        data.clone()
            .lock()
            .unwrap()
            .api_keys
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
