use std::{fs, path::PathBuf};

use envy::get_env;
use serde::{Deserialize, Serialize};
use sqlx::{
    migrate::MigrateDatabase, sqlite::SqliteConnection, FromRow, Sqlite, SqlitePool, Transaction,
};
use tracing::{error, info};

#[derive(Debug)]
enum MigrationError {
    Failed,
}

#[derive(Clone, Debug, PartialEq, Eq, FromRow, Deserialize, Serialize, Default)]
struct Migration {
    id: u32,
    name: String,
    ran: bool,
    timestamp: String,
}

pub async fn run_migrations(folder_path: &str) {
    let db_url = get_env("DATABASE_URL");
    create_database(&db_url).await;
    // Open a connection to the SQLite database
    let mut transaction = match transaction().await {
        Ok(t) => t,
        Err(e) => {
            println!("Could not start transaction: {:?}", e);
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
            Ok(entry) => match execute_migration(entry.path(), &mut transaction).await {
                Ok(_) => {}
                Err(e) => {
                    println!("error executing migration: {:?}", e);
                }
            },
            Err(err) => {
                println!("error executing migration: {err}");
                return;
            }
        }
    }
}

async fn create_database(db_url: &str) {
    match Sqlite::create_database(db_url).await {
        Ok(_) => println!("database created"),
        Err(err) => {
            println!("error creating db:  {err}");
            return;
        }
    };
}

async fn execute_migration<'a>(
    file_path: PathBuf,
    transaction: &mut Transaction<'a, Sqlite>,
) -> Result<u64, MigrationError> {
    let query = match tokio::fs::read_to_string(file_path).await {
        Ok(sql) => sql,
        Err(err) => {
            println!("error reading files:  {err}");
            return Err(MigrationError::Failed);
        }
    };
    match sqlx::query(&query)
        .execute(transaction as &mut SqliteConnection)
        .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(err) => {
            error!("deleting: {:?}", err);
            Err(MigrationError::Failed)
        }
    }
}

async fn rows_to_vec<'a>(
    query: String,
    transaction: &mut Transaction<'a, Sqlite>,
) -> Result<Vec<Migration>, sqlx::Error> {
    let rows = sqlx::query_as::<_, Migration>(&query)
        .fetch_all(transaction as &mut SqliteConnection)
        .await;

    match rows {
        Ok(result) => Ok(result),
        Err(err) => {
            error!("rows_to_vec error findig: {:?}", err);
            Err(err)
        }
    }
}

async fn update_migrations_history<'a>(
    transaction: &mut Transaction<'a, Sqlite>,
    id: u32,
) -> Result<u64, sqlx::Error> {
    let query = format!(
        "DELETE FROM {table} WHERE id = {id}",
        table = "Self::table().await"
    );
    match sqlx::query(&query)
        .execute(transaction as &mut SqliteConnection)
        .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(err) => {
            error!("deleting: {:?}", err);
            Err(err)
        }
    }
}

async fn commit_transaction<'a>(transaction: Transaction<'a, Sqlite>) -> Result<(), sqlx::Error> {
    match transaction.commit().await {
        Ok(_) => {
            info!("transacttion commit succeeded");
            Ok(())
        }
        Err(err) => {
            println!("transacttion commit error: {:?}", err);
            Err(err)
        }
    }
}

async fn transaction<'a>() -> Result<Transaction<'a, Sqlite>, sqlx::Error> {
    match connect().await.begin().await {
        Ok(transaction) => Ok(transaction),
        Err(err) => {
            println!("transaction errror launching: {:?}", err);
            return Err(err);
        }
    }
}

async fn connect() -> SqlitePool {
    match SqlitePool::connect(&get_env("DATABASE_URL")).await {
        Ok(db) => db,
        Err(e) => panic!("{}", e),
    }
}
