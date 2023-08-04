use std::{
    fs::{read_dir, DirEntry},
    path::PathBuf,
};

use envy::{get_bool_env, get_env};
use sqlx::{
    migrate::MigrateDatabase, sqlite::SqliteConnection, FromRow, Sqlite, SqlitePool, Transaction,
};
use tracing::{error, info};

use super::Orm;

#[derive(Debug)]
enum MigrationError {
    Failed,
}

#[derive(FromRow, Debug)]
struct Migration {
    id: u32,
    name: String,
    path: String,
    ran: bool,
    timestamp: String,
}

#[derive(Debug)]
struct MigrationFile {
    name: String,
    ran: bool,
    path: PathBuf,
}

impl MigrationFile {
    fn new(entry: DirEntry) -> Self {
        Self {
            name: entry.file_name().to_string_lossy().to_string(),
            ran: false,
            path: entry.path(),
        }
    }
}

pub async fn migrate(folder_path: &str) {
    let db_url = get_env("DATABASE_URL");
    create_database(&db_url).await;

    let mut transaction = match transaction().await {
        Ok(t) => t,
        Err(e) => {
            println!("Could not start transaction: {:?}", e);
            return;
        }
    };
    let _ = create_migrations_table(&mut transaction)
        .await
        .map_err(|e| {
            println!("Could not create_migrations_table: {:?}", e);
            return;
        });
    let migrations_history = match get_migrations_history(&mut transaction).await {
        Ok(m) => m,
        Err(e) => {
            println!("Could not get migrations history: {:?}", e);
            return;
        }
    };

    let migrations_files = match get_migrations_file(folder_path).await {
        Ok(m) => m,
        Err(e) => {
            println!("Could not get migrations files: {:?}", e);
            return;
        }
    };

    println!("Found {:?} migrations files", migrations_files);
    println!("Found {:?} migrations history", migrations_history);
    // maybe just loop over all the files migrations, save them into the database if they don0t exists.
    // then query the database to get the list of migrations and execute them.
    match migrations_history.is_empty() {
        true => run_inital_migrations(migrations_files, &mut transaction).await,
        false => run_migrations(migrations_files, migrations_history, &mut transaction).await,
    }
    match commit_transaction(transaction).await {
        Ok(_) => println!("Migration completed"),
        Err(e) => println!("Could not commit transaction: {:?}", e),
    };
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

async fn create_migrations_table<'a>(
    transaction: &mut Transaction<'a, Sqlite>,
) -> Result<u64, sqlx::Error> {
    let query = r#"
        CREATE TABLE IF NOT EXISTS migrations (
            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            ran BOOLEAN NOT NULL,
            timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    "#;
    let result = sqlx::query(query)
        .execute(transaction as &mut SqliteConnection)
        .await?;
    Ok(result.rows_affected())
}

async fn get_migrations_history<'a>(
    transaction: &mut Transaction<'a, Sqlite>,
) -> Result<Vec<Migration>, sqlx::Error> {
    let query = Orm::select("*").from("migrations").ready();
    let rows = sqlx::query_as::<_, Migration>(&query)
        .fetch_all(transaction as &mut SqliteConnection)
        .await;

    match rows {
        Ok(result) => Ok(result),
        Err(err) => {
            println!("get_migrations_history error findig: {:?}", err);
            Err(err)
        }
    }
}

async fn get_migrations_file(folder_path: &str) -> Result<Vec<MigrationFile>, std::io::Error> {
    let entries = match read_dir(folder_path) {
        Ok(result) => result,
        Err(err) => {
            println!("error reading dir: {err}");
            return Err(err);
        }
    };

    Ok(entries
        .into_iter()
        .map(|f| MigrationFile::new(f.ok().unwrap()))
        .collect())
}

async fn run_migrations<'a>(
    migrations_files: Vec<MigrationFile>,
    migrations_history: Vec<Migration>,
    transaction: &mut Transaction<'a, Sqlite>,
) {
    for mut migration_file in migrations_files {
        if migration_file.ran {
            continue;
        }

        match execute_migration(&migration_file.path, transaction).await {
            Ok(_) => {
                migration_file.ran = true;
                save_migration_to_history(&migration_file, transaction).await;
            }
            Err(e) => {
                println!("Could not run migration: {:?}", e);
                return;
            }
        }
    }
}

async fn run_inital_migrations<'a>(
    migrations_files: Vec<MigrationFile>,
    transaction: &mut Transaction<'a, Sqlite>,
) {
    for mut migration_file in migrations_files {
        if migration_file.ran || skip_test_migration(&migration_file.name).await {
            continue;
        }

        match execute_migration(&migration_file.path, transaction).await {
            Ok(_) => {
                migration_file.ran = true;
                match save_migration_to_history(&migration_file, transaction).await {
                    Ok(_) => {}
                    Err(e) => {
                        // revert commit, etc...
                        println!("Could not save migration to history: {:?}", e);
                        return;
                    }
                };
            }
            Err(e) => {
                println!("Could not run migration: {:?}", e);
                return;
            }
        }
    }
}

async fn skip_test_migration(migration_name: &str) -> bool {
    get_bool_env("RUN_TEST_MIGRATIONS") && migration_name.contains("test")
}

async fn execute_migration<'a>(
    file_path: &PathBuf,
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
            println!("deleting: {:?}", err);
            Err(MigrationError::Failed)
        }
    }
}

async fn save_migration_to_history<'a>(
    migration_file: &MigrationFile,
    transaction: &mut Transaction<'a, Sqlite>,
) -> Result<u64, sqlx::Error> {
    let query = Orm::insert("migrations")
        .set_columns("name,path,ran")
        .add_value(&format!(
            "'{}','{}',{}",
            migration_file.name,
            migration_file.path.display(),
            migration_file.ran
        ))
        .ready();
    match sqlx::query(&query)
        .execute(transaction as &mut SqliteConnection)
        .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(err) => {
            println!("deleting: {:?}", err);
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
