use std::{fs, path::PathBuf};

use envy::get_env;
use sqlx::{migrate::MigrateDatabase, sqlite::SqliteConnection, Connection, Executor, Sqlite};

pub async fn run_migrations(folder_path: &str) {
    let db_url = get_env("DATABASE_URL");
    create_database(&db_url).await;
    // Open a connection to the SQLite database
    let mut connection = match SqliteConnection::connect(&db_url).await {
        Ok(connection) => connection,
        Err(err) => {
            println!("{err}");
            return;
        }
    };
    let entries = match fs::read_dir(folder_path) {
        Ok(result) => result,
        Err(err) => {
            println!("error reading dir: {err}");
            return;
        }
    };
    for entry in entries {
        match entry {
            Ok(entry) => execute_migration(&mut connection, entry.path()).await,
            Err(err) => {
                println!("error executing migration: {err}");
                return;
            }
        }
    }
}

async fn execute_migration<T>(connection: &mut T, file_path: PathBuf) {
    let query = match tokio::fs::read_to_string(file_path).await {
        Ok(sql) => sql,
        Err(err) => {
            println!("error reading files:  {err}");
            return;
        }
    };
    match connection.execute(&*query).await {
        Ok(result) => println!("succesfull result: {:?}", result),
        Err(err) => {
            println!("error executing sql: {err}");
            return;
        }
    };
}
