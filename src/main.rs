use tokio_postgres::{NoTls, Error};
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    // Load environment variables
    let db_user = env::var("PG_USER").expect("PG_USER must be set");
    let db_password = env::var("PG_PASSWORD").expect("PG_PASSWORD must be set");
    let db_name = env::var("PG_DATABASE").expect("PG_DATABASE must be set");
    let db_host = env::var("PG_HOST").unwrap_or("localhost".to_string());
    let db_port = env::var("PG_PORT").unwrap_or("5432".to_string());

    let connection_string = format!(
        "host={} port={} user={} password={} dbname={}",
        db_host, db_port, db_user, db_password, db_name
    );

    // Connect to the PostgreSQL database
    let (_client, connection) =
        tokio_postgres::connect(&connection_string, NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    println!("Successfully connected to PostgreSQL database!");

    // Keeping the server running to maintain the connection
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    }
}
