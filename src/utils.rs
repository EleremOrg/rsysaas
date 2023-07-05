use sqlx::{sqlite::SqliteConnection, Connection, Executor};
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn read_env_file() {
    let file = File::open(".env").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            let trimmed_line = line.trim();
            let is_readable = !trimmed_line.is_empty() && !trimmed_line.starts_with('#');
            if is_readable {
                let parts: Vec<&str> = trimmed_line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();
                    env::set_var(key, value);
                }
            }
        }
    }
}

pub fn get_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| String::from(""))
}

pub fn tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init()
}

pub async fn execute_sql_file(file_path: &str) -> Result<(), sqlx::Error> {
    // Open a connection to the SQLite database
    let mut connection = SqliteConnection::connect(&get_env("DATABASE_URL")).await?;

    // Read the SQL file contents
    let sql = tokio::fs::read_to_string(file_path).await?;

    // Execute the SQL statements
    connection.execute(&*sql).await?;

    Ok(())
}
