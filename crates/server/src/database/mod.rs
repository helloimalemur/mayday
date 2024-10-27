use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::env;
use std::time::Duration;

pub async fn wait_for_db() -> DatabaseConnection {
    loop {
        if let Ok(db) = db_connect().await {
            return db;
        } else {
            println!("Failed to connect to server .. retrying..");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}

pub async fn db_connect() -> Result<DatabaseConnection, DbErr> {
    dotenv::from_filename("docker/.env").ok();
    let pg_user = env::var("MARIADB_USER").expect("missing MARIADB_USER");
    let pg_pass = env::var("MARIADB_PASS").expect("missing MARIADB_PASS");
    let pg_host = env::var("MARIADB_HOST").expect("missing MARIADB_HOST");
    let pg_db = env::var("MARIADB_DB").expect("missing MARIADB_DB");
    // let conn_string = format!("mysql://{}:{}@{}/{}", pg_user, pg_pass, pg_host, pg_db);
    let conn_string = format!(
        "mysql://{}:{}@{}:{}/{}",
        pg_user, pg_pass, pg_host, "3306", pg_db
    );

    println!("Connecting to mysql database at {}", conn_string);
    let mut opt = ConnectOptions::new(conn_string.as_str());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
    // .set_schema_search_path("my_schema"); // Setting default PostgreSQL schema

    Database::connect(opt).await
}
