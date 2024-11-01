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
    let mut connected = false;
    dotenv::from_filename("docker/.env").ok();
    let pg_user = env::var("MARIADB_USER").expect("missing MARIADB_USER");
    let pg_pass = env::var("MARIADB_PASS").expect("missing MARIADB_PASS");
    let mut pg_host = env::var("MARIADB_HOST").expect("missing MARIADB_HOST");
    let pg_port = env::var("MARIADB_PORT").expect("missing MARIADB_PORT");
    let pg_db = env::var("MARIADB_DB").expect("missing MARIADB_DB");

    // for iface in datalink::interfaces() {
    //     for ip in iface.ips {
    //         if ip.is_ipv4() {
    //             println!("IPv4: {:?}", ip);
    //         }
    //     }
    // }

    let mut dbcon = DatabaseConnection::default();
    let mut attempt = 0;
    while !connected {
        let conn_string = format!(
            "mysql://{}:{}@{}:{}/{}",
            pg_user, pg_pass, pg_host, pg_port, pg_db
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
        if let Ok(connection) = Database::connect(opt).await {
            dbcon = connection;
            connected = true;
        } else {
            // if first connection attempt fails check to see if we're running within a container
            if in_container::in_container() {
                let cloned = pg_host.clone();
                pg_host = cloned.replace("127.0.0.1", "172.17.0.1");
            }
            pg_host = try_host(attempt, pg_host);
        }
        attempt += 1;
    }
    Ok(dbcon)
}

fn try_host(mut attempt: i32, pg_host: String) -> String {
    let mut out = pg_host.clone();
    if attempt > 1 {
        let cloned = pg_host.clone();
        out = cloned.replace("172.17.0.1", "172.18.0.1");
    }
    if attempt > 2 {
        let cloned = pg_host.clone();
        out = cloned.replace("172.18.0.1", "172.19.0.1");
    }
    if attempt > 3 {
        let cloned = pg_host.clone();
        out = cloned.replace("172.19.0.1", "172.20.0.1");
    }
    if attempt > 4 {
        let cloned = pg_host.clone();
        out = cloned.replace("172.20.0.1", "172.16.0.1");
    }
    if attempt > 5 {
        attempt = 0;
    }

    out
}
