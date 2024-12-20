use crate::database::wait_for_db;
use crate::routes::root::root;
use crate::routes::session::session;
use crate::routes::user::user;
use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{web, App, HttpMessage, HttpServer, Responder};
use config::Config;
use maydaylib::appstate::AppState;
use maydaylib::load_keys_from_file;
use maydaylib::*;
use std::collections::HashMap;
use std::sync::Mutex;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use migration::{Migrator, MigratorTrait};

mod database;
mod logger;
mod routes;

#[tokio::main]
async fn main() {
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

    let db = wait_for_db().await;

    Migrator::up(&db, None).await.expect("unable to migrate database");

    let state = Data::new(Mutex::new(AppState::new(
        load_keys_from_file(),
        db.clone(),
        settings_map.clone(),
    )));

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    #[derive(OpenApi)]
    #[openapi(paths(routes::user::user, routes::session::session))]
    struct ApiDoc;
    let openapi = ApiDoc::openapi();
    // println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());

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
            // .service(web::resource("/register").post(register))
            // .service(web::resource("/register/").post(register))
            .service(web::resource("/user").post(user))
            .service(web::resource("/user/").post(user))
            .service(web::resource("/session").post(session))
            .service(web::resource("/session/").post(session))
            // .service(web::resource("/location").post(location))
            // .service(web::resource("/location/").post(location))
            // .service(web::resource("/alert").post(alert))
            // .service(web::resource("/alert/").post(alert))
            .service(web::resource("/").to(root))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
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
