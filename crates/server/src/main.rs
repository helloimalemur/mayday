use crate::database::wait_for_db;
use crate::routes::root::root;
use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{web, App, HttpMessage, HttpServer, Responder};
use config::Config;
use maydaylib::appstate::AppState;
use maydaylib::load_keys_from_file;
use maydaylib::user::{create_user_route, delete_user_route};
use maydaylib::*;
use pnet::datalink;
use routes::root;
use std::collections::HashMap;
use std::sync::Mutex;

mod database;
mod logger;
mod routes;

#[tokio::main]
async fn main() {
    for iface in datalink::interfaces() {
        println!("{:?}", iface.ips);
    }

    let settings = Config::builder()
        .add_source(config::File::with_name("config/Settings"))
        .build()
        .expect("could not load Settings.toml");
    let settings_map = settings
        .try_deserialize::<HashMap<String, String>>()
        .expect("unable to deserialize settings");

    let http_service_port = settings_map
        .get("http_service_port")
        .expect("could not get http_service_port from settings")
        .parse::<u16>()
        .unwrap();

    let backend_host = settings_map
        .get("backend_host")
        .expect("could not get backend_host from settings")
        .parse::<String>()
        .unwrap();

    // let mut database_url = settings_map
    //     .get("database_url")
    //     .expect("could not get database_url from settings")
    //     .parse::<String>()
    //     .unwrap();

    // if in_container::in_container() {
    //     let cloned = database_url.clone();
    //     database_url = cloned.replace("127.0.0.1", "172.17.0.1");
    // }

    let db = wait_for_db().await;

    let state = Data::new(Mutex::new(AppState::new(
        load_keys_from_file(),
        db.clone(),
        settings_map.clone(),
    )));

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let _server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            // .allowed_origin(base_address.as_str())
            // .allowed_origin("http://127.0.0.1:8202")
            // .allowed_origin(backend_host.as_str())
            // .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(state.clone())
            // .wrap(api_key::ApiKey::new("".to_string()))
            .service(web::resource("/register").post(create_user_route))
            .service(web::resource("/user/create").post(create_user_route))
            .service(web::resource("/user/create/").post(create_user_route))
            .service(web::resource("/user/delete").post(delete_user_route))
            .service(web::resource("/user/delete/").post(delete_user_route))
            .service(web::resource("/").to(root))
        // .default_service(web::to(default_handler))
    })
    .bind(("0.0.0.0", http_service_port))
    .expect("PORT/ADDRESS IN USE")
    .run()
    .await;
}

// async fn default_handler(req_method: Method) -> Result<impl Responder, Error> {
//     match req_method {
//         Method::GET => {
//             let file = NamedFile::open("../koontsnet-frontend/static/404.html")?
//                 .customize()
//                 .with_status(StatusCode::NOT_FOUND);
//             Ok(Either::Left(file))
//         }
//         _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
//     }
// }
