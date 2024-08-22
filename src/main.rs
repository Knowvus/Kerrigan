use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        env::var("PG_USER").expect("PG_USER must be set"),
        env::var("PG_PASSWORD").expect("PG_PASSWORD must be set"),
        env::var("PG_HOST").expect("PG_HOST must be set"),
        env::var("PG_PORT").expect("PG_PORT must be set"),
        env::var("PG_DATABASE").expect("PG_DATABASE must be set"),
    );
    println!("DATABASE_URL: {}", database_url);

    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    // Run migrations
    embedded_migrations::run(&connection).expect("Error running migrations");
}

// Embed migrations
embed_migrations!();