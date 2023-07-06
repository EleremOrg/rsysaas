use envy::get_env;
use sqlx::{migrate::MigrateDatabase, sqlite::SqliteConnection, Connection, Executor, Sqlite};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
    let db_url = get_env("DATABASE_URL");
    Sqlite::create_database(&db_url).await?;
    // Open a connection to the SQLite database
    let mut connection = SqliteConnection::connect(&db_url).await?;

    // Read the SQL file contents
    let sql = tokio::fs::read_to_string(file_path).await?;

    // Execute the SQL statements
    connection.execute(&*sql).await?;

    Ok(())
}
