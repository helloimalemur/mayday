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
    let sqlx_logging = env::var("SQLX_LOGGING")
        .expect("missing SQLX_LOGGING")
        .parse::<bool>()
        .expect("invalid sqlx logging value");
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
            .connect_timeout(Duration::from_secs(4))
            .acquire_timeout(Duration::from_secs(4))
            .idle_timeout(Duration::from_secs(4))
            .max_lifetime(Duration::from_secs(4))
            .sqlx_logging(sqlx_logging)
            .sqlx_logging_level(log::LevelFilter::Info);
        if let Ok(connection) = Database::connect(opt).await {
            dbcon = connection;
            connected = true;
        } else {
            // if first connection attempt fails check to see if we're running within a container
            // if in_container::in_container() { ;
            //
            // }
            pg_host = try_host(attempt, pg_host);
        }
        attempt += 1;
    }
    Ok(dbcon)
}

fn try_host(mut attempt: i32, pg_host: String) -> String {
    let mut out = pg_host.clone();
    if attempt > 1 {
        // Try Docker
        // Docker: The default subnet is 172.17.0.0/16.
        out = "172.17.0.1".to_string();
        println!("Trying Docker subnet: {}", out);
    }
    if attempt > 2 {
        // Try k3s
        out = "10.42.0.1".to_string();
        println!("Trying K3s subnet: {}", out);
    }
    if attempt > 3 {
        // Try openshift
        out = "10.128.0.1".to_string();
        println!("Trying Openshift subnet: {}", out);
        // OpenShift: The default Service CIDR is 172.30.0.0/16.
    }
    if attempt > 4 {
        // Try micro k8s
        // MicroK8s: The default pod CIDR is 10.1.0.0/16 and the default service CIDR is 10.152.183.0/24.
        out = "10.1.0.1".to_string();
        println!("Trying micro k8s subnet: {}", out);
    }
    if attempt > 5 {
        // Cilium: The default CIDR range is 10.0.0.0/8.
        out = "10.0.0.1".to_string();
        println!("Trying Cilium subnet: {}", out);
    }
    if attempt > 6 {
        attempt = 0;
    }

    out
}
